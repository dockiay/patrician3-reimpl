#!/usr/bin/env python3
"""Paquete de idioma para p3-juego: cadenas del EXE de otro idioma alineadas con las del EXE inglés.

El código elevado del cliente solo vale para el EXE inglés; cada EXE lleva sus cadenas de interfaz dentro (el CPR solo trae scripts,
misiones, nombres...). Este script empareja cada cadena del EXE inglés con la del EXE del otro idioma y escribe `idioma/<código>.pack`,
que el cliente carga para traducir los textos en las rutinas de cadenas y para copiar las pocas tablas de maquetación que cambian.

    python scripts/gen_language_pack.py EXE_INGLES EXE_OTRO CODIGO SALIDA.pack     (necesita pefile y capstone)

Método (probado con el EXE español 1.0 y el inglés 1.1):
  1. cadenas de .rdata/.data (NUL-terminadas, texto Latin-1);
  2. anclas: pares que el código referencia en el mismo punto (barrido lineal con capstone: mismo mnemónico y mismos tres previos),
     más cadenas idénticas únicas; subsecuencia creciente más larga;
  3. entre anclas, alineación Needleman-Wunsch con una similitud que exige los mismos marcadores (%s, %d, \\c, \\f, saltos de línea).
Solo se aceptan pares con los mismos marcadores. El resultado depende del EXE inglés exacto (el paquete guarda su SHA-1). No va a Git.
"""
import bisect, collections, difflib, functools, hashlib, math, re, sys, unicodedata
import capstone, pefile

PRINT = re.compile(rb'[\x20-\x7e\xa0-\xff\r\n\t]+')
LETTERS = re.compile(rb'[A-Za-z]{2}|%')

# Tablas de maquetación que cambian de sitio entre ediciones: (idioma, dirección en el EXE inglés, dirección en el otro, bytes)
LAYOUT_TABLES = {'spa': [(0x6970D8, 0x696978, 64)]}   # posiciones del resaltado del menú principal


def texty(x):
    # descarta bytes de datos que parecen texto (punteros, tablas): exige letras y espacios en su mayoría
    good = sum(1 for c in x if chr(c).isalpha() or c in b' .,:;!?\'"-()%\\_/\n0123456789')
    if len(x) < 2 or good / len(x) < 0.85 or len(re.findall(rb'[A-Za-z]', x)) < 2:
        return False
    # una palabra suelta necesita vocal y tres letras; los bytes de datos casi nunca las tienen
    if re.search(rb'%[-0-9.]*[sdiufcxl]', x):
        return True
    if len(x) < 6 and b' ' not in x:
        return re.fullmatch(rb'[A-Z]?[a-z\xdf-\xff]{2,}', x) is not None
    return b' ' in x or (len(x) >= 3 and re.search(rb'[aeiouyAEIOU\xe0-\xfc]', x) is not None and re.search(rb'[a-z]', x) is not None)


def identifier(x):
    # claves, rutas y nombres de archivo: no se traducen aunque cambien de una edición a otra
    if b' ' in x or x.startswith((b'\\c', b'\\f', b'\\l')):
        return False
    return re.search(rb'[\\/_:]|\.[A-Za-z]{2,4}$|^[A-Z0-9]{3,}$|^[a-z]+[A-Z]|[0-9]$', x) is not None


def strings(path):
    pe = pefile.PE(path)
    out = []
    for s in pe.sections:
        if s.Name.rstrip(b'\0').decode() not in ('.rdata', '.data'):
            continue
        data, base, pos = s.get_data(), 0x400000 + s.VirtualAddress, 0
        for chunk in data.split(b'\0'):
            if len(chunk) >= 2 and PRINT.fullmatch(chunk) and LETTERS.search(chunk) and texty(chunk):
                out.append((base + pos, chunk))
            pos += len(chunk) + 1
    return out


def code_refs(path, known):
    pe = pefile.PE(path)
    text = pe.sections[0]
    code, base = text.get_data(), 0x400000 + text.VirtualAddress
    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_32)
    md.detail = True
    refs, hist, off = [], [], 0
    while off < len(code):
        progressed = False
        for ins in md.disasm(code[off:], base + off):
            progressed = True
            off = ins.address - base + ins.size
            hist.append(ins.mnemonic)
            ref = None
            for size, at in ((ins.disp_size, ins.disp_offset), (ins.imm_size, ins.imm_offset)):
                if size == 4 and at:
                    v = int.from_bytes(ins.bytes[at:at + 4], 'little')
                    if v in known:
                        ref = v
                        break
            if ref is not None:
                refs.append((ref, ins.mnemonic, tuple(hist[-4:-1])))
        if not progressed:
            off += 1
    return refs


def markers(x):
    return re.findall(rb'%[a-z]', x)


def sim(a, b):
    if a == b:
        return 4
    if markers(a) != markers(b):
        return -3
    sc = 2
    sc += 1 if (a.count(b'\\c'), a.count(b'\\f'), a[:1] == b'\\') == (b.count(b'\\c'), b.count(b'\\f'), b[:1] == b'\\') else -2
    r = (len(b) + 3) / (len(a) + 3)
    sc += 1 if 0.6 <= r <= 2.2 else -1
    tok = lambda x: set(re.findall(rb'%[a-z]|[0-9]+|\\[a-z][0-9]?|\n|[A-Z][A-Za-z]{3,}', x))
    return sc + min(2, len(tok(a) & tok(b)))


def lis(pairs):
    tails, prev = [], [-1] * len(pairs)
    for k, (_, j) in enumerate(pairs):
        p = bisect.bisect_left([pairs[t][1] for t in tails], j)
        if p:
            prev[k] = tails[p - 1]
        if p == len(tails):
            tails.append(k)
        else:
            tails[p] = k
    out, k = [], tails[-1] if tails else -1
    while k >= 0:
        out.append(pairs[k])
        k = prev[k]
    return out[::-1]


@functools.lru_cache(maxsize=None)
def sim2(a, b):
    """Similitud inglés/otro idioma de dos cadenas: marcas iguales, longitud comparable (~15 % más larga), mismo comienzo (\\c, \\f) y
    parecido de caracteres (cognados). Va de unos -5 a +8; sobre 4 es casi siempre una traducción."""
    if a == b:
        return 4.0
    if markers(a) != markers(b):
        return -4.0
    sc = 1 if (a[:1] == b'\\') == (b[:1] == b'\\') else -2
    sc += 1 if a.count(b'\n') == b.count(b'\n') else -1
    sc += 2 - 2 * abs(math.log((len(b) + 3) / (len(a) + 3) / 1.15))
    return sc + 3 * difflib.SequenceMatcher(None, fold(a), fold(b)).ratio()


def accept(a, b, sc, lax=False):
    if lax:
        return sc >= (3.0 if len(a) <= 7 else 2.0)
    return sc >= (3.9 if len(a) <= 7 else 3.0)


TOK = re.compile(r'[a-zñü]+')
SHAPE = lambda x: (x[:1] == b'\\', x.count(b'\n') > 0, bool(re.search(rb'\\[cfld]', x)))


def toks(x):
    return [t for t in TOK.findall(fold(x)) if len(t) >= 2]


def dictionary(pairs):
    """Diccionario inglés -> otro idioma por coocurrencia (Dice) en los pares ya emparejados con confianza."""
    ce, cs, cc = collections.Counter(), collections.Counter(), collections.Counter()
    for a, b in pairs:
        ta, tb = set(toks(a)), set(toks(b))
        if not ta or not tb or len(ta) > 25:
            continue
        ce.update(ta)
        cs.update(tb)
        for x in ta:
            for y in tb:
                cc[(x, y)] += 1
    d = collections.defaultdict(dict)
    for (x, y), c in cc.items():
        w = 2 * c / (ce[x] + cs[y])
        if w >= 0.25 and (c >= 2 or w >= 0.9):
            d[x][y] = w
    return d


def fscore(ta, tb, d):
    if not ta or not tb:
        return 0.0
    tot = 0.0
    for x in ta:
        best = 0.0
        for y in tb:
            w = d.get(x, {}).get(y, 0.0)
            if x == y and len(x) >= 3:
                w = max(w, 0.9)
            elif len(x) >= 4 and len(y) >= 4 and w < 0.8:
                r = difflib.SequenceMatcher(None, x, y).ratio()
                if r >= 0.75:
                    w = max(w, r * 0.9)
            best = max(best, w)
        tot += best
    return 2 * tot / (len(ta) + len(tb))


def refine(e, s, base, rounds=2):
    """Reasigna por contenido, sin depender del orden: cada cadena inglesa se empareja con la del otro idioma que mejor la traduce según
    el diccionario aprendido de los pares fiables, con las mismas marcas, forma y longitud comparable, si es la mejor en ambos sentidos."""
    es = {a: t for a, t in e}
    ss = {a: t for a, t in s}
    cur = dict(base)                       # dirección inglesa -> dirección del otro
    for _ in range(rounds):
        d = dictionary([(es[a], ss[b]) for a, b in cur.items() if sim2(es[a], ss[b]) >= 5 or fscore(toks(es[a]), toks(ss[b]), {}) >= 0.8])
        index = collections.defaultdict(set)
        for b, t in s:
            for y in set(toks(t)):
                index[y].add(b)
        inv = collections.defaultdict(set)
        for x, ys in d.items():
            for y in ys:
                inv[x].add(y)
        best, bestb = {}, {}
        for a, t in e:
            if identifier(t) or not texty(t):
                continue
            ta, ma = toks(t), markers(t)
            cands = set()
            for x in set(ta):
                for y in inv.get(x, ()):
                    cands |= index.get(y, set())
                if len(x) >= 3:
                    cands |= index.get(x, set())
            scored = []
            for b in cands:
                u = ss[b]
                if markers(u) != ma or SHAPE(u) != SHAPE(t):
                    continue
                r = (len(u) + 3) / (len(t) + 3)
                if not 0.55 <= r <= 2.6:
                    continue
                scored.append((fscore(ta, toks(u), d), b))
            scored.sort(reverse=True)
            if scored and scored[0][0] >= 0.5 and (len(scored) == 1 or scored[0][0] - scored[1][0] >= 0.08):
                best[a] = scored[0]
                if scored[0][0] > bestb.get(scored[0][1], (0, 0))[0]:
                    bestb[scored[0][1]] = (scored[0][0], a)
        new = {}
        for a, (sc, b) in best.items():
            if bestb[b][1] == a:
                new[a] = b
        # Los pares del emparejado por orden se conservan salvo que el contenido diga otra cosa: el de contenido gana si la puntuación
        # del actual es baja (< 0,3); si dos cadenas inglesas reclaman la misma, vale la de contenido.
        taken = set(new.values())
        for a, b in cur.items():
            if a in new:
                continue
            new[a] = b
        cur = new
    return cur


def nw(A, B, gap=-1.5, lax=False):
    """Needleman-Wunsch por orden (las cadenas del mismo módulo van en el mismo orden en ambos EXE); banda según la diferencia de tamaño."""
    n, m, NEG, S = len(A), len(B), -1e9, {}
    band = max(40, abs(m - n) + 20)
    g = lambda i, j: S.get((i, j), NEG)
    off = m - n
    for i in range(n + 1):
        for j in range(max(0, min(i, i + off) - band), min(m, max(i, i + off) + band) + 1):
            if i == 0:
                S[(i, j)] = j * gap
            elif j == 0:
                S[(i, j)] = i * gap
            else:
                S[(i, j)] = max(g(i - 1, j - 1) + sim2(A[i - 1][1], B[j - 1][1]), g(i - 1, j) + gap, g(i, j - 1) + gap)
    i, j, out = n, m, []
    while i > 0 and j > 0:
        sc = sim2(A[i - 1][1], B[j - 1][1])
        if S[(i, j)] == g(i - 1, j - 1) + sc:
            if accept(A[i - 1][1], B[j - 1][1], sc, lax):
                out.append((i - 1, j - 1))
            i, j = i - 1, j - 1
        elif S[(i, j)] == g(i - 1, j) + gap:
            i -= 1
        else:
            j -= 1
    return out[::-1]


def nw_old(A, B, gap=-1.0, band=120):
    n, m, NEG, S = len(A), len(B), -1e9, {}
    g = lambda i, j: S.get((i, j), NEG)
    off = m - n
    for i in range(n + 1):
        for j in range(max(0, min(i, i + off) - band), min(m, max(i, i + off) + band) + 1):
            if i == 0:
                S[(i, j)] = j * gap
            elif j == 0:
                S[(i, j)] = i * gap
            else:
                S[(i, j)] = max(g(i - 1, j - 1) + sim(A[i - 1][1], B[j - 1][1]), g(i - 1, j) + gap, g(i, j - 1) + gap)
    i, j, out = n, m, []
    while i > 0 and j > 0:
        d = g(i - 1, j - 1) + sim(A[i - 1][1], B[j - 1][1])
        if S[(i, j)] == d:
            if sim(A[i - 1][1], B[j - 1][1]) >= 3:
                out.append((i - 1, j - 1))
            i, j = i - 1, j - 1
        elif S[(i, j)] == g(i - 1, j) + gap:
            i -= 1
        else:
            j -= 1
    return out[::-1]


def align(eng, oth, legacy=False):
    e, s = strings(eng), strings(oth)
    ke, ks = dict(e), dict(s)
    pe_, ps = {a: i for i, (a, _) in enumerate(e)}, {a: i for i, (a, _) in enumerate(s)}
    # anclas por código
    ev, sv = code_refs(eng, ke), code_refs(oth, ks)
    sm = difflib.SequenceMatcher(None, [(m, c) for _, m, c in ev], [(m, c) for _, m, c in sv], autojunk=False)
    votes = collections.defaultdict(collections.Counter)
    for tag, i0, i1, j0, j1 in sm.get_opcodes():
        if tag == 'equal':
            for k in range(i1 - i0):
                votes[ev[i0 + k][0]][sv[j0 + k][0]] += 1
    pairs = {}
    for a, c in votes.items():
        b = c.most_common(1)[0][0]
        if sim(ke[a], ks[b]) >= 2 and a in pe_ and b in ps:
            pairs[pe_[a]] = ps[b]
    ce, cs = collections.Counter(x[1] for x in e), collections.Counter(x[1] for x in s)
    pos_s = {x[1]: j for j, x in enumerate(s) if cs[x[1]] == 1}
    for i, x in enumerate(e):
        if ce[x[1]] == 1 and x[1] in pos_s and len(x[1]) >= 4:
            pairs.setdefault(i, pos_s[x[1]])
    anchors = lis(sorted(pairs.items()))
    res = dict(anchors)
    loose = set()
    bounds = [(-1, -1)] + anchors + [(len(e), len(s))]
    for (i0, j0), (i1, j1) in zip(bounds, bounds[1:]):
        A, B = e[i0 + 1:i1], s[j0 + 1:j1]
        if not A or not B:
            continue
        if legacy:
            if len(A) == len(B) and all(sim(a[1], b[1]) >= 2 for a, b in zip(A, B)):
                res.update({i0 + 1 + k: j0 + 1 + k for k in range(len(A))})
            else:
                for a, b in nw_old(A, B):
                    res[i0 + 1 + a] = j0 + 1 + b
            continue
        for a, b in nw(A, B):
            res[i0 + 1 + a] = j0 + 1 + b
    if legacy:
        return [(e[i][0], e[i][1], s[j][1], False, False, s[j][0]) for i, j in sorted(res.items()) if e[i][1] != s[j][1]]
    # Segunda pasada: entre dos pares vecinos ya emparejados, si sobran tantas cadenas de un lado como del otro van por orden
    # (no pueden desplazarse: están acotadas por ambos lados) con tal de que lleven las mismas marcas.
    ordered = sorted(res.items())
    for (i0, j0), (i1, j1) in zip([(-1, -1)] + ordered, ordered + [(len(e), len(s))]):
        A, B = e[i0 + 1:i1], s[j0 + 1:j1]
        if A and B and len(A) != len(B) and len(A) <= 40 and len(B) <= 40:
            for a, b in nw(A, B, lax=True):
                res[i0 + 1 + a] = j0 + 1 + b
                loose.add(i0 + 1 + a)
        if A and len(A) == len(B) and all(markers(a[1]) == markers(b[1]) and sim2(a[1], b[1]) > -1 for a, b in zip(A, B)):
            for k in range(len(A)):
                res[i0 + 1 + k] = j0 + 1 + k
                loose.add(i0 + 1 + k)
    strong = {i for i, _ in anchors}
    return [(e[i][0], e[i][1], s[j][1], i in strong, i in loose, s[j][0]) for i, j in sorted(res.items()) if e[i][1] != s[j][1]]


def fold(x):
    return unicodedata.normalize('NFD', x.decode('latin1').lower()).encode('ascii', 'ignore').decode()


def trustworthy(eng_s, spa_s, strong):
    """Un par sin ancla de código solo vale si las dos cadenas se parecen (cognados) o son largas y con las mismas marcas: en las
    tablas de palabras cortas la alineación por orden se desplaza y daba traducciones equivocadas («Yes» -> «Aprox.»)."""
    r = difflib.SequenceMatcher(None, fold(eng_s), fold(spa_s)).ratio()
    if strong:
        return len(eng_s) > 12 or r >= 0.3 or len(eng_s) <= 3
    if len(eng_s) <= 12:
        return r >= 0.5
    return r >= 0.3 or (markers(eng_s) and markers(eng_s) == markers(spa_s))


# Tablas cortas donde no hay ancla ni cognado: (cadenas inglesas consecutivas, traducción). Se buscan como carrera consecutiva en el EXE inglés.
MANUAL_RUNS = [
    ([b'Yes'], [b'S\xed']),
    ([b'Jan', b'Feb', b'Mar', b'Apr', b'May', b'Jun', b'Jul', b'Aug', b'Sep', b'Oct', b'Nov', b'Dec'],
     [b'ene.', b'feb.', b'mar.', b'abr.', b'may.', b'jun.', b'jul.', b'ago.', b'sep.', b'oct.', b'nov.', b'dic.']),
    ([b'January', b'February', b'March', b'April', b'May', b'June', b'July', b'August', b'September', b'October', b'November', b'December'],
     [b'enero', b'febrero', b'marzo', b'abril', b'mayo', b'junio', b'julio', b'agosto', b'septiembre', b'octubre', b'noviembre', b'diciembre']),
]


# Textos que el EXE español arma de otra forma y no tienen par directo: se traducen a mano si quedan sin emparejar.
MANUAL_TEXT = {
    b'to': b'al',
    b'Prognose': b'Previsi\xf3n',
    b'Age': b'Edad',
    b'1 year': b'1 a\xf1o',
    b'Ready': b'Listo',
    b'Join': b'Unirse',
    b"Ship's name": b'Nombre del barco',
    b'Goods Supply': b'Suministro de mercanc\xedas',
    b'Age:': b'Edad:',
    b'Social status in this town: %s': b'Estatus social en esta ciudad: %s',
    b'%s is represented with a trading office in ': b'%s tiene una oficina comercial en ',
    b'You have to employ a trading office administator in order to make automatic purchases possible.':
        b'Debes contratar a un administrador en la oficina comercial para poder realizar compras autom\xe1ticas.',
}


# Por dirección del EXE inglés (artículos del prestamista: «The patrono X solicita» no tiene artículo en el EXE español).
MANUAL_ADDR = {0x69B8C0: b'El', 0x69B8C4: b'El', 0x6A08B0: b'Compras autom\xe1ticas'}


# Siempre (aunque haya par): el emparejado desplazaba estas expresiones sueltas.
FORCE_TEXT = {b'the convoy': b'el convoy', b'Sum': b'Suma'}


# Por dirección y siempre: filas del balance de la oficina, que el EXE español ordena distinto.
FORCE_ADDR = {0x6A0898: b'Costes salariales', 0x6A08A4: b'Impuesto b\xe1sico', 0x6A08B0: b'Compras autom\xe1ticas', 0x6A08E4: b'Varios',
              # Ficha de viviendas: el inglés arma «Half-timbered » + «house(s)»; el español usa el nombre entero («Casas de madera») y deja
              # el sufijo vacío (tabla 0x6A2440 inglesa ↔ 0x6A23D8 española). Sin esto salía «Casas de ladrillohouse».
              0x6A22BC: b'', 0x6A22C4: b''}

# Frases que el EXE español arma con otros argumentos (otro orden o tipo: «%d de %s» frente a «%d-%d») y que no se pueden copiar: traducción
# propia con las mismas marcas y en el mismo orden que el EXE inglés. Por dirección en el EXE inglés.
MANUAL_SENTENCES = {
    0x69C460: r'\f1_La nave de %i toneladas %s "%s" se vendió por %i\C a %s %s %s.',
    0x69D668: r'Tienes de plazo hasta el ',
    0x69E078: r'\f1_Se celebrará una sesión extraordinaria del Consejo de la Liga Hanseática el %d-%d-%d en %s. El tema a debatir es un embargo comercial contra %s, porque ',
    # Carta «Misión aceptada» (taberna): el EXE español pone la fecha delante («Reúnete antes del %u de %s de %u con...»); sobre los
    # argumentos del EXE inglés (persona ×2, taberna, ciudad, día, mes, año, cantidad, unidad, mercancía) va con posicionales
    # `%$N$`, que el formateador del cliente (`ui_rt/crt.rs`) admite como el propio del juego (`0x64D410`).
    0x69E748: r'\f1_Reúnete antes del %$5$u de %$6$s de %$7$u con %$1$s %$2$s en la taberna %$3$s de %$4$s, para entregar %$8$d%$9$s de %$10$s.',
    0x6A08C8: 'Administrador (Experiencia %d)',
    0x6A1384: 'Dirigir manualmente',
    0x6A4CA0: 'el %i de %s de %i en %s',
    0x6C1F88: '%i oficinas comerciales',
    0x69EC78: r'\f1_La alta demanda de %s no está cubierta en nuestra ciudad. Por ello ofrecemos al mercader que pueda suministrarnos antes del %d-%d con %d%s un precio fijo de %d\C (equivale a %d\C por \%c).',
    0x6A1340: '\\c\\f3_Batalla naval\n\n\\l\\f1_%s "%s" está siendo atacado por %s "%s" %s.\n',
    0x6A2568: '\\f1_\n\n¿Debe el convoy %s zarpar en la mañana del %i-%i-%i con destino a %s?',
    0x6A4580: '\n\\c\\f0_Fallo de red\n\n\\l\\f1_Se ha producido un error de sincronización.\nSe recomienda detener la partida actual e iniciar una nueva partida en red cargando la partida guardada "network". Esta partida se creó automáticamente al producirse el error.',
    0x6A4814: '¿Debe realmente el jugador',
    0x6A8B10: '\\f1_Muchas gracias por ofrecerte a devolver la prosperidad a la ciudad de %s.\nAbastece a la ciudad y construye negocios y viviendas para que aumente la población.\nTu objetivo es aumentar su población hasta al menos %$5$d personas antes del %$2$d. %$3$s %$4$d.',
    0x6A8F7C: '\\f1_%s envió piratas el %d. %s',
    0x6A8FA8: '\\f1_%s saqueó las arcas de la Liga Hanseática el %d. %s',
    0x6AC8AC: 'La petición de bloqueo fue rechazada por %u votos contra %u.',
    0x6AC8E8: 'La decisión de bloquear la ciudad de %s fue aprobada con %u votos. Hubo %u votos en contra de esta moción.',
    0x6AC960: 'La petición de boicot a la ciudad de %s fue rechazada por %u votos contra %u.',
    0x6AC9B0: 'La moción de boicot a la ciudad de %s fue aprobada con %u votos. Se registraron %u votos en contra de esta moción.',
    0x6ACA20: 'La petición de imponer una multa elevada a %s fue rechazada por %u votos contra %u.',
    0x6ACA68: 'Se decidió imponer una multa elevada a la ciudad de %s por %u votos. En total se registraron %u votos en contra de esta idea.',
    0x6ACAE8: 'La petición de imponer una multa moderada a %s fue rechazada por %u votos contra %u.',
    0x6ACB38: 'Se decidió imponer una multa moderada a la ciudad de %s con %u votos. Hubo %u votos en contra de esta multa.',
    0x6ACBB0: 'La petición de imponer una multa pequeña a %s fue rechazada por %u votos contra %u.',
    0x6AD0F0: '\n\\l\nSe ha convocado un Consejo extraordinario de la Liga en %s el %d. %s %d',
    0x6AE8B0: 'Hoy has adquirido un edificio (%$2$s) en la subasta del gremio de %$1$s por la suma de %d00\\C comprado a %s %s.',
    0x6B1538: 'Un mercader de %s ha prometido %s %d\\%c %s antes del %d.%d.%d a un precio de %d\\C.\n ',
    0x6B31E8: '%s %s %s %s ha recibido el nuevo título de %s en su ciudad natal, %s',
    0x6B3230: '%s %s %s %s ha recibido el nuevo título de %s en su ciudad natal, %s',
    0x6B39F8: 'Un préstamo de %i\\C que contrataste el %u-%u-%u vence en %s',
    0x6B4A30: '\\dMuchas gracias por encargarte de este pequeño transporte. Nos vemos a más tardar el %u-%u-%u con %s %s en la %s de %s. Entrega allí %u\\B de %s y recibirás el pago acordado.',
    0x6B4AF0: '\\dMuchas gracias por encargarte de este pequeño transporte. Nos vemos a más tardar el %u-%u-%u con %s %s en la %s de %s. Entrega allí %u\\L de %s y recibirás el pago acordado.',
    0x6B59C0: ' Recibirás un dividendo de %u\\C por tus %u participaciones en esta empresa.',
    0x6B63C0: '%s %s se hundió en la expedición a los Mares del Sur el %u. %s %u.\nLamentablemente, todos los registros del timonel se hundieron con el barco.',
    0x6B6448: '\\c\\f0_Hambruna en %s\n\\l\n\\f1_Las reservas de trigo se agotaron hace tiempo, la gente pasa hambre y espera con ansia una pronta llegada de alimentos.',
    0x6B6A60: 'Los ejércitos enemigos han roto las defensas de %s tras %u días de asedio continuo y han saqueado el banco de la ciudad. Se capturaron %i\\C y se destruyeron %i edificios.',
    0x6B6BA0: 'Los ejércitos enemigos han roto las defensas de %s tras %u días de asedio continuo y han saqueado el banco de la ciudad. Se capturaron %i\\C.',
    0x6B6F54: 'impuesto de exportación sobre mercancías',
    0x6B6F70: 'impuesto especial para los talleres hanseáticos',
    0x6B6F9C: 'enviado bandas de piratas contra barcos hanseáticos',
    0x6B6FD8: '\nDía del Consejo de la Liga Hanseática en %s.\n\\l\n\\dLa ciudad de %s, que %s, fue objeto de debate hoy en %s.\n',
    0x6B704C: 'Una moción de boicot de %s fue rechazada por %u votos contra %u.',
    0x6B7090: 'Se acordó el boicot a la ciudad de %s por %u votos. Hubo %u votos en contra del boicot.',
    0x6C27E8: 'Hay una gran cantidad de %s disponible en %s en este momento y, por tanto, a un precio bastante bueno.',
    0x6C2848: 'Hay una gran cantidad de %s disponible en %s en este momento y, por tanto, a un precio bastante bueno.',
    0x6C2928: '%i litros de cerveza se beben cada día en %s, ¡qué barbaridad! Viejos borrachos.',
    0x6C3458: 'Se está celebrando una reunión del consejo municipal en %s.',
    0x6C3490: '%s ha desobedecido las órdenes de la Liga Hanseática.',
}


def manual(eng):
    e = strings(eng)
    out = {}
    for en, es in MANUAL_RUNS:
        for i in range(len(e) - len(en) + 1):
            if [x[1] for x in e[i:i + len(en)]] == en:
                for k in range(len(en)):
                    out[e[i + k][0]] = es[k]
    return out


def pointer_arrays(path):
    """Tablas de punteros a cadenas de .data/.rdata (listas que el código indexa: edificios, mercancías, meses...): series de dwords
    consecutivos que apuntan al comienzo de una cadena. El índice es el mismo en todas las ediciones, así que alinear dos tablas
    por posición da pares exactos."""
    pe = pefile.PE(path)
    img = pe.get_memory_mapped_image()
    base = 0x400000
    data = [(s.VirtualAddress, s.Misc_VirtualSize) for s in pe.sections if s.Name.rstrip(b'\0').decode() in ('.data', '.rdata')]
    lo, hi = min(v for v, _ in data), max(v + z for v, z in data)

    def at(v):
        o = v - base
        if not (lo <= o < hi) or o < 1 or img[o - 1] != 0:
            return None
        e = img.find(b'\0', o)
        if e < 0 or not 2 <= e - o <= 400:
            return None
        t = img[o:e]
        return t if PRINT.fullmatch(t) and re.search(rb'[A-Za-z]{2}', t) else None

    out, cur = [], []
    for va, sz in data:
        for o in range(va, va + sz - 3, 4):
            v = int.from_bytes(img[o:o + 4], 'little')
            t = at(v) if base <= v < base + hi else None
            if t is not None:
                cur.append((v, t))
            else:
                if len(cur) >= 2:
                    out.append(cur)
                cur = []
    if len(cur) >= 2:
        out.append(cur)
    return out


def array_pairs(eng, oth):
    """Pares de tablas de punteros emparejadas (misma longitud, textos que se parecen): lista de (tabla inglesa, tabla del otro idioma)."""
    A, B = pointer_arrays(eng), pointer_arrays(oth)

    def score(x, y):
        if len(x) != len(y):
            return None
        ok = [sim2(a[1], b[1]) for a, b in zip(x, y) if markers(a[1]) == markers(b[1])]
        if len(ok) < 0.7 * len(x):
            return None
        mean = sum(ok) / len(x)
        return (mean - 0.8) * len(x) if mean >= 0.8 else None

    n, m, NEG = len(A), len(B), -1e9
    S = {}
    band = max(30, abs(m - n) + 10)
    g = lambda i, j: S.get((i, j), NEG)
    for i in range(n + 1):
        for j in range(max(0, i - band), min(m, i + band + (m - n if m > n else 0)) + 1):
            if i == 0:
                S[(i, j)] = -j
            elif j == 0:
                S[(i, j)] = -i
            else:
                sc = score(A[i - 1], B[j - 1])
                S[(i, j)] = max(g(i - 1, j - 1) + sc if sc is not None else NEG, g(i - 1, j) - 1, g(i, j - 1) - 1)
    i, j, out = n, m, []
    while i > 0 and j > 0:
        sc = score(A[i - 1], B[j - 1])
        if sc is not None and S[(i, j)] == g(i - 1, j - 1) + sc:
            out.append((A[i - 1], B[j - 1]))
            i, j = i - 1, j - 1
        elif S[(i, j)] == g(i - 1, j) - 1:
            i -= 1
        else:
            j -= 1
    return out


def main():
    if len(sys.argv) != 5:
        sys.exit(__doc__)
    eng, oth, code, out = sys.argv[1:]
    kept = [p for p in align(eng, oth) if texty(p[2]) and not identifier(p[1]) and (trustworthy(p[1], p[2], True) if p[3] else (sim2(p[1], p[2]) > 0 or p[4]))]
    es, ss = strings(eng), strings(oth)
    final = refine(es, ss, {p[0]: p[5] for p in kept})
    sd, ed = dict(ss), dict(es)
    # El emparejado por orden clásico (`legacy`) acierta donde el nuevo se desplaza en tablas de palabras sueltas, y al revés en módulos
    # con otro orden. Donde coinciden, vale; donde discrepan, gana el que mejor puntúa con el diccionario de los pares que coinciden.
    legacy = {p[0]: p[5] for p in align(eng, oth, legacy=True) if texty(p[2]) and sim(p[1], p[2]) >= 2 and not identifier(p[1])}
    agreed = [(ed[a], sd[b]) for a, b in final.items() if legacy.get(a) == b]
    d = dictionary(agreed)
    cons = collections.defaultdict(set)
    for t, u in agreed:
        cons[t].add(u)
    for a, b in legacy.items():
        c = final.get(a)
        if c is None:
            final[a] = b
        elif c != b:
            sc_old = sim2(ed[a], sd[b]) + 4 * fscore(toks(ed[a]), toks(sd[b]), d) + (3 if sd[b] in cons[ed[a]] else 0)
            sc_new = sim2(ed[a], sd[c]) + 4 * fscore(toks(ed[a]), toks(sd[c]), d) + (3 if sd[c] in cons[ed[a]] else 0)
            if sc_old > sc_new:
                final[a] = b
    # Tablas de punteros: pares exactos por posición; mandan sobre el resto.
    arr = {}
    for X, Y in array_pairs(eng, oth):
        # Por posición, salvo que el otro EXE ordene distinto las filas (balance de la oficina): entonces gana el contenido.
        F = [[fscore(toks(a[1]), toks(b[1]), d) for b in Y] for a in X]
        n = len(X)
        choice = list(range(n))
        moved = any(
            j != i and F[i][j] >= 0.3 and F[i][j] - F[i][i] >= 0.25 and max(range(n), key=lambda k: F[k][j]) == i
            for i in range(n) for j in [max(range(n), key=lambda k: F[i][k])]
        )
        if moved:
            # Reordenación dentro de la tabla: asignación voraz por contenido y el resto, por orden.
            choice, free_j = [None] * n, set(range(n))
            for f, i, j in sorted(((F[i][j], i, j) for i in range(n) for j in range(n)), reverse=True):
                if f >= 0.3 and choice[i] is None and j in free_j:
                    choice[i] = j
                    free_j.discard(j)
            rest = sorted(free_j)
            for i in range(n):
                if choice[i] is None:
                    choice[i] = rest.pop(0)
        if moved:
            gain = sum(F[i][choice[i]] - F[i][i] for i in range(n))
            if gain < 0.5 or sum(choice[i] != i for i in range(n)) > 6:
                choice = list(range(n))   # demasiado cambio: no es una fila reordenada sino ruido del diccionario; vale la posición
        for i, a in enumerate(X):
            pick = Y[choice[i]]
            if markers(a[1]) == markers(pick[1]):
                arr.setdefault(a[0], pick[0])
    final.update({a: b for a, b in arr.items() if a in ed and b in sd and texty(sd[b]) and not identifier(ed[a])})
    # Las cadenas de la biblioteca C/MFC (antes de 0x690000: nombres de mes de strftime, errores del CRT) no son interfaz del juego.
    pairs = [(a, b'', sd[b]) for a, b in sorted(final.items()) if a >= 0x690000 and sd[b] != ed[a] and texty(sd[b])]
    over = manual(eng)
    have = {p[0] for p in pairs}
    over.update(FORCE_ADDR)
    over.update({a: t.encode('latin1') for a, t in MANUAL_SENTENCES.items() if a not in have})
    over.update({a: FORCE_TEXT[t] for a, t in es if t in FORCE_TEXT})
    over.update({a: t for a, t in MANUAL_ADDR.items() if a not in have})
    over.update({a: MANUAL_TEXT[t] for a, t in es if t in MANUAL_TEXT and a not in have})
    pairs = [p for p in pairs if p[0] not in over] + [(a, b'', t) for a, t in over.items()]
    dropped = 0
    image = pefile.PE(eng).get_memory_mapped_image()
    other = pefile.PE(oth).get_memory_mapped_image()
    import os
    os.makedirs(os.path.dirname(os.path.abspath(out)), exist_ok=True)
    with open(out, 'w') as f:
        f.write(f'H {hashlib.sha1(open(eng, "rb").read()).hexdigest()} {code}\n')
        for a, _, t in pairs:
            f.write(f'S {a:x} {t.hex()}\n')
        if code == 'spa':
            f.write(f'X 0 {bytes([0xf1, 0xd1, 0xbf, 0xa1]).hex()}\n')   # ñ Ñ ¿ ¡ (celdas en el CPR derivado)
        for ea, oa, n in LAYOUT_TABLES.get(code, []):
            f.write(f'P {ea:x} {other[oa - 0x400000:oa - 0x400000 + n].hex()}\n')
    print(f'{len(pairs)} cadenas distintas emparejadas -> {out}')


if __name__ == '__main__':
    main()

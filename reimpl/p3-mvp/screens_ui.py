"""Ventanas del juego (prestamista, taberna, ...) del cliente Tk.

Cada ventana pide al núcleo `screen` (lo que muestra el original) y envía `screen_action` al pulsar un botón; la simulación valida y
aplica la misma operación que encola el original. Aquí solo hay presentación."""
import random
import unicodedata
import tkinter as tk
from tkinter import ttk


def money(n):
    return f"{n:,}".replace(",", ".")


def scrolled(parent, width=None):
    """Marco con barras de desplazamiento: los controles que no caben en la ventana se alcanzan desplazando (WIN-02).

    Devuelve `(contenedor, interior)`: se empaqueta el contenedor y se ponen los controles en el interior."""
    outer = ttk.Frame(parent)
    canvas = tk.Canvas(outer, highlightthickness=0, borderwidth=0)
    if width:
        canvas.configure(width=width)
    sy = ttk.Scrollbar(outer, orient="vertical", command=canvas.yview)
    sx = ttk.Scrollbar(outer, orient="horizontal", command=canvas.xview)
    canvas.configure(yscrollcommand=sy.set, xscrollcommand=sx.set)
    canvas.grid(row=0, column=0, sticky="nsew")
    sy.grid(row=0, column=1, sticky="ns")
    sx.grid(row=1, column=0, sticky="ew")
    outer.rowconfigure(0, weight=1)
    outer.columnconfigure(0, weight=1)
    inner = ttk.Frame(canvas)
    item = canvas.create_window(0, 0, window=inner, anchor="nw")

    def resize(_event=None):
        canvas.configure(scrollregion=(0, 0, inner.winfo_reqwidth(), inner.winfo_reqheight()))
        # El interior ocupa al menos el ancho visible, para que `fill="x"` siga funcionando.
        canvas.itemconfigure(item, width=max(inner.winfo_reqwidth(), canvas.winfo_width()))

    inner.bind("<Configure>", resize)
    canvas.bind("<Configure>", resize)

    def wheel(event):
        if inner.winfo_reqheight() > canvas.winfo_height():
            step = -1 if (getattr(event, "delta", 0) > 0 or getattr(event, "num", 0) == 4) else 1
            canvas.yview_scroll(step, "units")

    def bind_wheel(_event):
        canvas.bind_all("<MouseWheel>", wheel)
        canvas.bind_all("<Button-4>", wheel)
        canvas.bind_all("<Button-5>", wheel)

    def unbind_wheel(_event):
        canvas.unbind_all("<MouseWheel>")
        canvas.unbind_all("<Button-4>")
        canvas.unbind_all("<Button-5>")

    outer.bind("<Enter>", bind_wheel)
    outer.bind("<Leave>", unbind_wheel)
    outer.canvas = canvas
    return outer, inner


class ScreenWindow:
    """Base: ventana hija que se refresca con el JSON de `screen` y que reenvía acciones."""
    name = ""
    title = ""
    size = "620x460"

    def __init__(self, client):
        self.client = client
        self.window = tk.Toplevel(client.root)
        self.window.title(self.title)
        self.window.geometry(self.size)
        self.head = tk.StringVar()
        ttk.Label(self.window, textvariable=self.head, padding=6, font=("Segoe UI", 11, "bold")).pack(anchor="w")
        self.status = tk.StringVar()
        ttk.Label(self.window, textvariable=self.status, padding=6, wraplength=580).pack(side="bottom", anchor="w")
        self.area, self.frame = scrolled(self.window)
        self.area.pack(fill="both", expand=True)
        self.frame.bind("<Configure>", self.fit, add="+")
        self.build()
        self.refresh()

    def fit(self, _event=None):
        """Agranda la ventana hasta lo que piden sus controles, sin pasar del 90 % de la pantalla; el resto queda en las barras."""
        if not self.alive():
            return
        self.window.update_idletasks()
        need_w = self.frame.winfo_reqwidth() + 24
        need_h = self.frame.winfo_reqheight() + self.window.winfo_reqheight() - self.area.winfo_reqheight() + 24
        max_w = int(self.window.winfo_screenwidth() * 0.9)
        max_h = int(self.window.winfo_screenheight() * 0.9)
        w, h = self.window.winfo_width(), self.window.winfo_height()
        new_w, new_h = max(w, min(need_w, max_w)), max(h, min(need_h, max_h))
        if (new_w, new_h) != (w, h) and w > 1:
            self.window.geometry(f"{new_w}x{new_h}")

    def alive(self):
        return self.window.winfo_exists()

    def refresh(self):
        self.client.request({"command": "screen", "name": self.name}, self.shown)

    def shown(self, screen):
        self.show(screen)
        if self.alive():
            self.window.after(50, self.fit)

    def act(self, action, **args):
        self.client.request({"command": "screen_action", "name": self.name, "action": action, "args": args}, self.done)

    def done(self, result):
        if "state" in result:
            self.client.update_state(result["state"])
            self.client.dirty = True
        screen = result.get("screen", result)
        self.shown(screen)

    def build(self):
        raise NotImplementedError

    def show(self, screen):
        raise NotImplementedError


class LenderWindow(ScreenWindow):
    """Prestamista (`MoneyLenderDialog`): tomar un préstamo, devolverlo, conceder uno a un solicitante y ver los concedidos."""
    name = "lender"
    title = "Prestamista"

    def build(self):
        tabs = ttk.Notebook(self.frame)
        tabs.pack(fill="both", expand=True, padx=6, pady=4)
        self.lists = {}
        self.buttons = {}
        for key, label, button in [("offers", "Tomar un préstamo", "Tomar"), ("own", "Devolver", "Devolver"),
                                   ("applicants", "Conceder", "Conceder"), ("granted", "Concedidos", None)]:
            frame = ttk.Frame(tabs, padding=6)
            tabs.add(frame, text=label)
            box = tk.Listbox(frame, height=10, exportselection=False)
            box.pack(fill="both", expand=True)
            self.lists[key] = box
            if button:
                row = ttk.Frame(frame)
                row.pack(fill="x", pady=4)
                if key == "applicants":
                    ttk.Label(row, text="Interés (0 = bajo … 4 = alto)").pack(side="left")
                    self.interest = tk.IntVar(value=2)
                    ttk.Spinbox(row, from_=0, to=4, textvariable=self.interest, width=3).pack(side="left", padx=4)
                b = ttk.Button(row, text=button, command=lambda k=key: self.press(k))
                b.pack(side="left", padx=4)
                self.buttons[key] = b
        self.screen = None

    def selected(self, key):
        sel = self.lists[key].curselection()
        return sel[0] if sel else None

    def press(self, key):
        i = self.selected(key)
        if i is None or not self.screen:
            self.status.set("Selecciona una fila.")
            return
        if key == "offers":
            self.act("take", index=i)
        elif key == "own":
            self.act("repay", task=self.screen["own_loans"][i]["task"])
        elif key == "applicants":
            self.act("grant", index=i, interest=int(self.interest.get()))

    def show(self, screen):
        if not self.alive():
            return
        self.screen = screen
        self.head.set(f"{screen['town_name']} · oro {money(screen['gold'])}")
        name = self.client.city_name
        for box in self.lists.values():
            box.delete(0, "end")
        status = screen["offers_status"]
        if status == -1:
            self.lists["offers"].insert("end", "No se te ofrece ningún préstamo.")
        elif status == -2 and not screen["offers"]:
            self.lists["offers"].insert("end", "Tu capacidad de crédito es insuficiente.")
        elif not screen["can_take"]:
            self.lists["offers"].insert("end", "Ya tienes tres préstamos: no se te ofrecen más.")
        else:
            for o in screen["offers"]:
                self.lists["offers"].insert("end", f"{money(o['amount'])} a {o['weeks']} semanas · devolver {money(o['repay'])}")
        self.buttons["offers"].configure(state="normal" if screen["can_take"] and screen["offers"] else "disabled")
        for t in screen["own_loans"]:
            self.lists["own"].insert("end", f"{name(t['town'])}: devolver {money(t['repay'])} · {t['weeks']} semanas")
        if not screen["own_loans"]:
            self.lists["own"].insert("end", "No tienes préstamos.")
        for a in screen["applicants"]:
            self.lists["applicants"].insert("end", f"{money(a['amount'])} a {a['weeks']} semanas · devolverían {money(a['repay'])} (interés {a['interest']})")
        if not screen["applicants"]:
            self.lists["applicants"].insert("end", "Nadie en la ciudad quiere un préstamo.")
        for t in screen["granted_loans"]:
            self.lists["granted"].insert("end", f"{name(t['town'])}: {money(t['repay'])} a devolver")
        if not screen["granted_loans"]:
            self.lists["granted"].insert("end", "No has concedido ningún préstamo.")


class ChurchWindow(ScreenWindow):
    """Iglesia (`ChurchContentWindow`): donar a la redecoración (0x32) y a la ampliación (0x31). «Alimentar a los pobres» usa la operación 0x30 (0x004FE557)."""
    name = "church"
    title = "Iglesia"
    size = "560x460"

    def build(self):
        self.info = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.info, padding=6, wraplength=480, justify="left").pack(anchor="w")
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        ttk.Label(row, text="Importe").pack(side="left")
        self.amount = tk.IntVar(value=500)
        ttk.Spinbox(row, from_=0, to=999999, increment=500, textvariable=self.amount, width=9).pack(side="left", padx=4)
        self.redecorate = ttk.Button(row, text="Redecorar la iglesia", command=lambda: self.act("redecorate", amount=self.value()))
        self.redecorate.pack(side="left", padx=4)
        self.extend = ttk.Button(row, text="Ampliar la iglesia", command=lambda: self.act("extend", amount=self.value()))
        self.extend.pack(side="left", padx=4)

        ttk.Label(self.frame, text="Alimentar a los pobres (lotes por mercancía)", padding=(6, 8, 6, 0)).pack(anchor="w")
        grid = ttk.Frame(self.frame, padding=6)
        grid.pack(fill="x")
        self.feed = [tk.IntVar(value=0) for _ in range(5)]
        self.feed_left = [tk.StringVar() for _ in range(5)]
        for k, name in enumerate(["Cereales", "Cerveza", "Pescado", "Carne", "Vino"]):
            ttk.Label(grid, text=name).grid(row=0, column=k * 2, padx=4)
            ttk.Spinbox(grid, from_=0, to=65535, textvariable=self.feed[k], width=5).grid(row=1, column=k * 2, padx=4)
            ttk.Label(grid, textvariable=self.feed_left[k]).grid(row=2, column=k * 2, padx=4)
        ttk.Button(self.frame, text="Entregar a los pobres", command=self.give).pack(anchor="w", padx=10)

    def give(self):
        values = []
        for v in self.feed:
            try:
                values.append(max(0, min(65535, int(v.get()))))
            except (tk.TclError, ValueError):
                values.append(0)
        self.act("feed_poor", amounts=values)

    def value(self):
        try:
            return max(0, int(self.amount.get()))
        except (tk.TclError, ValueError):
            return 0

    STAGES = ["Faltan muchas donaciones para ampliar.", "Ya se ha recibido mucho.", "Se ha reunido casi todo.",
              "La caja está llena: esperando las obras.", "Las obras de ampliación están en marcha.", "La iglesia no puede ampliarse más."]

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · oro {money(screen['gold'])}")
        self.info.set(f"Nivel de la iglesia {screen['level']} · nivel de donaciones {screen['tier']}\n"
                      f"Caja de ampliación: {money(screen['collected'])}\n{self.STAGES[screen['stage']]}")
        self.extend.configure(state="normal" if screen["stage"] < 3 else "disabled")
        for k, lots in enumerate(screen["feed"]["lots"]):
            self.feed_left[k].set(f"hay {lots}")


class TownHallWindow(ScreenWindow):
    """Ayuntamiento (`TownHallWindow`): candidatura a la alcaldía (0x45), votación del día (0x3F/0x49) y arcas de la ciudad (ingresar/retirar, operación 0x47)."""
    name = "town_hall"
    title = "Ayuntamiento"
    size = "520x420"

    def build(self):
        self.info = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.info, padding=6, wraplength=480, justify="left").pack(anchor="w")
        self.candidate = tk.BooleanVar(value=False)
        ttk.Checkbutton(self.frame, text="Presentarme a la alcaldía", variable=self.candidate,
                        command=lambda: self.act("candidacy", candidate=int(self.candidate.get()))).pack(anchor="w", padx=8)
        coffers = ttk.Frame(self.frame, padding=6)
        coffers.pack(fill="x")
        self.amount = tk.IntVar(value=100)
        ttk.Label(coffers, text="Arcas, importe:").pack(side="left")
        ttk.Spinbox(coffers, from_=1, to=100000000, textvariable=self.amount, width=10).pack(side="left", padx=4)
        self.coffer_buttons = [ttk.Button(coffers, text="Ingresar", command=lambda: self.act("treasury", deposit=1, amount=int(self.amount.get()))),
                               ttk.Button(coffers, text="Retirar", command=lambda: self.act("treasury", deposit=0, amount=int(self.amount.get())))]
        for b in self.coffer_buttons:
            b.pack(side="left", padx=2)
        self.coffer_note = tk.StringVar()
        ttk.Label(coffers, textvariable=self.coffer_note).pack(side="left", padx=6)
        self.vote_row = ttk.Frame(self.frame, padding=6)
        self.vote_row.pack(fill="x")
        self.vote_text = tk.StringVar()
        ttk.Label(self.vote_row, textvariable=self.vote_text).pack(anchor="w")
        buttons = ttk.Frame(self.vote_row)
        buttons.pack(anchor="w")
        self.vote_buttons = []
        for c in range(3):
            b = ttk.Button(buttons, text=f"Opción {c + 1}", command=lambda c=c: self.act("vote", council=int(self.council), choice=c))
            b.pack(side="left", padx=4)
            self.vote_buttons.append(b)
        self.council = False

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · oro {money(screen['gold'])}")
        mayor = screen.get("mayor_name") or ("vacante" if screen["mayor"] == 255 else f"comerciante {screen['mayor']}")
        # Como en el original: las arcas (Town coffers) solo se activan para el alcalde.
        for b in self.coffer_buttons:
            b.configure(state="normal" if screen.get("is_mayor") else "disabled")
        self.coffer_note.set("" if screen.get("is_mayor") else "(solo el alcalde)")
        self.info.set(f"Alcalde: {mayor}\nArcas de la ciudad: {money(screen['town_money'])}\n"
                      f"Mes anterior: {', '.join(money(v) for v in screen['transactions'])}")
        self.candidate.set(screen["candidate"])
        vote = screen["vote"]
        self.vote_text.set("Hoy hay votación." if vote else "No hay votación hoy.")
        for b in self.vote_buttons:
            b.configure(state="normal" if vote else "disabled")
        self.council = bool(vote and vote["kind"] == 0x20 and False)


class TavernWindow(ScreenWindow):
    """Taberna (`TavernSideMenu`): conversaciones disponibles hoy. Las transacciones de cada página (contratar, comprar noticias...) llegan por etapas."""
    name = "tavern"
    title = "Taberna"
    size = "460x520"
    PAGE_LABELS = ["Contrato de suministro", "Hablar con el comerciante", "Contratar marineros", "Contratar capitán", "Noticias",
                   "Hablar con el viajero", "Traficante de armas", "Hablar con el ladrón", "Hablar con el pirata", "Cartas de oferta"]

    def build(self):
        self.rows = []
        for label in self.PAGE_LABELS:
            row = ttk.Label(self.frame, text=label, padding=4)
            row.pack(anchor="w", padx=8)
            self.rows.append(row)

        self.crew = ttk.Frame(self.frame, padding=6)
        self.crew.pack(fill="x")

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · oro {money(screen['gold'])}")
        for w in self.crew.winfo_children():
            w.destroy()
        ttk.Label(self.crew, text=f"Marineros disponibles: {screen['hireable_sailors']}").pack(anchor="w")
        for sh in screen["ships"]:
            row = ttk.Frame(self.crew)
            row.pack(fill="x")
            ttk.Label(row, text=f"Barco {sh['ship']}: tripulación {sh['crew']}, caben {sh['room']}").pack(side="left")
            for cap in screen.get("captains", []):
                ttk.Button(row, text=f"Capitán {cap['captain']}", command=lambda sh=sh, cap=cap: self.act("hire_captain", captain=cap["captain"], ship=sh["ship"])).pack(side="left", padx=2)
            if sh["room"] > 0 and screen["hireable_sailors"] > 0:
                ttk.Button(row, text="Completar", command=lambda sh=sh: self.act("hire_sailors", ship=sh["ship"], amount=sh["room"])).pack(side="left", padx=4)
        for c in screen.get("contracts", []):
            row = ttk.Frame(self.crew)
            row.pack(fill="x")
            ttk.Label(row, text=f"Contrato de suministro: {WARE_NAMES[c['ware']]}, faltan {c['remaining']} a {c['price']} cada una ({c['deliverable']} entregables)").pack(side="left")
            if c["deliverable"] > 0:
                ttk.Button(row, text="Entregar", command=lambda c=c: self.act("deliver", task=c["task"], amount=c["deliverable"])).pack(side="left", padx=6)
        tr = screen.get("trader")
        if tr:
            ttk.Label(self.crew, text=f"Comerciante: {tr['lots']} lotes de {WARE_NAMES[tr['ware']]} por {money(tr['price'])}. Cargar en…").pack(anchor="w")
            row = ttk.Frame(self.crew)
            row.pack(fill="x")
            for sh in screen["ships"]:
                ttk.Button(row, text=f"Barco {sh['ship']}", command=lambda sh=sh: self.act("buy_ware", ship=sh["ship"])).pack(side="left", padx=2)
            ttk.Button(row, text="Oficina", command=lambda: self.act("buy_ware")).pack(side="left", padx=2)
        weapons = screen.get("weapons")
        if weapons:
            # Destino y cantidad se eligen una vez; cada clase tiene solo Comprar/Vender (antes 18 botones por fila desbordaban, WIN-02).
            ttk.Label(self.crew, text="Armero: compra o venta de armas para un barco o la oficina").pack(anchor="w")
            row = ttk.Frame(self.crew)
            row.pack(fill="x")
            targets = ["Oficina"] + [f"Barco {sh['ship']}" for sh in screen["ships"]]
            ships = [None] + [sh["ship"] for sh in screen["ships"]]
            dest = ttk.Combobox(row, values=targets, state="readonly", width=12)
            dest.current(0)
            qty = ttk.Combobox(row, values=["1", "5", "50"], state="readonly", width=4)
            qty.current(0)
            ttk.Label(row, text="Destino").pack(side="left")
            dest.pack(side="left", padx=4)
            ttk.Label(row, text="Cantidad").pack(side="left")
            qty.pack(side="left", padx=4)
            for wp in weapons:
                row = ttk.Frame(self.crew)
                row.pack(fill="x")
                ttk.Label(row, text=f"Clase {wp['class']}: {wp['stock10']} decenas; compra {wp['buy'][0]}, venta {wp['sell'][0]} por arma").pack(side="left")
                for buy, tag in ((True, "Comprar"), (False, "Vender")):
                    ttk.Button(row, text=tag, command=lambda wp=wp, buy=buy, dest=dest, qty=qty: self.act(
                        "weapons", **{"class": wp["class"], "quantity": int(qty.get()), "buy": int(buy), "ship": ships[max(dest.current(), 0)]})).pack(side="left", padx=2)
        pirate = screen.get("pirate")
        if pirate:
            ttk.Label(self.crew, text="Capitán pirata: se lleva un barco tuyo a la banda pirata más cercana (casco sano, sin capitán)").pack(anchor="w")
            row = ttk.Frame(self.crew)
            row.pack(fill="x")
            if not pirate["ships"]:
                ttk.Label(row, text="Ningún barco tuyo puede ir con él ahora").pack(side="left")
            for ship in pirate["ships"]:
                ttk.Button(row, text=f"Entregar barco {ship}", command=lambda ship=ship: self.act("hire_pirate", ship=ship, crew=random.randint(10, 20))).pack(side="left", padx=2)
        trav = screen.get("traveller")
        if trav and not trav["late"]:
            ttk.Label(self.crew, text=f"Viajero a {trav['dest_name']}: paga {money(trav['price'])}. Llevarlo con…").pack(anchor="w")
            for sh in screen["ships"]:
                ttk.Button(self.crew, text=f"Barco {sh['ship']}", command=lambda sh=sh: self.act("passage", ship=sh["ship"])).pack(anchor="w")
        elif trav:
            ttk.Label(self.crew, text=f"Viajero a {trav['dest_name']}: ya no queda margen").pack(anchor="w")
        if screen.get("news"):
            ttk.Button(self.crew, text=f"Comprar noticias de la región ({money(screen['news']['cost'])})", command=lambda: self.act("buy_news")).pack(anchor="w")
        if screen["pages"][7]["available"]:
            ttk.Label(self.crew, text="Ladrón: encargar un robo a…").pack(anchor="w")
            for r in screen.get("rivals", []):
                ttk.Button(self.crew, text=f"Comerciante {r['merchant']} ({money(r['price'])})", command=lambda r=r: self.act("burglary", victim=r["merchant"])).pack(anchor="w")
        for row, page in zip(self.rows, screen["pages"]):
            row.configure(text=("● " if page["available"] else "○ ") + self.PAGE_LABELS[page["page"]], foreground="" if page["available"] else "gray")


class HanseWindow(ScreenWindow):
    """Hansa: propuestas del día hanseático pendientes (op 0x4A): aceptar (fecha dentro de ocho días) o rechazar (código 6)."""
    name = "hanse"
    title = "Hansa"
    size = "460x300"

    def build(self):
        self.box = ttk.Frame(self.frame, padding=6)
        self.box.pack(fill="both", expand=True)

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · propuestas del día hanseático")
        for w in self.box.winfo_children():
            w.destroy()
        if not screen["proposals"]:
            ttk.Label(self.box, text="No hay propuestas pendientes.").pack(anchor="w")
        for p in screen["proposals"]:
            row = ttk.Frame(self.box)
            row.pack(fill="x", pady=2)
            ttk.Label(row, text=f"Propuesta {p['task']}").pack(side="left")
            due = screen["tick"] + 0x800
            ttk.Button(row, text="Aceptar", command=lambda p=p, due=due: self.act("decide", task=p["task"], code=p["bytes"][0], due=due)).pack(side="left", padx=4)
            ttk.Button(row, text="Rechazar", command=lambda p=p, due=due: self.act("decide", task=p["task"], code=6, due=due)).pack(side="left")


class OfficeWindow(ScreenWindow):
    """Oficina: comercio automático por mercancía (op 0x5B). Sin administrador el original borra los ajustes."""
    name = "office"
    title = "Oficina"
    size = "420x300"

    def build(self):
        self.info = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.info, padding=6).pack(anchor="w")
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        self.vars = {k: tk.IntVar(value=v) for k, v in (("ware", 0), ("amount", 0), ("price", 0))}
        for label, key in (("Mercancía (0-23)", "ware"), ("Cantidad", "amount"), ("Precio", "price")):
            ttk.Label(row, text=label).pack(side="left")
            ttk.Spinbox(row, from_=0, to=1000000, textvariable=self.vars[key], width=8).pack(side="left", padx=4)
        ttk.Button(self.frame, text="Fijar", command=lambda: self.act("autotrade", **{k: int(v.get()) for k, v in self.vars.items()})).pack(anchor="w", padx=8)
        self.admin = ttk.Button(self.frame, text="Contratar administrador", command=self.toggle_admin)
        self.admin.pack(anchor="w", padx=8, pady=6)
        self.has_admin = False

    def toggle_admin(self):
        self.act("fire_administrator" if self.has_admin else "hire_administrator")

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · oficina")
        active = [w for w in screen["wares"] if w["amount"]]
        self.has_admin = bool(screen["administrator"])
        self.admin.configure(text="Despedir al administrador" if self.has_admin else "Contratar administrador")
        extra = f" (jornal {screen['administrator_wage']}, habilidad {screen['administrator_skill']})" if self.has_admin and screen.get("administrator_wage") is not None else ""
        self.info.set(("Administrador contratado" + extra if self.has_admin else "Sin administrador: no hay comercio automático") +
                      f"\nMercancías con comercio automático: {len(active)}")


WARE_NAMES = ["Cereales", "Carne", "Pescado", "Cerveza", "Sal", "Miel", "Especias", "Vino", "Paño", "Pieles", "Aceite de ballena", "Madera", "Artículos de hierro", "Cuero", "Lana", "Brea", "Hierro en bruto", "Cáñamo", "Cerámica", "Ladrillos"]


class ShipyardWindow(ScreenWindow):
    """Astillero: mejorar un nivel de casco a los barcos atracados (op 0x0C → start_repair); muestra coste y días del presupuesto."""
    name = "shipyard"
    title = "Astillero"
    size = "520x320"

    def build(self):
        self.box = ttk.Frame(self.frame, padding=6)
        self.box.pack(fill="both", expand=True)

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · oro {money(screen['gold'])}")
        for w in self.box.winfo_children():
            w.destroy()
        if not screen["ships"]:
            ttk.Label(self.box, text="No tienes barcos en este puerto").pack(anchor="w")
        for order in screen.get("orders", []):
            row = ttk.Frame(self.box)
            row.pack(fill="x", pady=2)
            if order["cost"] is None:
                ttk.Label(row, text=f"Encargar {order['name']}: no disponible ahora (materiales, dinero o nivel)").pack(side="left")
            else:
                ttk.Label(row, text=f"Encargar {order['name']}: {money(order['cost'])}").pack(side="left")
                ttk.Button(row, text="Encargar", command=lambda order=order: self.act("order", kind=order["kind"])).pack(side="left", padx=6)
        for q in screen.get("queue", []):
            row = ttk.Frame(self.box)
            row.pack(fill="x", pady=2)
            names = ["snaikka", "crayer", "cog", "hulk"]
            if q["refund"] is None:
                txt = f"En obra: {names[q['kind'] & 3]} nivel {q['level']} ({q['progress']}/{q['target']}); cancelar no devuelve dinero"
            else:
                txt = f"En cola: {names[q['kind'] & 3]} nivel {q['level']}; cancelar devuelve {money(q['refund'])}"
            ttk.Label(row, text=txt).pack(side="left")
            ttk.Button(row, text="Cancelar encargo", command=lambda q=q: self.act("cancel_order", ship=q["ship"])).pack(side="left", padx=6)
        for cv in screen.get("convoys", []):
            row = ttk.Frame(self.box)
            row.pack(fill="x", pady=2)
            ttk.Label(row, text=f"Convoy {cv['convoy']}: reparar todos sus barcos por {money(cv['cost'])}").pack(side="left")
            ttk.Button(row, text="Reparar convoy", command=lambda cv=cv: self.act("repair_convoy", convoy=cv["convoy"])).pack(side="left", padx=6)
        for sh in screen["ships"]:
            row = ttk.Frame(self.box)
            row.pack(fill="x", pady=2)
            if sh["in_yard"]:
                text = f"Barco {sh['ship']}: nivel {sh['level']}, en obras"
            elif not sh["docked"]:
                text = f"Barco {sh['ship']}: nivel {sh['level']}, no atracado"
            elif sh["quote"] is None or sh["level"] >= 2:
                text = f"Barco {sh['ship']}: nivel {sh['level']} (máximo)"
            else:
                q = sh["quote"]
                text = f"Barco {sh['ship']}: nivel {sh['level']} → {sh['level'] + 1}, {money(q['cost'])} y {q['days']} días"
            ttk.Label(row, text=text).pack(side="left")
            q = sh["quote"]
            if sh["docked"] and q and q["affordable"] and sh["level"] < 2:
                ttk.Button(row, text="Mejorar", command=lambda sh=sh: self.act("upgrade", ship=sh["ship"], levels=1)).pack(side="left", padx=6)


class LettersWindow(ScreenWindow):
    """Buzón (cartas del comerciante): marcar como leída (0x6E), borrar (0x6F) y contestar con un pago (0x55)."""
    name = "letters"
    title = "Cartas"
    size = "640x420"

    @staticmethod
    def describe(letter):
        notes = letter.get("notes")
        head = f"(el texto redactado de este tipo lo compone el lector del juego; descripción de la investigación)\n\n"
        if not notes:
            return head + f"Datos: {letter['words']}, {letter['long']}"
        extra = ""
        if letter.get("city_name"):
            extra += f"Ciudad: {letter['city_name']}\n"
        d = letter.get("detail")
        if d:
            extra += "Datos: " + ", ".join(f"{k}={v}" for k, v in d.items() if v is not None) + "\n"
        head = head + extra + "\n" if extra else head
        return head + f"{notes[0]}\n\nw0, w1: {letter['words']}. {notes[1]}\nu32: {letter['long']}. {notes[2]}"

    def build(self):
        self.box = tk.Listbox(self.frame, height=14, exportselection=False)
        self.box.pack(fill="both", expand=True, padx=6, pady=4)
        self.box.bind("<<ListboxSelect>>", self.preview)
        # Página dibujada por la maqueta original (TextLayout 0x00420A10 en el enlazador ui_rt); el texto queda de reserva.
        self.page = tk.Label(self.frame, borderwidth=0)
        self.page_image = None
        self.body = tk.Text(self.frame, height=10, wrap="word", state="disabled")
        self.body.pack(fill="x", padx=6)
        row = self.row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        ttk.Button(row, text="Marcar leída", command=lambda: self.press("read")).pack(side="left", padx=4)
        ttk.Button(row, text="Borrar", command=lambda: self.press("delete")).pack(side="left", padx=4)
        self.payment = tk.IntVar(value=0)
        ttk.Label(row, text="Pago").pack(side="left", padx=(12, 2))
        ttk.Spinbox(row, from_=0, to=100000000, textvariable=self.payment, width=10).pack(side="left")
        ttk.Button(row, text="Contestar", command=lambda: self.press("answer")).pack(side="left", padx=4)
        self.letters = []

    def preview(self, _event=None):
        sel = self.box.curselection()
        letter = self.letters[sel[0]] if sel else None
        self.body.configure(state="normal")
        self.body.delete("1.0", "end")
        if letter and letter.get("layout"):
            self.render_layout(letter["layout"])
        elif letter:
            self.body.insert("end", letter.get("composed") or letter["text"] or self.describe(letter))
        self.body.configure(state="disabled")
        self.show_page(None)
        if letter and letter.get("composed_raw"):
            wanted = sel[0]
            self.client.request({"command": "ui_text", "text": letter["composed_raw"], "width": 600},
                                lambda data: self.page_ready(wanted, data))

    def page_ready(self, wanted, data):
        sel = self.box.curselection() if self.alive() else ()
        if not sel or sel[0] != wanted or data.get("error") or not data.get("png"):
            return
        self.show_page(data)

    def show_page(self, data):
        """Muestra la página del enlazador (PNG en base64) en lugar del texto de reserva, o vuelve al texto."""
        if data is None:
            self.page.pack_forget()
            self.body.pack(fill="x", padx=6, before=self.row)
            self.page_image = None
            return
        try:
            self.page_image = tk.PhotoImage(data=data["png"])
        except tk.TclError:
            return
        self.page.configure(image=self.page_image)
        self.body.pack_forget()
        self.page.pack(fill="x", padx=6, before=self.row)
        self.fit()

    ICONS = {"C": "\u00a4", "B": "\u25a0", "L": "\u25c6"}

    def render_layout(self, lines):
        """Pinta la maqueta de `0x00462520` que manda el núcleo: alineación por línea, fuente 1 en negrita, iconos y capitales (WIN-04)."""
        body = self.body
        body.tag_configure("left", justify="left")
        body.tag_configure("right", justify="right")
        body.tag_configure("center", justify="center")
        body.tag_configure("font1", font=("Segoe UI", 10, "bold"))
        body.tag_configure("initial", font=("Times New Roman", 18, "bold"))
        for n, line in enumerate(lines):
            start = body.index("end-1c")
            for run in line["runs"]:
                tags = ("font1",) if run["font"] == 1 else ()
                if "text" in run:
                    body.insert("end", run["text"], tags)
                elif "icon" in run:
                    body.insert("end", self.ICONS.get(run["icon"], ""), tags)
                elif "initial" in run:
                    body.insert("end", run["initial"], ("initial",))
            if n + 1 < len(lines):
                body.insert("end", "\n")
            body.tag_add(line["align"], start, body.index("end-1c"))

    def press(self, action):
        sel = self.box.curselection()
        if not sel:
            self.status.set("Elige una carta")
            return
        letter = self.letters[sel[0]]
        if action == "answer":
            if not letter["answerable"]:
                self.status.set("Esta carta no admite respuesta")
                return
            self.act("answer", letter=letter["letter"], payment=int(self.payment.get()), city=0, bits=0)
        else:
            self.act(action, letter=letter["letter"])

    def show(self, screen):
        if not self.alive():
            return
        self.letters = screen["letters"]
        self.head.set(f"Cartas · {screen['unread']} sin leer · {money(screen['gold'])} monedas")
        self.box.delete(0, "end")
        for l in self.letters:
            mark = "● " if l["unread"] else "  "
            kind = (l["name"] or f"tipo {l['kind']}").replace("letter_", "").replace("_", " ")
            self.box.insert("end", f"{mark}{l['day']:02d}/{l['month']:02d}/{l['year']}  {kind}" + ("  (contestada)" if l["answered"] else ""))


class MarketWindow(ScreenWindow):
    """Plaza del mercado: precios de un lote, existencias y compra/venta de la oficina o de un barco atracado."""
    name = "market"
    title = "Mercado"
    size = "760x520"

    def build(self):
        cols = ("ware", "unit", "stock", "buy", "sell", "office")
        self.tree = ttk.Treeview(self.frame, columns=cols, show="headings", height=14, selectmode="browse")
        for c, label, w in (("ware", "Mercancía", 170), ("unit", "Lote", 70), ("stock", "Ciudad (lotes)", 100), ("buy", "Compra", 80), ("sell", "Venta", 80), ("office", "Oficina (lotes)", 110)):
            self.tree.heading(c, text=label)
            self.tree.column(c, width=w, anchor="w" if c == "ware" else "e")
        self.tree.pack(fill="both", expand=True, padx=6, pady=4)
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        self.lots = tk.IntVar(value=1)
        ttk.Label(row, text="Lotes").pack(side="left")
        ttk.Spinbox(row, from_=1, to=50, textvariable=self.lots, width=4).pack(side="left", padx=4)
        self.target = tk.StringVar(value="oficina")
        self.combo = ttk.Combobox(row, textvariable=self.target, state="readonly", width=14, values=["oficina"])
        self.combo.pack(side="left", padx=6)
        ttk.Button(row, text="Comprar", command=lambda: self.trade(True)).pack(side="left", padx=4)
        ttk.Button(row, text="Vender", command=lambda: self.trade(False)).pack(side="left", padx=4)
        self.wares = []

    def trade(self, buy):
        sel = self.tree.selection()
        if not sel:
            self.status.set("Elige una mercancía")
            return
        w = next(x for x in self.wares if x["ware"] == int(sel[0]))
        args = {"buy": int(buy), "ware": w["ware"], "quantity": w["lot"] * int(self.lots.get())}
        if self.target.get().startswith("barco "):
            args["ship"] = int(self.target.get().split()[1])
        elif self.target.get().startswith("convoy "):
            args["convoy"] = int(self.target.get().split()[1])
        self.act("trade", **args)

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · mercado · {money(screen['gold'])} monedas")
        # Como «Trading goods» del original: orden alfabético y existencias en lotes enteros (barril = 200 unidades, carga = 2000).
        self.wares = sorted(screen["wares"], key=lambda w: unicodedata.normalize("NFD", WARE_NAMES[w["ware"]]).lower())
        self.combo.configure(values=["oficina"] + [f"barco {s['ship']}" for s in screen["ships"]] + [f"convoy {c['convoy']}" for c in screen["convoys"]])
        keep = self.tree.selection()
        self.tree.delete(*self.tree.get_children())
        for i, w in enumerate(self.wares):
            lots = lambda v: v // w["lot"] or ""
            self.tree.insert("", "end", iid=str(w["ware"]), values=(WARE_NAMES[w["ware"]], "barril" if w["lot"] == 200 else "carga", lots(w["stock"]), money(w["buy_price"]), money(w["sell_price"]), lots(w["office"])))
        if keep and self.tree.exists(keep[0]):
            self.tree.selection_set(keep[0])


class FleetWindow(ScreenWindow):
    """Flota: barcos propios con tripulación, capitán y carga; despedir marineros (0x05) y pasarlos a otro barco (0x10)."""
    name = "fleet"
    title = "Flota"
    size = "760x500"

    def build(self):
        cols = ("ship", "state", "crew", "need", "captain", "load")
        self.tree = ttk.Treeview(self.frame, columns=cols, show="headings", height=12, selectmode="browse")
        for c, label, w in (("ship", "Barco", 120), ("state", "Estado", 120), ("crew", "Marineros", 90), ("need", "Faltan", 70), ("captain", "Capitán", 80), ("load", "Carga", 140)):
            self.tree.heading(c, text=label)
            self.tree.column(c, width=w, anchor="w")
        self.tree.pack(fill="both", expand=True, padx=6, pady=4)
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        self.amount = tk.IntVar(value=1)
        ttk.Label(row, text="Marineros").pack(side="left")
        ttk.Spinbox(row, from_=1, to=500, textvariable=self.amount, width=5).pack(side="left", padx=4)
        ttk.Button(row, text="Despedir", command=self.dismiss).pack(side="left", padx=4)
        self.dest = tk.StringVar()
        self.combo = ttk.Combobox(row, textvariable=self.dest, state="readonly", width=12)
        self.combo.pack(side="left", padx=(12, 4))
        ttk.Button(row, text="Pasar a ese barco", command=self.transfer).pack(side="left", padx=4)
        ttk.Button(row, text="Pasar sables", command=lambda: self.transfer("transfer_cutlasses")).pack(side="left", padx=4)
        convoy = ttk.Frame(self.frame, padding=6)
        convoy.pack(fill="x")
        ttk.Label(convoy, text="Convoy:").pack(side="left")
        ttk.Button(convoy, text="Formar convoy aquí", command=lambda: self.convoy("convoy_depart")).pack(side="left", padx=4)
        ttk.Button(convoy, text="Unir al convoy del barco elegido a la derecha", command=self.join).pack(side="left", padx=4)
        ttk.Button(convoy, text="Sacar del convoy", command=lambda: self.convoy("convoy_remove")).pack(side="left", padx=4)
        ttk.Button(convoy, text="Disolver su convoy", command=self.disband).pack(side="left", padx=4)
        self.formations = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.formations, padding=6, justify="left").pack(fill="x")
        self.ships = []
        self.convoys_now = []

    def convoy(self, action):
        ship = self.selected()
        if ship is None or not ship["docked_here"]:
            self.status.set("Elige un barco atracado aquí")
            return
        self.act(action, ship=ship["ship"])

    def join(self):
        ship = self.selected()
        if ship is None or not ship["docked_here"] or not self.dest.get():
            self.status.set("Elige el barco que se une y, en el desplegable, un barco del convoy")
            return
        self.act("convoy_join", member=int(self.dest.get().split()[1]), ship=ship["ship"])

    def disband(self):
        ship = self.selected()
        if ship is None or ship["convoy"] == 65535:
            self.status.set("Elige un barco que esté en un convoy")
            return
        self.act("convoy_disband", convoy=ship["convoy"])

    def selected(self):
        sel = self.tree.selection()
        return self.ships[int(sel[0])] if sel else None

    def dismiss(self):
        ship = self.selected()
        if ship is None or not ship["docked_here"]:
            self.status.set("Elige un barco atracado aquí")
            return
        self.act("dismiss_sailors", ship=ship["ship"], amount=int(self.amount.get()))

    def transfer(self, action="transfer_sailors"):
        ship = self.selected()
        if ship is None or not ship["docked_here"] or not self.dest.get():
            self.status.set("Elige un barco atracado aquí y el destino")
            return
        self.act(action, **{"from": ship["ship"], "to": int(self.dest.get().split()[1]), "amount": int(self.amount.get())})

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · flota · {screen['hireable_sailors']} marineros disponibles")
        self.ships = screen["ships"]
        self.formations.set("\n".join(f"Convoy {f['convoy']}: líder barco {f['leader']}, barcos {', '.join(map(str, f['members']))}" for f in screen.get("formations", [])) or "Sin convoyes formados")
        self.combo.configure(values=[f"barco {s['ship']}" for s in self.ships if s["docked_here"]])
        keep = self.tree.selection()
        self.tree.delete(*self.tree.get_children())
        for i, s in enumerate(self.ships):
            where = "atracado aquí" if s["docked_here"] else ("atracado" if s["state"] < 2 else "en ruta")
            captain = "—" if s["captain"] == 65535 else s["captain"]
            load = "—" if s["capacity"] is None else f"{money(s['load'])}/{money(s['capacity'])}"
            self.tree.insert("", "end", iid=str(i), values=(f"Barco {s['ship']}", where, s["crew"], s["needed_crew"], captain, load))
        if keep and self.tree.exists(keep[0]):
            self.tree.selection_set(keep[0])


class WarehouseWindow(ScreenWindow):
    """Almacén de la oficina: existencias y precio medio, y traspaso de mercancía con un barco atracado (op 0x08)."""
    name = "warehouse"
    title = "Almacén"
    size = "780x520"

    def build(self):
        cols = ("ware", "office", "avg", "ship")
        self.tree = ttk.Treeview(self.frame, columns=cols, show="headings", height=14, selectmode="browse")
        for c, label, w in (("ware", "Mercancía", 170), ("office", "Oficina", 110), ("avg", "Precio medio", 110), ("ship", "En el barco", 110)):
            self.tree.heading(c, text=label)
            self.tree.column(c, width=w, anchor="w" if c == "ware" else "e")
        self.tree.pack(fill="both", expand=True, padx=6, pady=4)
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        self.amount = tk.IntVar(value=200)
        ttk.Label(row, text="Cantidad").pack(side="left")
        ttk.Spinbox(row, from_=10, to=1000000, increment=200, textvariable=self.amount, width=9).pack(side="left", padx=4)
        self.target = tk.StringVar()
        self.combo = ttk.Combobox(row, textvariable=self.target, state="readonly", width=12)
        self.combo.pack(side="left", padx=6)
        ttk.Button(row, text="Cargar en el barco", command=lambda: self.move(True)).pack(side="left", padx=4)
        ttk.Button(row, text="Descargar a la oficina", command=lambda: self.move(False)).pack(side="left", padx=4)
        self.screen = None
        self.tree.bind("<<TreeviewSelect>>", lambda e: None)

    def move(self, to_ship):
        sel = self.tree.selection()
        if not sel or not self.target.get():
            self.status.set("Elige una mercancía y un barco")
            return
        kind, number = self.target.get().split()
        if kind == "convoy":
            self.act("convoy_transfer", convoy=int(number), ware=int(sel[0]), amount=int(self.amount.get()), to_convoy=int(to_ship))
        else:
            self.act("transfer", ship=int(number), ware=int(sel[0]), amount=int(self.amount.get()), to_ship=int(to_ship))

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · almacén · {money(screen['gold'])} monedas")
        names = WARE_NAMES + ["Ballestas", "Espadas", "Cañones", "Armaduras"]
        ships = screen["ships"]
        convoys = screen["convoys"]
        self.combo.configure(values=[f"barco {s['ship']}" for s in ships] + [f"convoy {c['convoy']}" for c in convoys])
        if ships and not self.target.get():
            self.target.set(f"barco {ships[0]['ship']}")
        current = next((s for s in ships if self.target.get() == f"barco {s['ship']}"), None) or next((c for c in convoys if self.target.get() == f"convoy {c['convoy']}"), None)
        keep = self.tree.selection()
        self.tree.delete(*self.tree.get_children())
        for w in screen["wares"]:
            held = current["cargo"][w["ware"]] if current else 0
            self.tree.insert("", "end", iid=str(w["ware"]), values=(names[w["ware"]], money(w["amount"]), f"{w['average_price']:.1f}", money(held)))
        if keep and self.tree.exists(keep[0]):
            self.tree.selection_set(keep[0])


class BathsWindow(ScreenWindow):
    """Baños: entrar (0x39) y sobornar a un concejal para ocupar su escaño (0x42 si la oferta alcanza el precio, 0x43 si no)."""
    name = "baths"
    title = "Baños"
    size = "560x340"

    def build(self):
        self.tree = ttk.Treeview(self.frame, columns=("seat", "holder", "state"), show="headings", height=5, selectmode="browse")
        for c, label, w in (("seat", "Escaño", 80), ("holder", "Ocupa", 140), ("state", "Estado", 220)):
            self.tree.heading(c, text=label)
            self.tree.column(c, width=w, anchor="w")
        self.tree.pack(fill="x", padx=6, pady=4)
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        self.offer = tk.IntVar(value=1000)
        ttk.Label(row, text="Oferta").pack(side="left")
        ttk.Spinbox(row, from_=0, to=999999, increment=1000, textvariable=self.offer, width=9).pack(side="left", padx=4)
        ttk.Button(row, text="Entrar en los baños", command=lambda: self.act("enter")).pack(side="left", padx=8)
        ttk.Button(row, text="Sobornar", command=self.bribe).pack(side="left", padx=4)
        self.seats = []

    def bribe(self):
        sel = self.tree.selection()
        if not sel or not self.seats[int(sel[0])]["can_bribe"]:
            self.status.set("Elige un escaño que no sea tuyo y donde no hayas fallado hoy")
            return
        self.act("bribe", seat=self.seats[int(sel[0])]["seat"], amount=int(self.offer.get()))

    def done(self, result):
        if "outcome" in result:
            self.status.set(("Soborno aceptado" + (" (pagaste de más)" if result["overpaid"] else "")) if result["outcome"] == "success" else "El concejal rechaza la oferta")
        super().done(result)

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · baños · {money(screen['gold'])} monedas")
        self.seats = screen["seats"]
        self.tree.delete(*self.tree.get_children())
        for i, s in enumerate(self.seats):
            state = "tuyo" if s["mine"] else ("ya falló hoy" if s["failed_today"] else "se puede sobornar")
            self.tree.insert("", "end", iid=str(i), values=(s["seat"] + 1, s.get("holder_name") or f"comerciante {s['holder']}", state))


class WeaponsWindow(ScreenWindow):
    """Armero (Weapon smith, edificio 3; no es la Armería/Arsenal 33 de la milicia): lotes de armamento de barco de la ciudad (tipos 0 a 5) y alfanjes (tipo 6); comprar o vender uno por orden (0x56)."""
    name = "weapons"
    title = "Armero"
    size = "700x330"
    KINDS = ["Catapulta pequeña", "Balista pequeña", "Catapulta grande", "Balista grande", "Bombarda", "Cañón", "Alfanjes"]

    def build(self):
        cols = (("kind", "Arma", 210), ("town", "Ciudad", 80), ("office", "Tu oficina", 90), ("buy", "Compra", 80), ("sell", "Venta", 80))
        self.tree = ttk.Treeview(self.frame, columns=[c[0] for c in cols], show="headings", height=7, selectmode="browse")
        for c, label, w in cols:
            self.tree.heading(c, text=label)
            self.tree.column(c, width=w, anchor="w")
        self.tree.pack(fill="x", padx=6, pady=4)
        row = ttk.Frame(self.frame, padding=6)
        row.pack(fill="x")
        ttk.Button(row, text="Comprar un lote", command=lambda: self.deal("buy")).pack(side="left", padx=4)
        ttk.Button(row, text="Vender un lote", command=lambda: self.deal("sell")).pack(side="left", padx=4)

    def deal(self, action):
        sel = self.tree.selection()
        if not sel:
            self.status.set("Elige una fila")
            return
        self.act(action, kind=int(sel[0]))

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · armero · {money(screen['gold'])} monedas")
        self.tree.delete(*self.tree.get_children())
        for r in screen["rows"]:
            name = self.KINDS[r["kind"]]
            self.tree.insert("", "end", iid=str(r["kind"]), values=(name, r["town"], r["office"], r["buy_price"], r["sell_price"]))


class GuildWindow(ScreenWindow):
    """Gremio de la ciudad: pagar la cuota de ingreso (orden 0x37, que da el conocimiento del mapa de la ciudad)."""
    name = "guild"
    title = "Gremio"
    size = "420x170"

    def build(self):
        self.info = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.info, padding=10, justify="left").pack(fill="x")
        self.button = ttk.Button(self.frame, text="Pagar la cuota e ingresar", command=lambda: self.act("join"))
        self.button.pack(pady=6)

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{screen['town_name']} · gremio · {money(screen['gold'])} monedas")
        state = "ya perteneces al gremio" if screen["member"] else ("el ingreso no está permitido en esta partida" if screen["locked"] else "puedes ingresar" if screen["can_join"] else "oro insuficiente")
        self.info.set(f"Cuota de ingreso: {money(screen['fee'])} monedas\n{state}")
        self.button.state(["!disabled"] if screen["can_join"] else ["disabled"])


class FoundWindow(ScreenWindow):
    """Alta de oficinas (orden 0x5D): ciudades sin oficina con su presupuesto."""
    name = "found"
    title = "Fundar oficina"
    size = "420x380"

    def build(self):
        self.tree = ttk.Treeview(self.frame, columns=("cost",), height=14)
        self.tree.heading("#0", text="Ciudad")
        self.tree.heading("cost", text="Coste")
        self.tree.pack(fill="both", expand=True, padx=6, pady=6)
        ttk.Button(self.frame, text="Fundar oficina en la ciudad", command=self.found).pack(pady=6)

    def found(self):
        sel = self.tree.selection()
        if sel:
            self.act("found", city=int(sel[0]))

    def show(self, screen):
        if not self.alive():
            return
        self.head.set(f"{money(screen['gold'])} monedas")
        self.tree.delete(*self.tree.get_children())
        for c in screen["cities"]:
            if not c["has_office"]:
                self.tree.insert("", "end", iid=str(c["city"]), text=c["name"], values=(money(c["cost"]) if c["can_found"] else "no disponible",))


class StatisticsWindow(ScreenWindow):
    """Estadísticas del jugador: últimas 32 muestras diarias de los campos acreditados."""
    name = "statistics"
    title = "Estadísticas"
    size = "460x340"
    NAMES = {"company_value": "Valor de la compañía", "ship_sales_income": "Ventas de barcos", "ship_purchases": "Compras de barcos", "stock_quantity": "Existencias",
             "stock_value": "Valor de existencias", "ships_built": "Barcos construidos", "buildings": "Edificios", "houses": "Casas", "offices": "Oficinas",
             "workers": "Trabajadores", "home_reputation": "Reputación en la ciudad natal"}

    def build(self):
        self.tree = ttk.Treeview(self.frame, columns=("now", "min", "max"), height=12)
        for col, text in (("#0", "Serie"), ("now", "Última"), ("min", "Mín."), ("max", "Máx.")):
            self.tree.heading(col, text=text)
        self.tree.pack(fill="both", expand=True, padx=6, pady=6)
        row = ttk.Frame(self.frame)
        row.pack(fill="x", padx=6)
        self.mode = tk.StringVar(value="1A casas")
        ttk.Combobox(row, textvariable=self.mode, state="readonly", values=list(self.MODES)).pack(side="left")
        ttk.Button(row, text="Comparar comerciantes", command=self.compare).pack(side="left", padx=6)
        self.compare_info = tk.StringVar()
        ttk.Label(self.frame, textvariable=self.compare_info, padding=6, justify="left").pack(fill="x")
        self.owners = []

    MODES = {"10 valor": 0x10, "11 capacidad": 0x11, "12 reputación total": 0x12, "16 trabajadores": 0x16, "17 campo 0x474": 0x17, "18 barcos construidos": 0x18,
             "19 edificios": 0x19, "1A casas": 0x1A, "1B reputación local": 0x1B}

    def compare(self):
        self.client.request({"command": "screen_action", "name": self.name, "action": "compare", "args": {"mode": self.MODES[self.mode.get()], "owners": self.owners[:4], "period": "daily"}}, self.compared)

    def compared(self, result):
        lines = [f"{s['label'] or s['owner']}: {s['values'][-1]:,.0f}" for s in result.get("series", [])]
        self.compare_info.set("\n".join(lines) or result.get("error", ""))

    def show(self, screen):
        if not self.alive():
            return
        if "merchants" in screen:
            self.owners = screen["merchants"]
        self.head.set(f"{screen['years']} años de histórico")
        self.tree.delete(*self.tree.get_children())
        for key, values in screen["daily"].items():
            self.tree.insert("", "end", text=self.NAMES.get(key, key), values=(f"{values[-1]:,.0f}", f"{min(values):,.0f}", f"{max(values):,.0f}"))


WINDOWS = {"shipyard": ShipyardWindow, "office": OfficeWindow, "hanse": HanseWindow, "lender": LenderWindow, "church": ChurchWindow, "town_hall": TownHallWindow, "tavern": TavernWindow, "letters": LettersWindow, "market": MarketWindow, "fleet": FleetWindow, "warehouse": WarehouseWindow, "baths": BathsWindow, "weapons": WeaponsWindow, "guild": GuildWindow, "found": FoundWindow, "statistics": StatisticsWindow}
LABELS = {"shipyard": "Astillero", "office": "Oficina", "hanse": "Hansa", "lender": "Prestamista", "church": "Iglesia", "town_hall": "Ayuntamiento", "tavern": "Taberna", "letters": "Cartas", "market": "Mercado", "fleet": "Flota", "warehouse": "Almacén", "baths": "Baños", "weapons": "Armero", "guild": "Gremio", "found": "Fundar oficina", "statistics": "Estadísticas"}


def open_screen(client, name):
    return WINDOWS[name](client)

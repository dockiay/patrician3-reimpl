"""Cliente de escritorio del prototipo; la simulación y los recursos son Rust."""
import json
import os
import queue
import subprocess
import sys
import tempfile
import threading
import time
import tkinter as tk
from tkinter import filedialog, messagebox, ttk
from pathlib import Path
import winsound
import wave
import ctypes
sys.path.insert(0, str(Path(__file__).resolve().parent))
import screens_ui

WARES = ["Cereales", "Carne", "Pescado", "Cerveza", "Sal", "Miel", "Especias", "Vino", "Paño", "Pieles", "Aceite de ballena", "Madera", "Artículos de hierro", "Cuero", "Lana", "Brea", "Hierro en bruto", "Cáñamo", "Cerámica", "Ladrillos", "Espadas", "Arcos", "Ballestas", "Carabinas"]


# Tipos que el motor ya sabe empezar (comprobado uno a uno en la partida de prueba, `plan_probe`). Los nombres traducen la tabla de
# nombres del ejecutable (0x006A57C8, índice = tipo; ver research/systems/ciudades/BUILDING_TYPES.md).
TYPE_NAMES = {0: "Armería (sin uso)", 1: "Astillero de reparación", 2: "Camino", 3: "Armero", 4: "Pabellón de caza", 5: "Casa de pescador", 6: "Cervecería", 7: "Taller", 8: "Colmenar", 9: "Granja de cereales", 10: "Granja de ganado", 11: "Aserradero", 12: "Telar", 13: "Salinas", 14: "Fundición de hierro", 15: "Granja de ovejas", 16: "Viñedo", 17: "Alfarería", 18: "Ladrillar", 19: "Fábrica de brea", 20: "Cultivo de cáñamo", 21: "Casa de mercader (A)", 22: "Casa de mercader (B)", 23: "Casa de mercader (C)", 24: "Casa con hastial (A)", 25: "Casa con hastial (B)", 26: "Casa con hastial (C)", 27: "Casa entramada (A)", 28: "Casa entramada (B)", 29: "Casa entramada (C)", 30: "Almacén", 31: "Iglesia", 32: "Ayuntamiento", 33: "Armería", 34: "Casa del gremio", 35: "Mercado cubierto", 36: "Taberna", 37: "Casa del prestamista", 38: "Baños públicos", 39: "Oficina de comercio terrestre", 40: "Pozo", 41: "Hospital", 42: "Casa de la moneda", 43: "Escuela", 44: "Capilla", 45: "Monumento", 46: "Astillero", 47: "Mina", 48: "Bombarda del puerto", 49: "Cañón del puerto", 50: "Bombarda de puerta", 51: "Cañón de puerta", 52: "Lanzabrea", 53: "Lanzabrea", 54: "Muralla", 55: "Puerta", 56: "Puerta"}
BUILD_TYPES = [(0x04, "Pabellón de caza"), (0x06, "Cervecería"), (0x07, "Taller"), (0x08, "Colmenar"), (0x09, "Granja de cereales"), (0x0A, "Granja de ganado"), (0x0B, "Aserradero"), (0x0C, "Telar"), (0x0D, "Salinas"), (0x0E, "Fundición de hierro"), (0x0F, "Granja de ovejas"), (0x10, "Viñedo"), (0x11, "Alfarería"), (0x12, "Ladrillar"), (0x13, "Fábrica de brea"), (0x14, "Cultivo de cáñamo"), (0x15, "Casa de mercader (A)"), (0x16, "Casa de mercader (B)"), (0x17, "Casa de mercader (C)"), (0x18, "Casa con hastial (A)"), (0x19, "Casa con hastial (B)"), (0x1A, "Casa con hastial (C)"), (0x1B, "Casa entramada (A)"), (0x1C, "Casa entramada (B)"), (0x1D, "Casa entramada (C)"), (0x28, "Pozo"), (0x29, "Hospital"), (0x2A, "Casa de la moneda"), (0x2B, "Escuela"), (0x2C, "Capilla"), (0x2D, "Monumento"), (0x30, "Bombarda del puerto")]


class Client:
    def __init__(self, executable):
        self.exe = executable
        self.root = tk.Tk()
        self.root.title("Patrician III — prototipo visual")
        self.root.geometry("1200x760")
        self.process = None
        self.temp = tempfile.TemporaryDirectory(prefix="p3-mvp-")
        self.image_path = str(Path(self.temp.name) / "ciudad.png")
        self.busy = False
        self.running = False
        self.full_fleet = False
        self.battle_paused = False
        self.battle_ready = False
        self.battle_mode = 0
        self.tick = 0
        self.timer = None
        self.dirty = False
        self.responses = queue.Queue()
        self.image = None
        self.map_origin = (0, 0)
        self.buildings = []
        self.offices = []
        self.ships = []
        self.destinations = []
        self.city = 0
        self.show_buildings = tk.BooleanVar(value=True)
        self.sound_on = tk.BooleanVar(value=False)
        self.build_type = tk.StringVar(value="")
        self.animate_on = tk.BooleanVar(value=False)
        self.anim_ms = 0
        self.anim_job = None
        self.save_path = tk.StringVar()
        self.cpr_path = tk.StringVar()
        self.message = tk.StringVar(value="Selecciona una partida y un archivo de recursos CPR.")
        self.details = tk.StringVar(value="Sesión sin cargar")
        self.ware = tk.StringVar(value="0")
        self.amount = tk.StringVar(value="200")
        self.build()
        self.root.protocol("WM_DELETE_WINDOW", self.close)
        self.root.after(100, self.poll)

    def build(self):
        top = ttk.Frame(self.root, padding=8)
        top.pack(fill="x")
        for title, variable, kind, patterns in [
            ("Partida", self.save_path, "partida", [("Partidas", "*.pat"), ("Todos", "*.*")]),
            ("Recursos", self.cpr_path, "recursos", [("Archivos CPR", "*.cpr"), ("Todos", "*.*")]),
        ]:
            row = ttk.Frame(top)
            row.pack(fill="x", pady=2)
            ttk.Label(row, text=title, width=10).pack(side="left")
            ttk.Entry(row, textvariable=variable).pack(side="left", fill="x", expand=True)
            ttk.Button(row, text="Elegir…", command=lambda v=variable, k=kind, p=patterns: self.choose(v, k, p)).pack(side="left", padx=4)
        ttk.Button(top, text="Abrir sesión", command=self.open).pack(side="left", pady=4)
        self.controls = []
        for label, cmd in [("Activar flota IA", self.enable_fleet), ("Avanzar un día", self.advance), ("Avanzar combate", self.advance_battle), ("Reanudar / pausa", self.toggle), ("Guardar copia…", self.save)]:
            button = ttk.Button(top, text=label, command=cmd, state="disabled")
            button.pack(side="left", padx=4)
            self.controls.append(button)
        ttk.Label(top, textvariable=self.details).pack(side="right")
        body = ttk.Frame(self.root, padding=(8, 0, 8, 0))
        body.pack(fill="both", expand=True)
        view = ttk.Frame(body)
        view.pack(side="left", fill="both", expand=True)
        self.canvas = tk.Canvas(view, bg="#18242d", highlightthickness=0)
        self.canvas.bind("<ButtonRelease-1>", self.canvas_click, add="+")
        sx = ttk.Scrollbar(view, orient="horizontal", command=self.canvas.xview)
        sy = ttk.Scrollbar(view, orient="vertical", command=self.canvas.yview)
        self.canvas.configure(xscrollcommand=sx.set, yscrollcommand=sy.set)
        self.canvas.grid(row=0, column=0, sticky="nsew")
        sy.grid(row=0, column=1, sticky="ns")
        sx.grid(row=1, column=0, sticky="ew")
        view.rowconfigure(0, weight=1)
        view.columnconfigure(0, weight=1)
        self.canvas.bind("<ButtonPress-1>", lambda e: self.canvas.scan_mark(e.x, e.y))
        self.canvas.bind("<B1-Motion>", lambda e: self.canvas.scan_dragto(e.x, e.y, gain=1))
        # La columna lateral se desplaza por su cuenta: con 1200x760 no caben todos los controles (WIN-02).
        side_area, side_inner = screens_ui.scrolled(body, width=290)
        side_area.pack(side="right", fill="y")
        side = ttk.Frame(side_inner, padding=(14, 2))
        side.pack(fill="both", expand=True)
        ttk.Label(side, text="Oficina", font=("Segoe UI", 16, "bold")).pack(anchor="w")
        self.office_choice = ttk.Combobox(side, state="disabled", width=32)
        self.office_choice.pack(fill="x", pady=4)
        self.office_choice.bind("<<ComboboxSelected>>", self.select_office)
        self.ship_button = ttk.Button(side, text="Consultar barcos", command=self.show_ships, state="disabled")
        self.ship_button.pack(fill="x", pady=4)
        self.controls.append(self.ship_button)
        self.world_button = ttk.Button(side, text="Mapa del mundo", command=self.show_world, state="disabled")
        self.world_button.pack(fill="x", pady=4)
        self.controls.append(self.world_button)
        self.navmap_button = ttk.Button(side, text="Carta náutica", command=self.show_navmap, state="disabled")
        self.navmap_button.pack(fill="x", pady=4)
        self.controls.append(self.navmap_button)
        self.land_button = ttk.Button(side, text="Transporte por tierra", command=self.show_land_transport, state="disabled")
        self.land_button.pack(fill="x", pady=4)
        self.controls.append(self.land_button)
        for key, label in screens_ui.LABELS.items():
            button = ttk.Button(side, text=label, command=lambda k=key: screens_ui.open_screen(self, k), state="disabled")
            button.pack(fill="x", pady=2)
            self.controls.append(button)
        ttk.Label(side, text="Construir (elige tipo y pulsa en el mapa)").pack(anchor="w", pady=(8, 0))
        self.build_choice = ttk.Combobox(side, textvariable=self.build_type, values=[""] + [f"{t:#04x} {n}" for t, n in BUILD_TYPES], state="readonly", width=24)
        self.build_choice.pack(fill="x", pady=2)
        ttk.Checkbutton(side, text="Animar la ciudad (2 s por imagen)", variable=self.animate_on, command=self.toggle_animation).pack(anchor="w", pady=4)
        ttk.Checkbutton(side, text="Marcar edificios del guardado", variable=self.show_buildings, command=self.draw_buildings).pack(anchor="w", pady=4)
        self.sound = ttk.Checkbutton(side, text="Ambiente de mercado", variable=self.sound_on, command=self.toggle_sound, state="disabled")
        self.sound.pack(anchor="w", pady=4)
        ttk.Label(side, text="Existencias de ciudad y oficina; selecciona una mercancía.", wraplength=230).pack(anchor="w", pady=8)
        self.goods = tk.Listbox(side, width=34, height=20, exportselection=False)
        self.goods.pack(fill="x")
        self.goods.bind("<<ListboxSelect>>", self.select_ware)
        row = ttk.Frame(side)
        row.pack(fill="x", pady=6)
        ttk.Label(row, text="Cantidad").pack(side="left")
        ttk.Entry(row, textvariable=self.amount, width=10).pack(side="right")
        for label, side_name in [("Comprar a la ciudad", "buy"), ("Vender a la ciudad", "sell")]:
            button = ttk.Button(side, text=label, command=lambda v=side_name: self.trade(v), state="disabled")
            button.pack(fill="x", pady=3)
            self.controls.append(button)
        ttk.Label(self.root, textvariable=self.message, relief="sunken", anchor="w", padding=6).pack(fill="x", side="bottom")

    def choose(self, variable, kind, patterns):
        path = filedialog.askopenfilename(title=f"Seleccionar {kind}", filetypes=patterns)
        if path:
            variable.set(path)

    def open(self):
        if self.busy:
            return
        if not Path(self.save_path.get()).is_file() or not Path(self.cpr_path.get()).is_file():
            messagebox.showerror("Archivos", "Selecciona una partida y un CPR existentes.")
            return
        if self.dirty and not messagebox.askyesno("Cambios sin guardar", "¿Abrir otra partida y descartar los cambios de esta sesión?"):
            return
        self.stop_process()
        self.dirty = False
        self.full_fleet = False
        self.battle_paused = False
        self.battle_ready = False
        self.battle_mode = 0
        self.image = None
        for button in self.controls:
            button.configure(state="disabled")
        self.sound.configure(state="disabled")
        self.office_choice.configure(state="disabled")
        try:
            self.process = subprocess.Popen([self.exe, self.save_path.get(), self.cpr_path.get(), self.image_path],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, encoding="utf-8", bufsize=1,
                creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
        except OSError as exc:
            messagebox.showerror("Inicio", str(exc))
            return
        self.request({"command": "state"}, self.opened)

    def opened(self, state):
        self.update_state(state)
        self.request({"command": "render"}, self.rendered)

    def rendered(self, result):
        try:
            previous = (self.canvas.xview()[0], self.canvas.yview()[0]) if self.image else None
            self.image = tk.PhotoImage(file=self.image_path)
            self.canvas.delete("all")
            self.canvas.create_image(0, 0, image=self.image, anchor="nw")
            self.canvas.configure(scrollregion=(0, 0, result["width"], result["height"]))
            self.map_origin = result["origin"]
            self.draw_buildings()
            focus_x = max(0, (result["focus"][0] - self.canvas.winfo_width() / 2) / result["width"])
            focus_y = max(0, (result["focus"][1] - self.canvas.winfo_height() / 2) / result["height"])
            self.canvas.xview_moveto(previous[0] if previous else focus_x)
            self.canvas.yview_moveto(previous[1] if previous else focus_y)
            for button in self.controls:
                button.configure(state="normal")
            self.office_choice.configure(state="readonly")
            self.sound.configure(state="normal")
            self.message.set(f"Ciudad cargada. Sprites disponibles: {result['buildings_drawn']}/{result['buildings_total']}; marcas opcionales de registros.")
        except tk.TclError as exc:
            messagebox.showerror("Imagen", str(exc))

    def canvas_cell(self, event):
        """Casilla del plano (fila, columna) bajo el puntero: inversa de la proyección `px = 34 x + (34 | 17)`, `py = 22 (y/2) + (43 | 32)`."""
        px, py = self.canvas.canvasx(event.x), self.canvas.canvasy(event.y)
        ox, oy = self.map_origin
        best = None
        for y in range(max(0, int((py - 43) // 11) - 3), int((py - 32) // 11) + 4):
            base = 34 if y % 2 else 17
            x = round((px - base) / 34)
            cx, cy = 34 * x + base, 22 * (y // 2) + (43 if y % 2 else 32)
            dist = (cx - px) ** 2 + 4 * (cy - py) ** 2
            if best is None or dist < best[0]:
                best = (dist, x, y)
        return (best[2] - oy, best[1] - ox) if best else None

    def canvas_click(self, event):
        choice = self.build_type.get()
        if not choice or not self.image:
            return
        cell = self.canvas_cell(event)
        if cell is None or cell[0] < 0 or cell[1] < 0:
            return
        self.pending_build = (int(choice.split()[0], 16), cell[0], cell[1])
        ty, row, col = self.pending_build
        self.request({"command": "build_check", "type": ty, "row": row, "col": col}, self.build_checked)

    def build_checked(self, result):
        cost = result["cost"]
        if not result["matches"]:
            self.message.set(f"No se puede construir ahí (fila {result['row']}, columna {result['col']}).")
            return
        text = f"Construir tipo {result['type']:#04x} en ({result['row']}, {result['col']})\nLadrillo {cost['brick']:,}, madera {cost['timber']:,}, hierro {cost['iron']:,}, extra {cost['extra']:,}\nOro disponible: {result['gold']:,}"
        if messagebox.askyesno("Construir", text + "\n¿Empezar la obra?"):
            ty, row, col = self.pending_build
            self.request({"command": "build", "type": ty, "row": row, "col": col}, self.build_done)

    def build_done(self, state):
        self.update_state(state)
        self.message.set("Obra empezada.")
        self.request({"command": "render"}, self.rendered)

    def toggle_animation(self):
        if self.anim_job is not None:
            self.root.after_cancel(self.anim_job)
            self.anim_job = None
        if self.animate_on.get():
            self.anim_step()

    def anim_step(self):
        # Reloj de animación del juego (ms); el motor avanza los animadores de forma determinista hasta ese instante.
        self.anim_ms += 2000
        self.request({"command": "render", "clock_ms": self.anim_ms}, self.anim_rendered)

    def anim_rendered(self, result):
        self.rendered(result)
        if self.sound_on.get():
            # Disparadores de la vista de ciudad (llegada y salida de barcos): el motor los pasa por las ranuras con paridad x86.
            self.request({"command": "town_sounds", "clock_ms": self.anim_ms, "sites": result.get("sound_sites", [])}, self.play_town_sounds)
        elif self.animate_on.get():
            self.anim_job = self.root.after(200, self.anim_step)

    def play_town_sounds(self, result):
        self.apply_sounds(result)
        if self.animate_on.get():
            self.anim_job = self.root.after(200, self.anim_step)

    def map_sound(self, kind, ship=None):
        """Sonidos del mapa (0x459790) con la planificación del original: 1 = selección de barco, 4 = ruta marcada.

        Se envían la id y la posición del barco (como `0x45101B`/`0x451DD4`) y una vista de 1024×768 centrada en él; el motor
        elige la muestra, no repite la misma id mientras suena y calcula volumen y panorámica por distancia."""
        if self.sound_on.get() and not self.busy:
            event = {"class": kind, "id": 0, "x": 0, "y": 0}
            view = None
            if ship is not None:
                event.update(id=ship["id"])
                if kind != 1:
                    event.update(x=ship["x"], y=ship["y"])
                view = [ship["x"] - 512, ship["y"] - 384, 1024, 768]
            req = {"command": "town_sounds", "clock_ms": getattr(self, "anim_ms", 0), "map_events": [event]}
            if view:
                req["map_view"] = view
            self.request(req, self.play_sounds_only)

    def play_sounds_only(self, result):
        self.apply_sounds(result)

    def stop_sounds(self):
        """`0x443940` (clase 1 del mapa): cierra todas las muestras abiertas por MCI."""
        winmm = ctypes.windll.winmm
        for alias in list(getattr(self, "mci_open", set())):
            winmm.mciSendStringW(f"close {alias}", None, 0, None)
        self.mci_open = set()

    def set_levels(self, alias, volume, pan):
        """Volumen 0..127 y panorámica 0..127 (64 = centro) de una muestra abierta con `mpegvideo`."""
        winmm = ctypes.windll.winmm
        level = max(0, min(127, volume)) / 127
        left = level * (1 - max(0, (pan - 64) / 63))
        right = level * (1 - max(0, (64 - pan) / 64))
        winmm.mciSendStringW(f"setaudio {alias} left volume to {int(left * 1000)}", None, 0, None)
        winmm.mciSendStringW(f"setaudio {alias} right volume to {int(right * 1000)}", None, 0, None)

    def apply_sounds(self, result):
        """`stop_all`, muestras nuevas y `adjust`: el motor reajusta volumen y panorámica de las que siguen sonando (por id)."""
        if result.get("stop_all"):
            self.stop_sounds()
        for item in result.get("sounds", []):
            self.mix_sound(Path(self.temp.name) / item["file"], item["volume"], item["pan"], item.get("id"))
        for item in result.get("adjust", []):
            for alias in list(getattr(self, "mci_by_id", {}).get(item["id"], ())):
                if alias in getattr(self, "mci_open", set()):
                    self.set_levels(alias, item["volume"], item["pan"])

    def mix_sound(self, path, volume, pan, sound_id=None):
        """Reproduce una muestra a la vez que las demás con MCI: volumen 0..127 y panorámica 0..127 (64 = centro) como el motor.

        El dispositivo `waveaudio` no admite `setaudio` (MCIERR_UNRECOGNIZED_COMMAND, 261, comprobado en Windows 11 25H2):
        se abre con `mpegvideo` (DirectShow), que sí ajusta el volumen por canal, y solo si falla se cae a `waveaudio` sin panorámica."""
        self.mci_count = getattr(self, "mci_count", 0) + 1
        alias = f"p3snd{self.mci_count}"
        winmm = ctypes.windll.winmm
        def mci(command):
            return winmm.mciSendStringW(command, None, 0, None)
        try:
            with wave.open(str(path)) as w:
                millis = int(1000 * w.getnframes() / max(1, w.getframerate()))
        except (wave.Error, OSError):
            return
        if mci(f'open "{path}" type mpegvideo alias {alias}') == 0:
            self.set_levels(alias, volume, pan)
            if sound_id is not None:
                self.mci_by_id = getattr(self, "mci_by_id", {})
                self.mci_by_id.setdefault(sound_id, set()).add(alias)
        elif mci(f'open "{path}" type waveaudio alias {alias}') != 0:
            return
        if mci(f"play {alias}") != 0:
            mci(f"close {alias}")
            return
        opened = getattr(self, "mci_open", set())
        opened.add(alias)
        self.mci_open = opened
        def close():
            mci(f"close {alias}")
            self.mci_open.discard(alias)
            for aliases in getattr(self, "mci_by_id", {}).values():
                aliases.discard(alias)
        self.root.after(millis + 300, close)

    def city_name(self, city):
        for d in self.destinations:
            if d["city"] == city:
                return d.get("name") or f"Ciudad {d['town_id']}"
        return f"Ciudad {city}"

    def update_state(self, state):
        self.tick = state["tick"]
        self.battle_paused = state.get("battle_paused", False)
        self.battle_ready = state.get("battle_ready", False)
        self.details.set(f"{state['day']:02}/{state['month']:02}/{state['year']}    Oro: {state['gold']:,}    Población: {sum(state['population']):,}    {state.get('town_name') or 'Ciudad ' + str(state['town_id'])}")
        chosen = self.goods.curselection()
        index = chosen[0] if chosen else 0
        self.goods.delete(0, "end")
        for i, (town, office) in enumerate(zip(state["town_stock"], state["office_stock"])):
            self.goods.insert("end", f"{i:02} {WARES[i]:<16} {town:>8} / {office:>8}")
        self.goods.selection_set(index)
        self.ware.set(str(index))
        self.offices = state["offices"]
        self.ships = state["ships"]
        self.destinations = state["destinations"]
        self.city = state["city"]
        self.office_choice.configure(values=[f"{self.city_name(o['city'])} · mercader {o['owner']} · oficina {o['office']}" for o in self.offices])
        for i, office in enumerate(self.offices):
            if office["office"] == state["office"]:
                self.office_choice.current(i)
                break
        changed = self.buildings != state["buildings"]
        self.buildings = state["buildings"]
        self.draw_buildings()
        return changed

    def select_office(self, _event):
        index = self.office_choice.current()
        if index < 0 or index >= len(self.offices) or self.busy:
            return
        self.running = False
        if self.timer is not None:
            self.root.after_cancel(self.timer)
            self.timer = None
        self.request({"command": "office", "index": self.offices[index]["office"]}, self.office_selected)

    def office_selected(self, state):
        self.image = None
        self.update_state(state)
        self.request({"command": "render"}, self.rendered)

    def show_world(self):
        self.request({"command": "render_world"}, self.world_ready)

    def world_ready(self, result):
        path = Path(self.image_path).with_name("mundo.png")
        window = tk.Toplevel(self.root)
        window.title("Mapa del mundo")
        window.geometry("1100x760")
        zoom = tk.IntVar(value=2)
        ships_on = tk.BooleanVar(value=True)
        bar = ttk.Frame(window, padding=6)
        bar.pack(fill="x")
        frame = ttk.Frame(window)
        frame.pack(fill="both", expand=True)
        canvas = tk.Canvas(frame, background="#0b2a4a", highlightthickness=0)
        sy = ttk.Scrollbar(frame, orient="vertical", command=canvas.yview)
        sx = ttk.Scrollbar(frame, orient="horizontal", command=canvas.xview)
        canvas.configure(xscrollcommand=sx.set, yscrollcommand=sy.set)
        canvas.grid(row=0, column=0, sticky="nsew")
        sy.grid(row=0, column=1, sticky="ns")
        sx.grid(row=1, column=0, sticky="ew")
        frame.rowconfigure(0, weight=1)
        frame.columnconfigure(0, weight=1)
        canvas.bind("<ButtonPress-1>", lambda e: canvas.scan_mark(e.x, e.y))
        canvas.bind("<B1-Motion>", lambda e: canvas.scan_dragto(e.x, e.y, gain=1))
        base = tk.PhotoImage(file=str(path))
        window.base = base
        info = tk.StringVar()
        ttk.Label(bar, textvariable=info).pack(side="right")
        def draw():
            k = zoom.get()  # 1 = tamaño real, 2 = mitad
            img = base if k == 1 else base.subsample(k, k)
            window.img = img
            canvas.delete("all")
            canvas.create_image(0, 0, image=img, anchor="nw")
            canvas.configure(scrollregion=(0, 0, img.width(), img.height()))
            for c in result["cities"]:
                x, y = c["x"] / k, c["y"] / k
                colour = "#ffd23f" if c["current"] else "#f4f1de"
                r = 6 if c["current"] else 4
                canvas.create_oval(x - r, y - r, x + r, y + r, fill=colour, outline="#222")
                lx, ly = c["label_x"] / k, c["label_y"] / k
                canvas.create_text(lx + 1, ly + 1, text=c["name"], fill="#000", anchor="w" if c["label_right"] else "e", font=("Segoe UI", 9, "bold"))
                canvas.create_text(lx, ly, text=c["name"], fill=colour, anchor="w" if c["label_right"] else "e", font=("Segoe UI", 9, "bold"))
            own = other = 0
            if ships_on.get():
                for s in result["ships"]:
                    x, y = s["x"] / k, s["y"] / k
                    if s["own"]:
                        own += 1
                        canvas.create_polygon(x, y - 6, x + 5, y + 4, x - 5, y + 4, fill="#ff4d4d", outline="#fff")
                    else:
                        other += 1
                        canvas.create_rectangle(x - 2, y - 2, x + 2, y + 2, fill="#9bd1ff" if s["at_sea"] else "#7a8a99", outline="")
            info.set(f"{len(result['cities'])} ciudades · barcos propios {own} · ajenos {other}")
        ttk.Radiobutton(bar, text="Mitad", variable=zoom, value=2, command=draw).pack(side="left")
        ttk.Radiobutton(bar, text="Tamaño real", variable=zoom, value=1, command=draw).pack(side="left", padx=6)
        ttk.Checkbutton(bar, text="Barcos", variable=ships_on, command=draw).pack(side="left", padx=6)
        draw()

    def show_navmap(self):
        """Carta náutica desplazable (`navmap` → `nav_map_update` 0x0044DCE0 portado con paridad x86).

        Flechas del teclado o ratón en el borde para desplazar (paso de 1500 ms por pantalla, como el original), doble clic para
        centrar, arrastre para mover la vista. Con «Avanzar el tiempo» cada fotograma avanza unos ticks y los barcos se mueven
        (necesita «Activar flota IA» para que naveguen los barcos que no gestiona la sesión). El motor dibuja fondo, tormentas,
        calmas, barcos, flechas, símbolos de ciudad (oficinas y ciudad natal), iconos de noticias, barcos en puerto y la ruta de
        los barcos elegidos; los nombres de las ciudades se escriben con la fuente del juego (`fonts.ini`); aquí se reproducen los sonidos del mapa. Clic en
        un barco propio para elegirlo (Mayús añade), Escape para soltar la selección."""
        if getattr(self, "navmap_window", None) is not None and self.navmap_window.winfo_exists():
            self.navmap_window.lift()
            return
        window = tk.Toplevel(self.root)
        self.navmap_window = window
        window.title("Carta náutica")
        window.geometry("1024x720")
        bar = ttk.Frame(window, padding=6)
        bar.pack(fill="x")
        advancing = tk.BooleanVar(value=False)
        speed = tk.IntVar(value=2)
        sound = tk.BooleanVar(value=True)
        names = tk.BooleanVar(value=False)
        info = tk.StringVar(value="Cargando la carta…")
        ttk.Checkbutton(bar, text="Avanzar el tiempo", variable=advancing).pack(side="left")
        ttk.Label(bar, text="ticks/imagen").pack(side="left", padx=(8, 2))
        ttk.Spinbox(bar, from_=1, to=32, width=4, textvariable=speed).pack(side="left")
        ttk.Checkbutton(bar, text="Sonido", variable=sound).pack(side="left", padx=8)
        ttk.Checkbutton(bar, text="Nombres (fuente del sistema)", variable=names).pack(side="left")
        ttk.Label(bar, textvariable=info).pack(side="right")
        canvas = tk.Canvas(window, background="#0b2a4a", highlightthickness=0, takefocus=1)
        canvas.pack(fill="both", expand=True)
        state = {"keys": set(), "mouse": None, "center": None, "drag": None, "scroll": (0, 0), "start": time.monotonic(),
                 "selection": [], "ships": []}
        arrows = {"Left": "left", "Up": "up", "Right": "right", "Down": "down"}
        def key(event, down):
            if event.keysym in arrows:
                (state["keys"].add if down else state["keys"].discard)(arrows[event.keysym])
        window.bind("<KeyPress>", lambda e: key(e, True))
        window.bind("<KeyRelease>", lambda e: key(e, False))
        canvas.bind("<Motion>", lambda e: state.update(mouse=(e.x, e.y)))
        canvas.bind("<Leave>", lambda e: state.update(mouse=None))
        canvas.bind("<Double-Button-1>", lambda e: state.update(center=(state["scroll"][0] + e.x, state["scroll"][1] + e.y)))
        def press(e):
            canvas.focus_set()
            state["drag"] = (e.x, e.y, state["scroll"])
        def drag(e):
            if state["drag"]:
                x0, y0, (sx, sy) = state["drag"]
                w, h = canvas.winfo_width(), canvas.winfo_height()
                state["center"] = (sx - (e.x - x0) + w // 2, sy - (e.y - y0) + h // 2)
        canvas.bind("<ButtonPress-1>", press)
        canvas.bind("<B1-Motion>", drag)
        def release(e):
            start = state["drag"]
            state["drag"] = None
            if not start or abs(e.x - start[0]) > 3 or abs(e.y - start[1]) > 3:
                return
            hit = [s for s in state["ships"] if s["own"] and abs(s["x"] - e.x) <= 14 and abs(s["y"] - e.y) <= 14]
            if not hit:
                return
            ship = min(hit, key=lambda s: (s["x"] - e.x) ** 2 + (s["y"] - e.y) ** 2)["id"]
            if e.state & 0x1:
                if ship not in state["selection"]:
                    state["selection"].append(ship)
            else:
                state["selection"] = [ship]
        canvas.bind("<ButtonRelease-1>", release)
        window.bind("<Escape>", lambda e: state.update(selection=[]))
        def frame():
            if not window.winfo_exists():
                return
            if self.busy or not self.process:
                window.after(100, frame)
                return
            w, h = max(64, canvas.winfo_width()), max(64, canvas.winfo_height())
            req = {"command": "navmap", "width": w, "height": h, "clock_ms": int((time.monotonic() - state["start"]) * 1000),
                   "keys": sorted(state["keys"]), "ticks": speed.get() if advancing.get() else 0,
                   "selection": list(state["selection"])}
            if state["mouse"] and not state["drag"]:
                req["mouse"] = list(state["mouse"])
            if state["center"]:
                req["center"] = list(state["center"])
                state["center"] = None
            self.request(req, ready)
        def ready(result):
            if not window.winfo_exists():
                return
            image = tk.PhotoImage(file=str(Path(self.image_path).with_name("carta.ppm")))
            window.image = image
            state["scroll"] = tuple(result["scroll"])
            state["ships"] = result["ships"]
            canvas.delete("all")
            canvas.create_image(0, 0, image=image, anchor="nw")
            for s in result["ships"]:
                if s["id"] in state["selection"]:
                    canvas.create_oval(s["x"] - 13, s["y"] - 13, s["x"] + 13, s["y"] + 13, outline="#ffe288")
            if names.get():
                for c in result["cities"]:
                    anchor = "w" if c["label_right"] else "e"
                    canvas.create_text(c["label_x"] + 1, c["label_y"] + 1, text=c["name"], fill="#000", anchor=anchor, font=("Segoe UI", 9, "bold"))
                    canvas.create_text(c["label_x"], c["label_y"], text=c["name"], fill="#f4f1de", anchor=anchor, font=("Segoe UI", 9, "bold"))
            if sound.get():
                self.apply_sounds(result)
            own = sum(1 for s in result["ships"] if s["own"])
            day = result["day"]
            info.set(f"Día {day} · tick {result['tick'] % 256}/256 · vista {result['scroll'][0]},{result['scroll'][1]} · "
                     f"barcos a la vista {len(result['ships'])} (propios {own}, elegidos {len(state['selection'])})" + (" · " + result["errors"][0] if result["errors"] else ""))
            window.after(60, frame)
        canvas.focus_set()
        window.after(50, frame)

    def show_land_transport(self):
        """Ventana del almacén para enviar carros a la ciudad unida por tierra (`0x005EB3C0` y operación 0x3A portados con paridad).

        Se eligen cargas enteras por mercancía (2.000 unidades a granel o 200 en barriles) y la escolta; el presupuesto se recalcula
        como el original (carros, valor, escolta exigida y su coste). Con menos escolta de la exigida puede haber emboscada y perder
        parte de la carga. Los carros salen en la vista de la ciudad y llegan en unos días."""
        window = tk.Toplevel(self.root)
        window.title("Transporte por tierra")
        window.geometry("560x640")
        head = tk.StringVar()
        ttk.Label(window, textvariable=head, padding=6, font=("Segoe UI", 11, "bold")).pack(anchor="w")
        grid = ttk.Frame(window, padding=6)
        grid.pack(fill="x")
        loads = [tk.IntVar(value=0) for _ in range(20)]
        avail = [tk.StringVar() for _ in range(20)]
        for i in range(20):
            r, c = i % 10, (i // 10) * 3
            ttk.Label(grid, text=WARES[i]).grid(row=r, column=c, sticky="w")
            ttk.Spinbox(grid, from_=0, to=255, textvariable=loads[i], width=4, command=lambda: quote()).grid(row=r, column=c + 1, padx=4)
            ttk.Label(grid, textvariable=avail[i], width=8).grid(row=r, column=c + 2, sticky="w")
        escort = tk.IntVar(value=0)
        row = ttk.Frame(window, padding=6)
        row.pack(fill="x")
        ttk.Label(row, text="Escolta").pack(side="left")
        radios = []
        for k, name in enumerate(["Ninguna", "Pequeña", "Grande"]):
            b = ttk.Radiobutton(row, text=name, variable=escort, value=k, command=lambda: quote())
            b.pack(side="left", padx=4)
            radios.append((b, name))
        summary = tk.StringVar()
        ttk.Label(window, textvariable=summary, padding=6, wraplength=530).pack(anchor="w")
        actions = ttk.Frame(window, padding=6)
        actions.pack(fill="x")
        listing = tk.Listbox(window, height=8)
        listing.pack(fill="both", expand=True, padx=6, pady=6)
        PHASES = {0: "cargando", 1: "saliendo", 2: "en camino", 3: "en camino", 4: "en camino", 5: "en camino", 6: "llegando"}

        def current():
            out = []
            for v in loads:
                try:
                    out.append(max(0, min(255, int(v.get()))))
                except (tk.TclError, ValueError):
                    out.append(0)
            return out

        def quote():
            if not window.winfo_exists():
                return
            self.request({"command": "land_transport_quote", "loads": current(), "escort": escort.get()}, quoted)

        def quoted(q):
            if not window.winfo_exists():
                return
            if q["destination"] is None:
                head.set(f"{q.get('origin_name') or self.city_name(q['origin'])}: sin ciudad unida por tierra")
                summary.set("Esta ciudad no tiene camino por tierra.")
                return
            head.set(f"{q['origin_name']} → {q['destination_name']}")
            for i in range(20):
                avail[i].set(f"/ {q['available'][i]}")
                if q["loads"][i] != current()[i]:
                    loads[i].set(q["loads"][i])
            for k, (b, name) in enumerate(radios):
                b.configure(text=f"{name} ({q['costs'][k]:,})" + (" · exigida" if k == q["required"] else ""))
            summary.set(f"Carros {q['wagons']} · valor {q['value']:,} · escolta exigida {['ninguna', 'pequeña', 'grande'][q['required']]} · "
                        f"coste {q['cost']:,} · oro {q['gold']:,} · plazas {q['slots']}/{q['max_slots']}"
                        + ("" if q["can_send"] else " · no se puede enviar"))
            send_button.configure(state="normal" if q["can_send"] else "disabled")

        def send():
            self.request({"command": "land_transport_send", "loads": current(), "escort": escort.get()}, sent)

        def sent(result):
            self.update_state(result["state"])
            self.dirty = True
            lost = sum(result["lost"])
            text = f"Enviados {len(result['tasks'])} transportes a {self.city_name(result['destination'])}."
            if result["ambush"]:
                text += f" ¡Emboscada! Cargas perdidas: {lost}."
            self.message.set(text)
            if window.winfo_exists():
                for v in loads:
                    v.set(0)
                refresh()

        def refresh():
            self.request({"command": "land_transports"}, listed)

        def listed(items):
            if not window.winfo_exists():
                return
            listing.delete(0, "end")
            for t in items:
                listing.insert("end", f"{self.city_name(t['origin'])} → {self.city_name(t['destination'])}: {t['loads']} cargas de {WARES[t['good']]}, "
                                      f"{PHASES.get(t['phase'], t['phase'])} {t['progress'] * 100 // 255} %")
            if not items:
                listing.insert("end", "Ningún transporte por tierra en curso.")
            quote()

        send_button = ttk.Button(actions, text="Enviar", command=send, state="disabled")
        send_button.pack(side="left", padx=4)
        ttk.Button(actions, text="Actualizar", command=refresh).pack(side="left", padx=4)
        refresh()

    def show_ships(self):
        ships = list(self.ships)
        window = tk.Toplevel(self.root)
        window.title("Barcos del comerciante")
        window.geometry("620x430")
        ttk.Label(window, text="Barcos propios. Selecciona un líder para mover su convoy o un subordinado para separarlo.", padding=8).pack(anchor="w")
        listing = tk.Listbox(window, height=10, exportselection=False)
        listing.pack(fill="x", padx=8)
        # Las acciones se reservan al pie antes que el texto, que se encoge si falta sitio (antes quedaban fuera, WIN-02).
        actions = ttk.Frame(window, padding=(8, 0, 8, 8))
        actions.pack(side="bottom", fill="x")
        cargo = tk.Text(window, height=10, state="disabled")
        cargo.pack(fill="both", expand=True, padx=8, pady=8)
        ttk.Label(actions, text="Usa la mercancía y cantidad elegidas en la oficina.").pack(side="left")
        for ship in ships:
            listing.insert("end", f"Barco {ship['id']}  ·  estado {ship['state']}  ·  ciudad {ship['town']}  ·  posición {ship['x']}, {ship['y']}")
        def show(_event):
            selection = listing.curselection()
            if not selection:
                return
            ship = ships[selection[0]]
            self.map_sound(1, ship)
            cargo.configure(state="normal")
            cargo.delete("1.0", "end")
            cargo.insert("end", f"Capacidad: {ship['capacity']}\n\n")
            for i, amount in enumerate(ship["cargo"]):
                if amount:
                    cargo.insert("end", f"{WARES[i]}: {amount}\n")
            cargo.configure(state="disabled")
        listing.bind("<<ListboxSelect>>", show)
        def trade(side):
            selected = listing.curselection()
            if not selected:
                return
            try:
                amount, ware = int(self.amount.get()), int(self.ware.get())
            except ValueError:
                messagebox.showerror("Comercio", "La cantidad debe ser un número entero.", parent=window)
                return
            def completed(result):
                self.traded(result)
                ships[:] = self.ships
                if not window.winfo_exists():
                    return
                listing.delete(0, "end")
                for ship in ships:
                    listing.insert("end", f"Barco {ship['id']}  ·  estado {ship['state']}  ·  ciudad {ship['town']}  ·  posición {ship['x']}, {ship['y']}")
                listing.selection_set(min(selected[0], len(ships)-1))
                show(None)
            self.request({"command": "ship_trade", "ship": ships[selected[0]]["id"], "side": side, "ware": ware, "quantity": amount}, completed)
        ttk.Button(actions, text="Comprar", command=lambda: trade("buy")).pack(side="left", padx=4)
        ttk.Button(actions, text="Vender", command=lambda: trade("sell")).pack(side="left", padx=4)
        route_row = ttk.Frame(window, padding=(8, 0, 8, 8))
        route_row.pack(side="bottom", fill="x", before=cargo)
        destinations = [d for d in self.destinations if d["city"] != self.city]
        choice = ttk.Combobox(route_row, state="readonly", values=[d.get("name") or f"Ciudad {d['town_id']}" for d in destinations], width=18)
        choice.pack(side="left")
        if destinations:
            choice.current(0)
        def preview():
            index = choice.current()
            if index < 0:
                return
            def ready(result):
                if not window.winfo_exists():
                    return
                self.root.after(50, lambda: self.map_sound(4, ships[selected[0]] if (selected := listing.curselection()) else None))
                view = tk.Toplevel(window)
                view.title("Previsualización de ruta")
                ttk.Label(view, text="Rejilla de navegación del CPR. El barco aún no viaja.", padding=6).pack()
                view.image = tk.PhotoImage(file=str(Path(self.temp.name) / "navegacion.png"))
                map_canvas = tk.Canvas(view, width=view.image.width(), height=view.image.height(), highlightthickness=0)
                map_canvas.pack()
                map_canvas.create_image(0, 0, image=view.image, anchor="nw")
                def cell(value):
                    return int(((value - 68) * 1000 + 1768) / 3536)
                coords = [cell(v) for point in result["points"] for v in point]
                if len(coords) >= 4:
                    map_canvas.create_line(*coords, fill="#ffdc74", width=2)
                    for x, y in zip(coords[::2], coords[1::2]):
                        map_canvas.create_oval(x-2, y-2, x+2, y+2, fill="#fff0a0", outline="")
            self.request({"command": "route_preview", "destination": destinations[index]["city"]}, ready)
        ttk.Button(route_row, text="Previsualizar ruta", command=preview).pack(side="left", padx=6)
        def sail():
            selected, destination = listing.curselection(), choice.current()
            if not selected or destination < 0:
                return
            def started(state):
                self.update_state(state)
                self.dirty = True
                self.message.set("Orden de viaje iniciada; avanza el tiempo para que zarpe.")
                if window.winfo_exists():
                    window.destroy()
            self.request({"command": "sail", "ship": ships[selected[0]]["id"], "destination": destinations[destination]["city"]}, started)
        ttk.Button(route_row, text="Enviar barco", command=sail).pack(side="left", padx=6)
        if ships:
            listing.selection_set(0)
            show(None)

    def draw_buildings(self):
        self.canvas.delete("building")
        if not self.image or not self.show_buildings.get():
            return
        ox, oy = self.map_origin
        for building in self.buildings:
            x, y = building["x"] + ox, building["y"] + oy
            px = 34 * x + (34 if y % 2 else 17)
            py = 22 * (y // 2) + (43 if y % 2 else 32)
            color = "#f3cc62" if building["owner"] == 255 else "#e47c51"
            marker = self.canvas.create_oval(px-5, py-5, px+5, py+5, fill=color, outline="#252116", tags="building")
            self.canvas.tag_bind(marker, "<ButtonRelease-1>", lambda _event, b=building: self.message.set(f"Edificio {b['record']}: {TYPE_NAMES.get(b['kind'], 'tipo ' + str(b['kind']))} (tipo {b['kind']}), propietario {b['owner']}, obra pendiente {b['work']}"))

    def select_ware(self, _event):
        chosen = self.goods.curselection()
        if chosen:
            self.ware.set(str(chosen[0]))

    def toggle_sound(self):
        if not self.sound_on.get():
            winsound.PlaySound(None, 0)
        elif self.busy:
            self.sound_on.set(False)
            self.message.set("Espera a que termine la operación antes de activar el ambiente.")
        else:
            self.request({"command": "ambient"}, self.play_ambient)

    def play_ambient(self, _result):
        if self.sound_on.get():
            try:
                winsound.PlaySound(str(Path(self.temp.name) / "ambiente.wav"), winsound.SND_FILENAME | winsound.SND_ASYNC | winsound.SND_LOOP)
            except RuntimeError as exc:
                self.sound_on.set(False)
                messagebox.showerror("Sonido", str(exc))

    def trade(self, side):
        try:
            amount = int(self.amount.get())
            ware = int(self.ware.get())
        except ValueError:
            messagebox.showerror("Comercio", "La cantidad debe ser un número entero.")
            return
        self.request({"command": "trade", "side": side, "ware": ware, "quantity": amount}, self.traded)

    def traded(self, result):
        self.update_state(result["state"])
        self.dirty = self.dirty or result["quantity"] != 0
        self.message.set(f"Operación: {result['quantity']} unidades; variación de oro {result['gold_change']:+,}.")

    def enable_fleet(self):
        if self.battle_paused:
            self.message.set("La batalla está en pausa.")
            return
        self.request({"command": "enable_full_fleet"}, self.fleet_enabled)

    def fleet_enabled(self, result):
        self.full_fleet = True
        self.update_state(result["state"])
        self.message.set(f"Flota de IA activada: {result['ships_resumed']} barcos y líderes reanudados.")

    def advance(self):
        if self.battle_paused:
            self.running = False
            self.message.set("La batalla está en pausa; puedes guardar la partida.")
        elif self.full_fleet:
            self.request({"command": "advance_until_encounter", "target": min(self.tick + 256, 0xffffffff), "dialog_bytes": []}, self.fleet_advanced)
        else:
            self.request({"command": "advance"}, self.advanced)

    def fleet_advanced(self, result):
        changed = self.update_state(result["state"])
        self.dirty = True
        if result["encounter"] is not None:
            self.running = False
            self.message.set("Encuentro naval: completando el tick.")
            self.request({"command": "complete_encounter_tick"}, self.fleet_encounter_ready)
        else:
            self.message.set("Día avanzado con flota de IA.")
            if changed:
                self.request({"command": "render"}, self.rendered)
            if self.running:
                self.timer = self.root.after(1000, self.auto_advance)

    def fleet_encounter_ready(self, result):
        self.update_state(result["state"])
        self.message.set("Batalla en pausa. Puedes guardarla o avanzar el combate sin vista.")

    def advance_battle(self):
        if self.running:
            self.message.set("Pausa el tiempo antes de avanzar un paso manual.")
        elif not self.battle_paused:
            self.message.set("No hay una batalla activa.")
        elif self.battle_ready:
            self.request({"command": "advance_battle", "ticks": 16}, self.battle_advanced)
        else:
            self.request({"command": "activate_encounter_battle"}, self.battle_activated)

    def battle_activated(self, result):
        self.update_state(result["state"])
        if self.running:
            self.battle_pump()
        else:
            self.request({"command": "advance_battle", "ticks": 16}, self.battle_advanced)

    def battle_advanced(self, result):
        self.update_state(result["state"])
        self.dirty = True
        self.message.set("Combate avanzado sin vista. Puedes continuar o guardar.")

    def battle_pump(self):
        now = (time.monotonic_ns() // 1_000_000) & 0xffffffff
        self.request({"command": "pump_battle_time", "now_ms": now, "mode": self.battle_mode}, self.battle_pumped)

    def battle_pumped(self, result):
        self.update_state(result["state"])
        if result["time_request"] is not None:
            self.battle_mode = result["time_request"]
        self.dirty = self.dirty or result["battle_advanced"] != 0
        self.message.set("Combate en marcha sin vista.")
        if self.running:
            self.timer = self.root.after(100, self.auto_advance)

    def battle_drained(self, result):
        self.update_state(result["state"])
        if result["time_request"] is not None:
            self.battle_mode = result["time_request"]
        self.dirty = self.dirty or result["battle_advanced"] != 0
        self.message.set("Combate en pausa. Puedes guardar.")

    def advanced(self, result):
        changed = self.update_state(result["state"])
        self.dirty = True
        if result["errors"]:
            self.running = False
            messagebox.showwarning("Simulación", "\n".join(result["errors"][:5]))
        self.message.set("Día avanzado. Algunos sistemas siguen inactivos; esta sesión es experimental.")
        if changed:
            self.request({"command": "render"}, self.rendered)
        if self.running:
            if self.timer is not None:
                self.root.after_cancel(self.timer)
            self.timer = self.root.after(1000, self.auto_advance)

    def auto_advance(self):
        self.timer = None
        if self.running and not self.busy:
            self.battle_pump() if self.battle_paused and self.battle_ready else self.advance()
        elif self.running:
            self.timer = self.root.after(100, self.auto_advance)

    def toggle(self):
        if self.busy:
            self.message.set("Espera a que termine el paso actual.")
            return
        if self.battle_paused:
            self.running = not self.running
            if self.timer is not None:
                self.root.after_cancel(self.timer)
                self.timer = None
            if self.running:
                if self.battle_ready:
                    self.battle_pump()
                else:
                    self.request({"command": "activate_encounter_battle"}, self.battle_activated)
            elif self.battle_ready:
                self.request({"command": "drain_battle_time"}, self.battle_drained)
            else:
                self.message.set("Batalla en pausa.")
            return
        self.running = not self.running
        if not self.running and self.timer is not None:
            self.root.after_cancel(self.timer)
            self.timer = None
        self.message.set("Tiempo en marcha" if self.running else "Tiempo en pausa")
        if self.running and not self.busy:
            self.advance()
        elif self.running and self.timer is None:
            self.timer = self.root.after(100, self.auto_advance)

    def save(self):
        if self.busy:
            self.message.set("Espera a que termine el paso actual.")
            return
        self.running = False
        if self.timer is not None:
            self.root.after_cancel(self.timer)
            self.timer = None
        path = filedialog.asksaveasfilename(title="Guardar una copia nueva", defaultextension=".pat", filetypes=[("Partidas", "*.pat")], confirmoverwrite=False)
        if path:
            if Path(path).exists():
                messagebox.showerror("Guardar", "Elige un nombre nuevo: el prototipo no sobrescribe partidas existentes.")
                return
            if self.battle_ready:
                def drained(result):
                    self.battle_drained(result)
                    self.request({"command": "save", "path": path}, lambda saved: self.saved(path, saved))
                self.request({"command": "drain_battle_time"}, drained)
            else:
                self.request({"command": "save", "path": path}, lambda result: self.saved(path, result))

    def saved(self, path, result):
        self.dirty = False
        self.message.set(f"Copia guardada: {path} ({result['bytes']:,} bytes).")

    def request(self, command, callback):
        if not self.process or self.busy:
            return
        self.busy = True
        process = self.process
        def worker():
            try:
                process.stdin.write(json.dumps(command, ensure_ascii=False) + "\n")
                process.stdin.flush()
                line = process.stdout.readline()
                if not line:
                    raise RuntimeError(process.stderr.read().strip() or "El proceso de simulación terminó")
                self.responses.put((callback, json.loads(line), None))
            except (OSError, ValueError, RuntimeError) as exc:
                self.responses.put((callback, None, str(exc)))
        threading.Thread(target=worker, daemon=True).start()

    def poll(self):
        try:
            while True:
                callback, result, error = self.responses.get_nowait()
                self.busy = False
                if error or not result["ok"]:
                    self.running = False
                    if callback == self.play_ambient:
                        self.sound_on.set(False)
                    messagebox.showerror("Sesión", error or result["error"])
                else:
                    callback(result["data"])
        except queue.Empty:
            pass
        self.root.after(100, self.poll)

    def stop_process(self):
        self.running = False
        if self.sound_on.get():
            winsound.PlaySound(None, 0)
            self.sound_on.set(False)
        if self.timer is not None:
            self.root.after_cancel(self.timer)
            self.timer = None
        if self.process:
            self.process.stdin.close()
            try:
                self.process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self.process.terminate()
                self.process.wait(timeout=3)
            self.process.stdout.close()
            self.process.stderr.close()
            self.process = None

    def close(self):
        if self.busy:
            self.message.set("Espera a que termine la operación en curso.")
            return
        if self.dirty and not messagebox.askyesno("Cambios sin guardar", "¿Cerrar y descartar los cambios de esta sesión?"):
            return
        self.stop_process()
        self.root.destroy()
        self.temp.cleanup()


if __name__ == "__main__":
    if len(sys.argv) not in (2, 4) or not os.path.isfile(sys.argv[1]):
        raise SystemExit("Uso: client.py RUTA_P3_MVP_EXE [PARTIDA CPR]")
    client = Client(sys.argv[1])
    if len(sys.argv) == 4:
        client.save_path.set(sys.argv[2])
        client.cpr_path.set(sys.argv[3])
        client.root.after(100, client.open)
    client.root.mainloop()

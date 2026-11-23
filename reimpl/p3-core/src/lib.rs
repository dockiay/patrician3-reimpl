//! Núcleo de simulación de Patrician III.
//!
//! Reimplementación con paridad exacta del ejecutable inglés (SHA-256 `fe436dc5…`). Cada función
//! documenta la dirección original que reproduce. El núcleo no hace E/S ni usa Win32: los demás
//! sistemas (ciudades, comerciantes, barcos…) se conectan mediante rasgos.

pub mod ai;
pub mod buildings;
pub mod dialogs;
pub mod calendar;
pub mod economy;
pub mod flatmem;
pub mod frame;
pub mod lifted;
pub mod mission_format;
pub mod mission_letter;
pub mod mission_script;
pub mod mission_script_calls;
pub mod mission_script_flat;
pub mod mission_treasure;
pub mod numeric;
pub mod pump;
pub mod rng;
pub mod route;
pub mod scheduler;
pub mod ships;
pub mod shipyard;
pub mod tick;
pub mod x86rt;
pub mod ui_hud;
pub mod ui_dtors;
pub mod ui_input;
pub mod town;

#[derive(Clone, Copy, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Aubergine,
}

#[derive(Clone, Copy)]
pub enum LogLevel {
    Info,
    Ok,
    Warn,
    Err,
}

pub struct LogLine {
    pub ts: &'static str,
    pub level: LogLevel,
    pub msg: &'static str,
}

pub struct TreeEntry {
    pub indent: u8,
    pub is_dir: bool,
    pub is_open: bool,
    pub name: &'static str,
    pub meta: &'static str,
    pub selected: bool,
    pub checked: bool,
    pub cursor: bool,
}

pub struct App {
    pub theme: Theme,
    pub tick: u64,
    pub cursor_visible: bool,
    pub log_head: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            theme: Theme::Dark,
            tick: 0,
            cursor_visible: true,
            log_head: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        if self.tick % 10 == 0 {
            self.cursor_visible = !self.cursor_visible;
        }
        if self.tick % 48 == 0 {
            self.log_head = (self.log_head + 1) % LOG_POOL.len();
        }
    }

    pub fn next_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Aubergine,
            Theme::Aubergine => Theme::Dark,
        };
    }
}

pub static TREE: &[TreeEntry] = &[
    TreeEntry { indent: 0, is_dir: true,  is_open: true,  name: "datos/",          meta: "",                  selected: false, checked: false, cursor: false },
    TreeEntry { indent: 1, is_dir: true,  is_open: false, name: "catastro/",        meta: "12 archivos",       selected: false, checked: false, cursor: false },
    TreeEntry { indent: 1, is_dir: true,  is_open: true,  name: "rol_cobro/",       meta: "3 arch · 8.4 MB",  selected: true,  checked: false, cursor: false },
    TreeEntry { indent: 2, is_dir: false, is_open: false, name: "rol_2024_s1.csv",  meta: "3.8 MB",            selected: false, checked: false, cursor: false },
    TreeEntry { indent: 2, is_dir: false, is_open: false, name: "rol_2024_s2.csv",  meta: "4.2 MB",            selected: false, checked: true,  cursor: false },
    TreeEntry { indent: 2, is_dir: false, is_open: false, name: "rol_2025_s1.csv",  meta: "412 KB",            selected: false, checked: false, cursor: true  },
    TreeEntry { indent: 1, is_dir: true,  is_open: false, name: "exenciones/",      meta: "5 archivos",        selected: false, checked: false, cursor: false },
    TreeEntry { indent: 1, is_dir: true,  is_open: false, name: "predios/",         meta: "28 archivos",       selected: false, checked: false, cursor: false },
    TreeEntry { indent: 0, is_dir: true,  is_open: false, name: "configs/",         meta: "",                  selected: false, checked: false, cursor: false },
];

pub static INITIAL_LOGS: &[LogLine] = &[
    LogLine { ts: "12:04:01", level: LogLevel::Info, msg: "conectado a mysql://etl_user@localhost:3306/contribuciones" },
    LogLine { ts: "12:04:01", level: LogLevel::Ok,   msg: "inferencia de schema · 18 columnas (varchar 11, decimal 4, date 2, int 1)" },
    LogLine { ts: "12:04:02", level: LogLevel::Warn, msg: "3 advertencias en columna 7 · valores vacíos coercidos a NULL" },
    LogLine { ts: "12:04:02", level: LogLevel::Ok,   msg: "validación · 0 errores · 3 warnings" },
    LogLine { ts: "12:04:03", level: LogLevel::Info, msg: "batch 1/13 insertado (1 000 filas · 184 ms)" },
    LogLine { ts: "12:04:04", level: LogLevel::Info, msg: "batch 5/13 insertado (5 000 filas · 901 ms)" },
    LogLine { ts: "12:04:05", level: LogLevel::Info, msg: "batch 8/13 insertado (8 000 filas · 1.42 s)" },
    LogLine { ts: "12:04:06", level: LogLevel::Ok,   msg: "checkpoint guardado · proceso_id=42  status=RUNNING" },
];

pub static LOG_POOL: &[LogLine] = &[
    LogLine { ts: "12:04:07", level: LogLevel::Info, msg: "batch 9/13 insertado (9 000 filas · 1.61 s)" },
    LogLine { ts: "12:04:08", level: LogLevel::Info, msg: "batch 10/13 insertado (10 000 filas · 1.79 s)" },
    LogLine { ts: "12:04:09", level: LogLevel::Warn, msg: "duplicado ignorado · pk 2024-04421 ya existe" },
    LogLine { ts: "12:04:10", level: LogLevel::Info, msg: "batch 11/13 insertado (11 000 filas · 1.92 s)" },
    LogLine { ts: "12:04:11", level: LogLevel::Info, msg: "batch 12/13 insertado (12 000 filas · 2.04 s)" },
    LogLine { ts: "12:04:12", level: LogLevel::Ok,   msg: "batch final 13/13 · 12 500 filas insertadas" },
    LogLine { ts: "12:04:12", level: LogLevel::Ok,   msg: "verificando integridad referencial (3 fk)" },
    LogLine { ts: "12:04:13", level: LogLevel::Ok,   msg: "COMMIT · proceso_id=42  status=DONE" },
    LogLine { ts: "12:04:14", level: LogLevel::Err,  msg: "conexión perdida · reintentando en 3 s…" },
];

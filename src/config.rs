use cruct::{FromConfigValue, ParserError, cruct};
use lazy_static::lazy_static;

#[cruct(load_config(path = "config.toml", format = "toml"))]
#[derive(Debug, Clone)]
pub struct AppConfig {
    #[field(default = "logs".into())]
    pub logs_dir: String,

    #[field(default = 1000.0)]
    pub tick_rate: f64,

    #[field(default = 60.0)]
    pub frame_rate: f64,

    pub key_bindings: KeyBindings,
}

#[cruct(load_config(path = "keybindings.toml", format = "toml"))]
#[derive(Debug, Clone)]
pub struct KeyBindings {
    #[field(default = "q".into())]
    pub quit: String,

    #[field(default = "?".into())]
    pub toggle_help: String,

    #[field(default = "s".into())]
    pub toggle_settings: String,

    #[field(default = "r".into())]
    pub toggle_search: String,

    #[field(default = "f".into())]
    pub toggle_sort: String,

    #[field(default = "t".into())]
    pub toggle_filter: String,
}

impl FromConfigValue for KeyBindings {
    fn from_config_value(value: &cruct::ConfigValue) -> Result<Self, ParserError> {
        KeyBindings::load_from(value)
    }
}

lazy_static! {
    pub static ref PROJECT_NAME: String = env!("CARGO_CRATE_NAME")
        .to_uppercase()
        .to_string();
}

pub fn init() -> Result<AppConfig, cruct::ParserError> {
    AppConfig::loader()
        .with_config()
        .load()
}

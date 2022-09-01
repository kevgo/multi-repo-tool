use crate::config::Config;

pub fn all(config: Config) -> Config {
    Config {
        folders: None,
        ..config
    }
}

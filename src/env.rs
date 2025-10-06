use std::path::PathBuf;

static DATA_DIR_ENV_VAR_KEY: &str = "KENNEL_CLUB_DATA_DIR";
static DATA_DIR_DEFAULT: &str = "./data";

pub fn data_dir() -> PathBuf {
    std::env::var(DATA_DIR_ENV_VAR_KEY)
        .unwrap_or(DATA_DIR_DEFAULT.to_string())
        .into()
}

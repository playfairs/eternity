use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub questions_dir: PathBuf,
    pub logo_path: PathBuf,
}

impl AppConfig {
    pub fn local() -> Self {
        let data_dir = PathBuf::from("data");
        Self {
            db_path: data_dir.join("eternity.db"),
            data_dir,
            questions_dir: PathBuf::from(".resources/questions"),
            logo_path: PathBuf::from(".resources/ascii/logo.txt"),
        }
    }
}

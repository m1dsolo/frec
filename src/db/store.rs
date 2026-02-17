use crate::db::models::Database;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::RwLock;

pub struct Store {
    db_path: PathBuf,
    db: RwLock<Database>,
}

impl Store {
    pub fn new() -> io::Result<Self> {
        let db_path = Self::get_db_path()?;
        let db = Self::load_from_file(&db_path)?;
        Ok(Self {
            db_path,
            db: RwLock::new(db),
        })
    }

    fn get_db_path() -> io::Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "data directory not found"))?;
        let app_dir = data_dir.join("frec");
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)?;
        }
        Ok(app_dir.join("db.json"))
    }

    fn load_from_file(path: &PathBuf) -> io::Result<Database> {
        if !path.exists() {
            return Ok(Database::default());
        }
        let content = fs::read_to_string(path)?;
        serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn save(&self) -> io::Result<()> {
        let db = self.db.read().unwrap();
        let content = serde_json::to_string_pretty(&*db)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(&self.db_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn get_db(&self) -> std::sync::RwLockReadGuard<'_, Database> {
        self.db.read().unwrap()
    }

    pub fn get_db_mut(&self) -> std::sync::RwLockWriteGuard<'_, Database> {
        self.db.write().unwrap()
    }
}

use crate::models::AppDatabase;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: path.into(),
        }
    }

    pub fn load(&self) -> anyhow::Result<AppDatabase> {
        if !self.file_path.exists() {
            return Ok(AppDatabase::default());
        }

        let content = fs::read_to_string(&self.file_path)?;
        let db: AppDatabase = serde_json::from_str(&content)?;
        Ok(db)
    }

    pub fn save(&self, db: &AppDatabase) -> anyhow::Result<()> {
        if let Some(parent) = self.file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = serde_json::to_string_pretty(db)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

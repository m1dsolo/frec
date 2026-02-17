use crate::algorithm::update_entry;
use crate::db::Store;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn add(store: &Store, table: &str, path: &str) -> Result<(), String> {
    let path = normalize_path(path)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    {
        let mut db = store.get_db_mut();
        let table = db.get_or_create_table(table);
        let entry = table
            .entries
            .entry(path.clone())
            .or_insert_with(|| crate::db::Entry::new());
        update_entry(entry, now);
    }

    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

fn normalize_path(path: &str) -> Result<String, String> {
    let path = if path.starts_with('~') {
        let home = dirs::home_dir().ok_or("cannot find home directory")?;
        path.replacen('~', home.to_str().unwrap_or(""), 1)
    } else {
        path.to_string()
    };

    let p = std::path::Path::new(&path);
    if p.is_absolute() {
        return Ok(p.to_string_lossy().to_string());
    }

    // Convert relative path to absolute
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let absolute = cwd.join(p);
    Ok(absolute.to_string_lossy().to_string())
}

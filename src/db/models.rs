use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    #[serde(default)]
    pub tables: HashMap<String, Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    #[serde(default)]
    pub entries: HashMap<String, Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub rank: u32,
    pub last_accessed: u64,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl Database {
    pub fn get_or_create_table(&mut self, name: &str) -> &mut Table {
        self.tables.entry(name.to_string()).or_default()
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }
}

impl Entry {
    pub fn new() -> Self {
        Self {
            rank: 0,
            last_accessed: 0,
        }
    }
}

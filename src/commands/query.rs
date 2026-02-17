use crate::algorithm::calculate_score;
use crate::db::Store;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct QueryResult {
    pub path: String,
    pub score: f64,
}

pub fn query(
    store: &Store,
    table: &str,
    keyword: Option<&str>,
    show_score: bool,
) -> Result<Vec<QueryResult>, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let db = store.get_db();
    let table = match db.get_table(table) {
        Some(t) => t,
        None => return Ok(vec![]),
    };

    let mut results: Vec<QueryResult> = table
        .entries
        .iter()
        .filter(|(path, _)| {
            if let Some(kw) = keyword {
                path.contains(kw)
            } else {
                true
            }
        })
        .map(|(path, entry)| {
            let score = calculate_score(entry, now);
            QueryResult {
                path: path.clone(),
                score,
            }
        })
        .collect();

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if !show_score {
        results = results
            .into_iter()
            .map(|r| QueryResult {
                path: r.path,
                score: 0.0,
            })
            .collect();
    }

    Ok(results)
}

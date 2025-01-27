use std::{fs, path::PathBuf, time::SystemTime};

pub fn get_latest_bpmn_file(inp_dir: &str) -> Option<PathBuf> {
    fs::read_dir(inp_dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().map_or(false, |ext| ext == "bpmn")
        })
        .max_by_key(|entry| {
            entry
                .metadata()
                .ok()
                .and_then(|meta| meta.modified().ok())
                .unwrap_or_else(|| SystemTime::now())
        })
        .map(|entry| entry.path().clone())
}

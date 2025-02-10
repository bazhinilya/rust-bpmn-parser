use std::{fs, path::PathBuf, time::UNIX_EPOCH};

pub fn get_latest_bpmn_file(inp_dir: &str) -> Option<PathBuf> {
    fs::read_dir(inp_dir)
        .ok()?
        .flatten() 
        .filter(|entry| {
            let path = entry.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "bpmn")
        })
        .max_by_key(|entry| {
            entry
                .metadata()
                .ok()
                .and_then(|meta| meta.modified().ok())
                .unwrap_or(UNIX_EPOCH)
        })
        .map(|entry| entry.path())
}

use std::{fs, io, path::{Path, PathBuf}};

pub fn check_audio_file(path: &PathBuf) -> Result<bool, io::Error> {
    if let Some(t) = infer::get_from_path(path.to_str().unwrap())? {
        let mime_type = t.mime_type();

        return Ok(mime_type.contains("audio"));
    }

    Ok(false)
}

pub fn get_entrys(folder_path: &Path) -> Vec<PathBuf> {
    let mut files_path_vec = Vec::new();
    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            files_path_vec.push(path);
        } else if path.is_dir() {
            files_path_vec.push(path.clone());
        }
    }
    files_path_vec
}

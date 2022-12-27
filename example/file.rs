use std::{ fs, io };

pub fn read_files(path: &str, file_type: &str) -> io::Result<Vec<String>> {
    let mut texts = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension() == Some(file_type.as_ref()) {
            let text = fs::read_to_string(path).unwrap();
            texts.push(text);
        }
    }

    Ok(texts)
}
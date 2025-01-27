use std::io::{self, Read, Write};
use std::path::Path;

pub fn read_file_content(path: &str) -> io::Result<String> {
    let mut content = String::new();
    match std::fs::File::open(path) {
        Ok(mut file) => {
            file.read_to_string(&mut content)?;
            Ok(content)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e),
    }
}

pub fn write_file_content(path: &str, content: &str) -> io::Result<()> {
    std::fs::write(path, content)
}

pub fn create_config_file(
    name: String,
    path: String,
    content: String,
) -> Result<(), std::io::Error> {
    println!(
        "Creating config file with name: {}, path: {}, content: {:?}",
        name.clone(),
        path.clone(),
        content.clone()
    );

    let mut file = match std::fs::File::create(format!("{path}/{name}")) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return Err(e);
        }
    };

    file.write_all(content.as_bytes()).unwrap_or_default();

    Ok(())
}

pub fn update_config_file<F>(name: String, path: String, editor_fn: F) -> io::Result<()>
where
    F: FnOnce(String) -> io::Result<String>,
{
    let file_path = format!("{path}/{name}");
    let path = Path::new(&file_path);

    // Create parent directories if they don't exist
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Read existing content or start with empty string if file doesn't exist
    let current_content = read_file_content(&file_path)?;

    match editor_fn(current_content) {
        Ok(new_content) => write_file_content(&file_path, &new_content),
        Err(e) => Err(e),
    }
}

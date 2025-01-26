use std::io::Write;

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
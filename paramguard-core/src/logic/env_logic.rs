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

pub fn create_env_file(
    name: String,
    path: String,
    env_vars: Option<Vec<String>>,
) -> Result<(), std::io::Error> {
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{path}/{name}"))
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file");
            return Err(e);
        }
    };

    if let Some(vars) = env_vars {
        vars.iter().for_each(|var| {
            let var_with_newline = format!("{var}\n");
            let var_bytes = var_with_newline.as_bytes();
            file.write_all(var_bytes).unwrap_or_default();
        });
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty environment variables",
        ));
    }

    Ok(())
}

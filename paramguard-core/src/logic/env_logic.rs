use std::io::Write;

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

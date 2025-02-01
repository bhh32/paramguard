use crate::config::error::ConfigError;
use std::io::Write;

pub fn create_env_file(
    name: String,
    path: String,
    env_vars: Option<Vec<String>>,
) -> Result<(), ConfigError> {
    let mut file = match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{path}/{name}"))
    {
        Ok(file) => file,
        Err(e) => {
            return Err(ConfigError::PermissionDenied(
                "Error creating file".to_string(),
            ));
        }
    };

    if let Some(vars) = env_vars {
        vars.iter().for_each(|var| {
            let var_with_newline = format!("{var}\n");
            let var_bytes = var_with_newline.as_bytes();
            file.write_all(var_bytes).unwrap_or_default();
        });
    } else {
        return Err(ConfigError::ValidationError(
            "Empty environment variables".to_string(),
        ));
    }

    Ok(())
}

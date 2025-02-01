use crate::args::configargs::ConfigCommands;
use paramguard_core::{
    config::{error::ConfigError, manager::ConfigManager},
    logic::env_logic,
};

pub fn handle_config_command(cmd: &ConfigCommands) -> Result<(), ConfigError> {
    let mut config_mgr = ConfigManager::new();

    match cmd {
        ConfigCommands::Add { name, path } => {
            let path = path.join(name);

            match config_mgr.add_config_file(&path) {
                Ok(_) => println!("{name} is now being tracked by ParamGuard"),
                Err(e) => {
                    eprintln!("Error tracking {name}");
                    return Err(e);
                }
            };
        }
        ConfigCommands::Create {
            name,
            path,
            content,
            env_var,
        } => {
            if let Some(content) = content {
                let cfg_fmt = ConfigManager::detect_format(path)?;

                match config_mgr.create_config_file(
                    name.as_str(),
                    path.as_path(),
                    cfg_fmt,
                    Some(&content),
                ) {
                    Ok(_) => {
                        println!("{name} was created successfully!");
                    }
                    Err(e) => {
                        eprintln!("Error creating config file: {}", e);
                        return Err(e);
                    }
                }
            } else {
                if let Some(env_var) = env_var {
                    // Create the env file
                    match env_logic::create_env_file(
                        name.clone(),
                        String::from(path.to_str().unwrap()),
                        Some(env_var.clone()),
                    ) {
                        Ok(_) => println!("{name} was created successfull!"),
                        Err(e) => {
                            eprintln!("Error creating env file: {}", e);
                            return Err(e);
                        }
                    }
                }
            }
        }
        ConfigCommands::Update { name, path } => match config_mgr.update_config(name, path) {
            Ok(_) => println!("{name} was updated successfully!"),
            Err(e) => {
                eprintln!("Error updating {name}: {e}");
                return Err(e);
            }
        },
    }

    Ok(())
}

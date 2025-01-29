use paramguard_core::archive::{ArchiveInterface, ArchiveService};

pub fn handle_archive_command(cmd: &ArchiveCommands) -> Result<(), Box<dyn std::error::Error>> {
    let archive_service = ArchiveService::new("paramguard.db")?;

    match cmd {
        ArchiveCommands::Store {
            name,
            path,
            retention_days,
            reason,
        } => {
            let id = archive_service.store(name, path, *retention_days, reason.clone())?;
            println!("Archived '{name}' with ID: {id}");
        }
        ArchiveCommands::Restore { id, output_path } => {
            let restored_path = archive_service.restore(*id, output_path.clone())?;
            println!("Restored archive {id} to {}", restored_path.display());
        }
        ArchiveCommands::List => {
            let archives = archive_service.list()?;
            display_archives(&archives);
        }
        ArchiveCommands::Search { query } => {
            let results = archive_service.search(query)?;
            display_archives(&results);
        }
        ArchiveCommands::Cleanup => {
            let count = archive_service.cleanup()?;
            println!("Cleaned up {count} expired archives");
        }
    }

    Ok(())
}

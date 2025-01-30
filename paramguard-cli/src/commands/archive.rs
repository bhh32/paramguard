use crate::args::archiveargs::ArchiveCommands;
use paramguard_core::archive::{
    db::{ArchiveStatistics, RetentionInfo},
    interface::{
        display::{ArchiveDisplayInfo, DefaultFormatter, DisplayFormatter, UiType},
        ArchiveInterface, ArchiveService,
    },
};

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
        ArchiveCommands::List {
            limit: _,
            expired: _,
            detailed,
        } => {
            let archives = archive_service.list()?;
            let display_info: Vec<_> = archives
                .iter()
                .map(|a| {
                    a.to_display_info(UiType::Cli {
                        detailed: *detailed,
                    })
                })
                .collect();
            display_archives(&display_info);
        }
        ArchiveCommands::Search { query, detailed } => {
            let results = archive_service.search(query)?;
            let display_info: Vec<_> = results
                .iter()
                .map(|arch| {
                    arch.to_display_info(UiType::Cli {
                        detailed: *detailed,
                    })
                })
                .collect();
            display_archives(&display_info);
        }
        ArchiveCommands::Cleanup { dry_run: _ } => {
            let count = archive_service.cleanup()?;
            println!("Cleaned up {count} expired archives");
        }
        ArchiveCommands::Stats => {
            let stats = archive_service.get_statistics()?;
            display_statistics(&stats);
        }
        ArchiveCommands::Retention { id, days } => {
            archive_service.update_retention(*id, *days)?;
            println!("Update retention period for archive {id} to {days}");

            // Show new retention information
            let info = archive_service.get_retention_info(*id)?;
            display_retention_info(*id, &info);
        }
    }

    Ok(())
}

fn display_archives(archives: &[ArchiveDisplayInfo]) -> std::io::Result<()> {
    if archives.is_empty() {
        println!("No archives found");
        return Ok(());
    }

    for info in archives {
        println!("{}: {} ({}) {}", info.id, info.name, info.age, info.status);

        if let Some(reason) = &info.reason {
            println!(" Reason: {reason}");
        }

        if let Some(size) = &info.size {
            println!(" Size: {size}");
        }
    }

    Ok(())
}

fn display_statistics(stats: &ArchiveStatistics) {
    let formatter = DefaultFormatter;

    println!("Archive Statistics");
    println!("=================");
    println!("Total archives:     {}", stats.total_archives);
    println!("Expired archives:   {}", stats.expired_count);
    println!(
        "Total size:         {}",
        formatter.format_size(stats.total_size)
    );
    println!("Avg retention:      {:.1} days", stats.avg_retention_days);
}

fn display_retention_info(id: i64, info: &RetentionInfo) {
    println!("Retention Information for Archive {id}");
    println!("================================");
    println!(
        "Archive date:       {}",
        info.archive_date.format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "Retention period:   {} days",
        info.retention_period.num_days()
    );

    if let Some(remaining) = &info.time_remaining {
        println!("Time remaining:       {} days", remaining.num_days());
    } else {
        println!("Status: Expired (can be deleted)");
    }
}

use clap::Args;

#[derive(Args, Clone, Debug, PartialEq)]
#[clap(about = "Archive a configuration file")]
pub struct ArchiveArgs {
    #[arg(
        short,
        long,
        required = true,
        help = "Name of the configuration file to archive"
    )]
    pub name: String,
    #[arg(
        short,
        long,
        required = true,
        help = "Path to the configuration file to archive"
    )]
    pub path: String,
}

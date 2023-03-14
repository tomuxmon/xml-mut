use clap::{Args, Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct MutCli {
    /// path to xml mutation file, usually with a .xut file extension
    pub xml_mut_path: std::path::PathBuf,
    #[command(subcommand)]
    pub xmls: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// scan for xml files in a directory
    Scan(ScanArgs),
    /// include multiple xml files
    Include(IncludeArgs),
}

#[derive(Args, Debug)]
pub struct ScanArgs {
    /// directory to scan for xml files
    pub base_path: std::path::PathBuf,
    /// extension of xml file to be included (can be multiple)
    #[arg(short, long, required = true, action = clap::ArgAction::Append)]
    pub extension: Vec<String>,
}

#[derive(Args, Debug)]
pub struct IncludeArgs {
    /// xml path to be included (can be multiple)
    #[arg(short, long, required = true, action = clap::ArgAction::Append)]
    pub xml_path: Vec<std::path::PathBuf>,
}

impl MutCli {
    pub fn scan(&self) -> Vec<std::path::PathBuf> {
        match &self.xmls {
            Commands::Scan(s) => s.scan(),
            Commands::Include(p) => p.xml_path.clone(),
        }
    }
}

impl ScanArgs {
    pub fn scan(&self) -> Vec<std::path::PathBuf> {
        WalkDir::new(self.base_path.clone())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                if e.metadata().map(|m| m.is_file()).unwrap_or(false)
                    && self.extension.iter().any(|ext| {
                        ext == e
                            .path()
                            .extension()
                            .and_then(std::ffi::OsStr::to_str)
                            .unwrap_or("")
                    })
                {
                    Some(e.into_path())
                } else {
                    None
                }
            })
            .collect()
    }
}

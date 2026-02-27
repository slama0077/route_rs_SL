use crate::kernel::muskingum::MuskingumCungeKernel;
use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use num_cpus;
use std::path::PathBuf;
/// Network routing simulation tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Route (root) directory path
    route_dir: PathBuf,

    /// Optional GeoPackage file path (defaults to first .gpkg in <route_dir>/config)
    gpkg_file: Option<PathBuf>,

    /// Optional lateral inflow CSV directory (defaults to <route_dir>/outputs/ngen)
    csv_dir: Option<PathBuf>,

    /// Optional output data directory (defaults to <route_dir>/outputs/troute)
    data_dir: Option<PathBuf>,

    /// Internal timestep in seconds
    #[arg(short, long, default_value_t = 300)]
    internal_timestep_seconds: usize,
    #[arg(short, long, default_value_t = MuskingumCungeKernel::TRouteModernized)]
    kernel: MuskingumCungeKernel,
    #[arg(short, long, default_value_t = num_cpus::get())]
    num_threads: usize,
}
pub fn print_banner(config: &Config) {
    eprintln!("   {}", "🌊 Route RS".cyan().bold());
    eprintln!("  Kernel:   {}", format!("{}", config.kernel).green());
    eprintln!("  Timestep: {}s", config.internal_timestep_seconds);
    eprintln!("  Threads:  {}", config.num_threads);
    eprintln!(
        "  GeoPackage: {}",
        config.gpkg_file.display().to_string().dimmed()
    );
    eprintln!();
}
pub struct Config {
    pub config_dir: PathBuf,
    pub csv_dir: PathBuf,
    pub gpkg_file: PathBuf,
    pub internal_timestep_seconds: usize,
    pub output_dir: PathBuf,
    pub kernel: MuskingumCungeKernel,
    pub num_threads: usize,
}

pub fn get_args() -> Result<Config> {
    let args = Args::parse();

    let root_dir = args.route_dir;
    let config_dir = root_dir.join("config");

    let csv_dir = args
        .csv_dir
        .unwrap_or_else(|| root_dir.join("outputs").join("ngen"));

    let output_dir = args
        .data_dir
        .unwrap_or_else(|| root_dir.join("outputs").join("troute"));

    // Use provided .gpkg path when available; otherwise preserve legacy discovery.
    let gpkg_file = if let Some(path) = args.gpkg_file {
        path
    } else {
        config_dir
            .read_dir()
            .context("Failed to read config directory")?
            .filter_map(Result::ok)
            .find(|entry| entry.path().extension().map_or(false, |ext| ext == "gpkg"))
            .ok_or_else(|| anyhow::anyhow!("No .gpkg file found in config directory"))?
            .path()
    };

    let cfg = Config {
        config_dir,
        csv_dir,
        gpkg_file,
        internal_timestep_seconds: args.internal_timestep_seconds,
        output_dir,
        kernel: args.kernel,
        num_threads: args.num_threads,
    };
    print_banner(&cfg);
    Ok(cfg)
}

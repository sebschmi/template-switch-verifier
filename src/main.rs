use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use clap::Parser;
use lib_tsalign::a_star_aligner::{
    alignment_result::AlignmentResult, template_switch_distance::AlignmentType,
};
use log::{info, LevelFilter};
use serde::{Deserialize, Serialize};
use simplelog::{ColorChoice, TermLogger, TerminalMode};

#[derive(Parser)]
struct Cli {
    #[clap(index = 1)]
    ground_truth_statistics: PathBuf,

    #[clap(index = 1)]
    test_statistics: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct StatisticsFile {
    #[serde(flatten)]
    pub statistics: AlignmentResult<AlignmentType>,

    #[serde(flatten)]
    pub parameters: HashMap<String, String>,
}

fn main() -> Result<(), String> {
    TermLogger::init(
        LevelFilter::Info,
        Default::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let cli = Cli::parse();

    info!(
        "Loading ground truth statistics from {:?}",
        cli.ground_truth_statistics
    );
    let ground_truth_statistics = read_to_string(&cli.ground_truth_statistics).unwrap();
    let ground_truth_statistics: StatisticsFile = toml::from_str(&ground_truth_statistics).unwrap();
    let ground_truth_statistics = ground_truth_statistics.statistics;

    info!("Loading test statistics from {:?}", cli.test_statistics);
    let test_statistics = read_to_string(&cli.test_statistics).unwrap();
    let test_statistics: StatisticsFile = toml::from_str(&test_statistics).unwrap();
    let test_statistics = test_statistics.statistics;

    (ground_truth_statistics.alignment != test_statistics.alignment)
        .then_some(())
        .ok_or_else(|| "Alignment mismatch".to_string())
}

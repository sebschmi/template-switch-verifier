use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use clap::Parser;
use lib_tsalign::{
    a_star_aligner::{alignment_result::AlignmentResult, template_switch_distance::AlignmentType},
    costs::U64Cost,
};
use log::{LevelFilter, info};
use serde::{Deserialize, Serialize};
use serde_value::Value;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

#[derive(Parser)]
struct Cli {
    #[clap(index = 1)]
    ground_truth_statistics: PathBuf,

    #[clap(index = 2)]
    test_statistics: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct StatisticsFile {
    pub statistics: AlignmentResult<AlignmentType, U64Cost>,

    #[serde(flatten)]
    pub parameters: HashMap<String, Value>,
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
    let ground_truth_statistics: StatisticsFile = toml::from_str(&ground_truth_statistics)
        .unwrap_or_else(|error| panic!("Error parsing ground truth statistics: {error}"));
    let AlignmentResult::WithTarget {
        alignment: ground_truth_alignment,
        statistics: ground_truth_statistics,
    } = ground_truth_statistics.statistics
    else {
        return Err("Ground truth alignment missing".to_string());
    };

    info!("Loading test statistics from {:?}", cli.test_statistics);
    let test_statistics = read_to_string(&cli.test_statistics).unwrap();
    let test_statistics: StatisticsFile = toml::from_str(&test_statistics)
        .unwrap_or_else(|error| panic!("Error parsing test statistics: {error}"));
    let AlignmentResult::WithTarget {
        alignment: test_alignment,
        statistics: test_statistics,
    } = test_statistics.statistics
    else {
        return Err("Test alignment missing".to_string());
    };

    let mut error = false;

    if ground_truth_alignment == test_alignment {
        info!("The alignments are the same");
    } else {
        info!("The alignments are NOT the same");
    }

    if ground_truth_statistics.cost == test_statistics.cost {
        info!("The costs are the same");
    } else {
        error = true;
        info!("The costs are NOT the same");
    }

    (!error)
        .then_some(())
        .ok_or_else(|| "see the log above".to_string())
}

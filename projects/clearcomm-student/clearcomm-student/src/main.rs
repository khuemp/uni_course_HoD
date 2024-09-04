#![forbid(unsafe_code)]

use clearcomm::*;
use color_eyre::eyre::Result;
mod coder;
use coder::{decode, encode};

pipeline!(encode, decode);

#[async_std::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let style = parse_style_from_args();
    let mut results = vec![];
    for mut channel in channels() {
        let run_metrics = pipeline_run(&mut channel).await?;
        results.push(analytics::analyze(&channel, run_metrics).await?);
    }
    analytics::report(&results, style);
    Ok(())
}

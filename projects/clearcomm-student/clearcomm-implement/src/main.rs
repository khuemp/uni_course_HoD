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
        // tạo ra các struct Channels với h và tau khác nhau
        let run_metrics = pipeline_run(&mut channel).await?; // chạy toàn bộ pipeline (encoder,...,decoder) với given channel để nhận thời gian đi qua pipeline, số input byte, byte sau khi đi qua encoder vào pipeline
        results.push(analytics::analyze(&channel, run_metrics).await?); // results contain list of Analytics struct
    }
    analytics::report(&results, style); // lấy thông tin từ list of Analytics struct từ results để print
    Ok(())
}

use async_std::{
    fs::{File, OpenOptions},
    io::BufReader,
    io::BufWriter,
    prelude::*,
};
use color_eyre::eyre::Result;

const BUF_SIZE: usize = 4096;

#[macro_export]
macro_rules! pipeline {
    ($encode:ident, $decode:ident) => {
        async fn pipeline_run(
            // cho chạy toàn bộ pipeline với input là file mp4 để đo thời gian đi qua pipeline, số input byte, byte sau khi đi qua encoder vào pipeline
            channel: &mut crate::channel::Channel, // với từng giá trị channel được nhận thông qua channels()
        ) -> Result<(std::time::Duration, u32, u32)> {
            use async_std::prelude::*;
            use std::time::{Duration, Instant};

            let start = Instant::now(); // bắt đầu tính giờ
            let stream = crate::pipeline::input().await?; // convert input từ file mp4
            let mut input_byte_count: u32 = 0;
            let stream = stream.map(|b| {
                input_byte_count += 1; // iterate qua từng b trong stream để đếm số byte trong input
                b
            });
            let stream = $encode(stream).await?; // stream chứa input được xử lý qua encoder để cho vào channel
            let mut channel_byte_count: u32 = 0;
            let stream = stream.map(|b| {
                channel_byte_count += 1; // iterate qua từng b trong stream để đếm số byte khi chuẩn bị vào channel
                b
            });
            let stream = channel.process(stream).await?; // stream đi qua phần giữa encoder và decoder (channel) mang theo error
            let stream = $decode(stream).await?; // stream đi đến decoder
            crate::pipeline::output(stream).await?; // convert thành output sau khi decode
            Ok((start.elapsed(), input_byte_count, channel_byte_count))
            // output của function là thời gian đi qua pipeline (dùng async/.await để execute từng function một để tính giờ), số input byte, byte sau khi đi qua encoder vào pipeline
        }
    };
}

pub async fn input() -> Result<impl Stream<Item = u8>> {
    // đọc input và biến thành mã u8 chứ không phải 01 từ file mp4
    let file = BufReader::with_capacity(BUF_SIZE, File::open("resources/original.mp4").await?);
    Ok(file.bytes().map(|b| b.unwrap()))
}

pub async fn output<S>(mut stream: S) -> Result<()>
// biến mã sau khi được transmitted thành output
where
    S: Stream<Item = u8> + std::marker::Unpin,
{
    let mut output = BufWriter::with_capacity(
        BUF_SIZE,
        OpenOptions::new()
            .create(true)
            .write(true)
            .open("result.mp4")
            .await?,
    );
    while let Some(b) = stream.next().await {
        let buf = vec![b];
        output.write(&buf[..]).await?;
    }
    output.flush().await?;
    Ok(())
}

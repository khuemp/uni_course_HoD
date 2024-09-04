use async_std::prelude::*;
use color_eyre::eyre::Result;

pub(super) async fn encode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(byte) = stream.next().await {
        data.push(byte);
    }
    let output = async_std::stream::from_iter(data);
    Ok(output)
}

pub(super) async fn decode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(byte) = stream.next().await {
        data.push(byte);
    }
    let output = async_std::stream::from_iter(data);
    Ok(output)
}

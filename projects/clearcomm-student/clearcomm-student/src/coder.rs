use async_std::prelude::*;
use color_eyre::eyre::Result;

pub(super) async fn encode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(byte) = stream.next().await {
        // need await to execute each iteration separately
        let data_bits = one_byte_to_eight_bits(byte);
        let mut data_bits_after_hc = vec![];
        for i in 0..data_bits.len() {
            data_bits_after_hc.push(data_bits[i]);
            data_bits_after_hc.push(data_bits[i]);
            data_bits_after_hc.push(data_bits[i]);
        }
        let third_data_byte = eight_bits_to_one_byte(data_bits_after_hc.split_off(16));
        let second_data_byte = eight_bits_to_one_byte(data_bits_after_hc.split_off(8));
        let first_data_byte = eight_bits_to_one_byte(data_bits_after_hc);
        data.push(first_data_byte);
        data.push(second_data_byte);
        data.push(third_data_byte);
    }
    let output = async_std::stream::from_iter(data);
    Ok(output)
}

pub(super) async fn decode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    let mut data_bytes = vec![];
    while let Some(byte) = stream.next().await {
        data_bytes.push(byte);
        if data_bytes.len() == 3 {
            let mut data_bits = vec![
                one_byte_to_eight_bits(data_bytes[0]),
                one_byte_to_eight_bits(data_bytes[1]),
                one_byte_to_eight_bits(data_bytes[2]),
            ]
            .concat();

            let mut sum = 0;
            for i in 1..=8 {
                let data_bits_block = hamming_code_decode(data_bits.split_off(24 - i * 3));
                sum += u8::pow(2, (8 - i) as u32) * data_bits_block;
            }
            data.push(sum);
            data_bytes.clear();
        }
    }
    let output = async_std::stream::from_iter(data);
    Ok(output)
}

fn one_byte_to_eight_bits(byte: u8) -> Vec<u8> {
    // 1->2->4->8->16->32->64->128
    let mut bits = vec![];
    for i in 0..8 {
        bits.push((byte >> i) & 1);
    }
    bits
}

fn eight_bits_to_one_byte(bits: Vec<u8>) -> u8 {
    let mut bytes = 0;
    for i in 0..bits.len() {
        bytes += u8::pow(2, i as u32) * bits[i];
    }
    bytes
}

// Hamming Code (3,1)

fn hamming_code_decode(data_block_length_3: Vec<u8>) -> u8 {
    match data_block_length_3[..] {
        [0, 0, 0] | [0, 0, 1] | [0, 1, 0] | [1, 0, 0] => 0,
        [0, 1, 1] | [1, 0, 1] | [1, 1, 0] | [1, 1, 1] => 1,
        _ => panic!("Wrong bits"),
    }
}

use async_std::prelude::*;
use color_eyre::eyre::Result;

pub(super) async fn encode(
    mut stream: impl Stream<Item = u8> + Unpin,
) -> Result<impl Stream<Item = u8>> {
    let mut data = vec![];
    while let Some(byte) = stream.next().await {
        // need await to execute each iteration separately
        //print!("{}  ", byte);
        let data_bits = one_byte_to_eight_bits(byte);
        let (first_data_bits, second_data_bits) = data_bits.split_at(4);
        let first_data_byte = eight_bits_to_one_byte(hamming_code_encode(first_data_bits.to_vec()));
        let second_data_byte =
            eight_bits_to_one_byte(hamming_code_encode(second_data_bits.to_vec()));
        data.push(first_data_byte);
        data.push(second_data_byte);
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
        if data_bytes.len() == 2 {
            let first_data_byte = data_bytes[0];
            let second_data_byte = data_bytes[1];
            let first_data_bits = one_byte_to_eight_bits(first_data_byte);
            let second_data_bits = one_byte_to_eight_bits(second_data_byte);
            let concatenated_bits = vec![
                hamming_code_decode(first_data_bits),
                hamming_code_decode(second_data_bits),
            ]
            .concat();
            data.push(eight_bits_to_one_byte(concatenated_bits));
            data_bytes = vec![];
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

//Hamming Code (8,4)
fn hamming_code_encode(data_block_length_4: Vec<u8>) -> Vec<u8> {
    let c3 = data_block_length_4[0];
    let c5 = data_block_length_4[1];
    let c6 = data_block_length_4[2];
    let c7 = data_block_length_4[3];
    let p1 = (c3 + c5 + c7) % 2;
    let p2 = (c3 + c6 + c7) % 2;
    let p4 = (c5 + c6 + c7) % 2;
    let p8 = (c3 + c5 + c6 + c7) % 2;

    let mut data_block_length_8 = vec![];
    data_block_length_8.push(p1);
    data_block_length_8.push(p2);
    data_block_length_8.push(c3);
    data_block_length_8.push(p4);
    data_block_length_8.push(c5);
    data_block_length_8.push(c6);
    data_block_length_8.push(c7);
    data_block_length_8.push(p8);
    data_block_length_8
}

fn hamming_code_decode(data_block_length_8: Vec<u8>) -> Vec<u8> {
    let p1 = data_block_length_8[0];
    let p2 = data_block_length_8[1];
    let mut c3 = data_block_length_8[2];
    let p4 = data_block_length_8[3];
    let mut c5 = data_block_length_8[4];
    let mut c6 = data_block_length_8[5];
    let mut c7 = data_block_length_8[6];
    let p8 = data_block_length_8[7];
    let s1 = (p1 + c3 + c5 + c7) % 2;
    let s2 = (p2 + c3 + c6 + c7) % 2;
    let s4 = (p4 + c5 + c6 + c7) % 2;
    let s8 = (p8 + c3 + c5 + c6 + c7) % 2;
    //println!("{} {} {}",s1, s2, s4);
    let error_position = s1 * 1 + s2 * 2 + s4 * 4;
    if s8 == 0 && error_position != 0 {
        match error_position {
            3 => c3 = 0,
            5 => c5 = 0,
            6 => c6 = 0,
            7 => c7 = 0,
            _ => (),
        };
    } else if s8 != 0 && error_position != 0 {
        // two errors can not do anything
    }

    let mut data_block_length_4 = vec![];
    data_block_length_4.push(c3);
    data_block_length_4.push(c5);
    data_block_length_4.push(c6);
    data_block_length_4.push(c7);
    data_block_length_4
}

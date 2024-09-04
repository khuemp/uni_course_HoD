use async_std::{fs::File, io::BufReader, prelude::*};
use color_eyre::eyre::Result;
use num_format::{Locale, ToFormattedString};
use prettytable::{format::Alignment, Row, Table};
use std::{iter::FromIterator, time::Duration};

use crate::channel::{Channel, ChannelInformation};

const BUF_SIZE: usize = 4096;

pub enum Style {
    Pretty,
    CSV,
}

#[derive(Debug)]
pub struct Analytics {
    residual_bit_errors: u32,
    channel_bit_errors: u32,
    input_byte_count: u32,
    channel_byte_count: u32,
    end_to_end_time: Duration,
    channel: ChannelInformation,
}

pub async fn analyze(channel: &Channel, run_metrics: (Duration, u32, u32)) -> Result<Analytics> {
    let (end_to_end_time, input_byte_count, channel_byte_count) = run_metrics;
    let input = BufReader::with_capacity(BUF_SIZE, File::open("resources/original.mp4").await?);
    let output = BufReader::with_capacity(BUF_SIZE, File::open("result.mp4").await?);
    let residual_bit_errors: u32 = input
        .bytes()
        .zip(output.bytes())
        .map::<Result<u32>, _>(|(i, o)| Ok((i? ^ o?).count_ones()))
        .sum()
        .await?;

    Ok(Analytics {
        residual_bit_errors,                              // được tính
        channel_bit_errors: channel.channel_bit_errors(), // đọc từ struct Channel sau khi bị function process làm thay đổi
        input_byte_count, // đọc từ input của function <- nhận được sau khi chạy toàn bộ pipeline
        channel_byte_count, // đọc từ input của function <- nhận được sau khi chạy toàn bộ pipeline
        end_to_end_time,  // đọc từ input của function <- nhận được sau khi chạy toàn bộ pipeline
        channel: channel.channel_information(), // là h và tau được cho sẵn
    })
}

pub fn report(analytics: &[Analytics], style: Style) {
    // vẽ bảng
    use prettytable::format;
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[
                format::LinePosition::Top,
                format::LinePosition::Intern,
                format::LinePosition::Bottom,
            ],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    let mut table = raw_table_from_data(analytics);
    table.set_format(format);
    right_align(&mut table);

    match style {
        Style::Pretty => table.printstd(),
        Style::CSV => {
            let out = std::io::stdout();
            table.to_csv(out).unwrap();
        }
    }
}

fn raw_table_from_data(analytics: &[Analytics]) -> Table {
    // tạo ra bảng từ data trong input Analysis struct
    let locale = &Locale::en;
    let mut table = Table::new();
    table.add_row(Row::from_iter(vec![
        "E2E Time",
        "Input Bits",
        "Channel Bits",
        "Overhead Ratio",
        "Channel",
        "Channel Errors",
        "Residual Errors",
        "Residual Error Ratio",
    ]));
    analytics.into_iter().for_each(|analytics| {
        table.add_row(Row::from_iter(vec![
            format!(
                "{} ms",
                (analytics.end_to_end_time.as_micros() as f64 / 1000f64) as u64
            ),
            format!(
                "{} bit",
                (analytics.input_byte_count * 8).to_formatted_string(locale)
            ),
            format!(
                "{} bit",
                (analytics.channel_byte_count * 8).to_formatted_string(locale)
            ),
            format!(
                "{}%",
                ((analytics.channel_byte_count as f64 / analytics.input_byte_count as f64) - 1.0)
                    * 100.0
            ),
            format!(
                "h: {:.2}, tau: {:.2}",
                analytics.channel.get_h(),
                analytics.channel.get_tau()
            ),
            format!(
                "{}",
                analytics.channel_bit_errors.to_formatted_string(locale)
            ),
            format!(
                "{}",
                analytics.residual_bit_errors.to_formatted_string(locale)
            ),
            format!(
                "{:.3}%",
                (analytics.residual_bit_errors as f64 / analytics.channel_bit_errors as f64)
                    * 100.0
            ),
        ]));
    });
    table
}

fn right_align(table: &mut Table) {
    for row in table.row_iter_mut() {
        for cell in row.iter_mut() {
            cell.align(Alignment::RIGHT);
        }
    }
}

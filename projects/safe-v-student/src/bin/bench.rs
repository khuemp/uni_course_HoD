use prettytable::{Cell, Row, Table};
use safe_v::{parse_style_from_args, util::parse_dft, Style};
use walkdir::WalkDir;

fn main() {
    let style = parse_style_from_args();
    let files = WalkDir::new("ffort")
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => {
                if entry
                    .file_name()
                    .to_str()
                    .map(|s| s.ends_with(".dft"))
                    .unwrap_or_else(|| false)
                {
                    Some(entry)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .map(|entry| format!("{}", entry.path().display()))
        .collect::<Vec<_>>();

    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("File"),
        Cell::new("Inference Time"),
    ]));

    files.into_iter().for_each(|file| {
        let text = &std::fs::read_to_string(&file).unwrap();
        let ft = parse_dft(text);
        let start = std::time::Instant::now();
        let _ = ft.minimal_cut_sets();
        let elapsed = start.elapsed();
        table.add_row(Row::new(vec![
            Cell::new(&file),
            Cell::new(&format!("{} μs", elapsed.as_micros())),
        ]));
    });
    match style {
        Style::Pretty => table.printstd(),
        Style::CSV => {
            let out = std::io::stdout();
            table.to_csv(out).unwrap();
        }
    }
}

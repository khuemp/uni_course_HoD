mod tui;

use format_lib::line;
use line::*;

fn split_words(content: &str, length: usize) -> Vec<String> {
    content
        .split_whitespace()
        .map(|w| w.to_string())
        .flat_map(|s| {
            s.as_bytes()
                .chunks(length)
                .map(|w| String::from_utf8(w.into()).unwrap())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read from file {}.", &path))
}

fn compose_lines(words: Vec<String>, length: usize) -> Vec<Line> {
    let mut lines = vec![];
    let mut line = Line::new(length);
    for word in words {
        match line.try_push(word) {
            Some(word) => {
                lines.push(line);
                line = Line::new(length);
                line.try_push(word);
            }
            None => {}
        }
    }
    lines.push(line);
    lines
}

fn format(lines: Vec<Line>, length: usize) -> String {
    lines
        .into_iter()
        .map(|l| format!("{:>length$}", l, length = length))
        .collect::<Vec<String>>()
        .join("\n")
}

fn main() {
    let (path, length) = tui::parse_args();
    let content = read_file(&path);
    let words = split_words(&content, length);
    let lines = compose_lines(words, length);
    let formatted = format(lines, length);
    tui::output(&formatted);
}

#[cfg(test)]

mod tests {
    use crate::{compose_lines, Line};

    #[test]
    fn test_compose_lines() {
       // write tests
    }

    #[test]
    fn test_collect_words() {
        // write tests
    }
}


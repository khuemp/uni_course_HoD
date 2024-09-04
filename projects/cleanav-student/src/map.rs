use std::path::Path;

use crate::navigation;
use crate::tui;

#[derive(Clone, Debug)]
pub struct Map {
    cells: Vec<Vec<navigation::Cell>>,
}

impl Map {
    pub fn from_path(path: impl AsRef<Path>) -> Map {
        let map_content = tui::read_map(path);
        map_content.into()
    }

    pub fn get(&self, position: (usize, usize)) -> navigation::Cell {
        self.cells[position.1][position.0]
    }

    pub fn clean(&mut self, position: (usize, usize)) {
        match self.get(position) {
            navigation::Cell::Trash => {
                self.cells[position.1][position.0] = navigation::Cell::Free;
            }
            _ => unreachable!("Attempted to clean a non-Trash patch."),
        }
    }

    pub fn width(&self) -> usize {
        self.cells.get(0).unwrap().len()
    }

    pub fn height(&self) -> usize {
        self.cells.len()
    }
}

impl From<String> for Map {
    fn from(text: String) -> Self {
        let cells = text
            .lines()
            .into_iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| navigation::Cell::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { cells }
    }
}

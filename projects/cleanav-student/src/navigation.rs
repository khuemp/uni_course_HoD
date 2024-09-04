use crate::error::Error;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    Free,
    Trash,
    Buoy,
}

#[derive(Clone, Debug)]
pub struct State {
    x: usize,
    y: usize,
    visited_cells: HashMap<Cell, usize>,
    num_steps: usize,
}

impl State {
    pub fn new() -> Self {
        let visited_cells = HashMap::new();
        Self {
            x: 0,
            y: 0,
            visited_cells,
            num_steps: 0,
        }
    }
    pub fn set_ship_x(&mut self, x: usize) {
        self.x = x;
    }
    pub fn set_ship_y(&mut self, y: usize) {
        self.y = y;
    }
    pub fn get_ship_x(&self) -> usize {
        self.x
    }
    pub fn get_ship_y(&self) -> usize {
        self.y
    }
    pub fn add_visited_cell(&mut self, key: Cell) {
        *self.visited_cells.entry(key).or_insert(0) += 1;
    }
    pub fn get_num_symbol(&self, symbol: &Cell) -> usize {
        if self.visited_cells.get(symbol) == None {
            0
        } else {
            *self.visited_cells.get(symbol).unwrap()
        }
    }
    pub fn get_num_distinct_symbols(&self) -> usize {
        self.visited_cells.len()
    }
    pub fn increase_num_steps_by_1(&mut self) {
        self.num_steps += 1;
    }
    pub fn get_num_steps(&self) -> usize {
        self.num_steps
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Free),
            'X' => Ok(Cell::Trash),
            'O' => Ok(Cell::Buoy),
            c => Err(Error::InvalidCellValue(c)),
        }
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Free => '.',
            Cell::Trash => 'X',
            Cell::Buoy => 'O',
        }
    }
}

pub enum Termination {
    Steps(usize),
    Symbol(Cell),
    Position(usize, usize),
}

impl From<String> for Termination {
    fn from(text: String) -> Self {
        match text.as_str() {
            "S,." => Termination::Symbol(Cell::Free),
            "S,X" => Termination::Symbol(Cell::Trash),
            "S,O" => Termination::Symbol(Cell::Buoy),
            _ => {
                if text.contains("P") {
                    let v: Vec<&str> = text.split(',').collect();
                    Termination::Position(v[1].parse().unwrap(), v[2].parse().unwrap())
                } else {
                    //Termination::Steps(text.parse().unwrap())
                    match text.parse::<usize>() {
                        Ok(num) => Termination::Steps(num), 
                        Err(_) => panic!("Wrong argument {}", text),
                    }
                }
            }
        }
    }
}

pub(crate) fn terminal(termination: &Termination, map: &crate::map::Map, state: &State) -> bool {
    match termination {
        Termination::Steps(steps) => state.get_num_steps() == *steps,
        Termination::Symbol(cell) => map.get((state.get_ship_x(), state.get_ship_y())) == *cell,
        Termination::Position(x, y) => state.get_ship_x() == *x && state.get_ship_y() == *y,
    }
}

pub enum Output {
    Steps,
    SymbolCount(Cell),
    TerminalSymbol,
    Position,
    DistinctSymbol,
}

impl FromStr for Output {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Output::from(s))
    }
}

impl<'a, S> From<S> for Output
where
    S: Into<&'a str>,
{
    fn from(text: S) -> Self {
        let text: &str = text.into();
        match text.to_lowercase().as_str() {
            "n" => Output::Steps,
            "c,." => Output::SymbolCount(Cell::Free),
            "c,x" => Output::SymbolCount(Cell::Trash),
            "c,o" => Output::SymbolCount(Cell::Buoy),
            "s" => Output::TerminalSymbol,
            "p" => Output::Position,
            "d" => Output::DistinctSymbol,
            _ => panic!("{} is not supported", text),
        }
    }
}

pub(crate) fn output(output: &Output, end: (crate::map::Map, State)) -> String {
    match output {
        Output::Steps => end.1.get_num_steps().to_string(),
        Output::SymbolCount(symbol) => {
            if end.1.get_num_distinct_symbols() == 0 {
                if *symbol == end.0.get((0, 0)) {
                    1.to_string()
                } else {
                    0.to_string()
                }
            } else {
                end.1.get_num_symbol(symbol).to_string()
            }
        }
        Output::TerminalSymbol => {
            let res = char::from(end.0.get((end.1.get_ship_x(), end.1.get_ship_y())));
            if res == 'X' {
                '.'.to_string()
            } else {
                res.to_string()
            }
        }
        Output::Position => {
            format!("{:?}", (end.1.get_ship_x(), end.1.get_ship_y()))
        }
        Output::DistinctSymbol => {
            if end.1.get_num_distinct_symbols() == 0 {
                1.to_string()
            } else {
                end.1.get_num_distinct_symbols().to_string()
            }
        }
    }
}

pub(crate) fn step(map: &mut crate::map::Map, state: &mut State, slope: (usize, usize)) -> () {
    // add first visited position (0,0)
    if state.get_num_distinct_symbols() == 0 {
        state.add_visited_cell(map.get((0, 0)))
    }
    let old_position = (state.get_ship_x(), state.get_ship_y());
    let new_position = (
        (state.get_ship_x() + slope.0) % map.width(),
        (state.get_ship_y() + slope.1) % map.height(),
    );

    // clean only after check termination criteria
    if map.get(old_position) == Cell::Trash {
        map.clean(old_position)
    }
    state.set_ship_x(new_position.0);
    state.set_ship_y(new_position.1);
    state.add_visited_cell(map.get(new_position));
    state.increase_num_steps_by_1();
}

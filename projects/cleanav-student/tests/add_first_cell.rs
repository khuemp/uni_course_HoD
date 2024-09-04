use cleanav::{map::Map, navigation, run};

#[test]
fn num_steps_num_symbol_encoutered_1() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(0),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("1")
    );
}

#[test]
fn num_steps_num_symbol_encoutered_0() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(0),
            navigation::Output::SymbolCount(navigation::Cell::Free)
        ),
        format!("0")
    );
}

#[test]
fn num_steps_terminal_cell_symbol() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(0),
            navigation::Output::TerminalSymbol
        ),
        format!(".")
    );
}

#[test]
fn num_steps_position() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(0),
            navigation::Output::Position
        ),
        format!("{:?}", (0, 0))
    );
}

#[test]
fn num_steps_num_distinct_symbol() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(0),
            navigation::Output::DistinctSymbol
        ),
        format!("1")
    );
}
//-------------------------------------------------------------------------------------------
#[test]
fn reach_symbol_first_time_num_steps() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::Steps
        ),
        format!("0")
    );
}

#[test]
fn reach_symbol_first_time_num_symbol_encoutered() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("1")
    );
}

#[test]
fn reach_symbol_first_time_num_symbol_encoutered_another() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Free),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("1")
    );
}

#[test]
fn reach_symbol_first_time_num_symbol_encoutered_another_other() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::SymbolCount(navigation::Cell::Free)
        ),
        format!("0")
    );
}

#[test]
fn reach_symbol_first_time_position() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::Position
        ),
        format!("{:?}", (0, 0))
    );
}

#[test]
fn reach_symbol_first_time_num_distinct_symbol() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::DistinctSymbol
        ),
        format!("1")
    );
}
//-------------------------------------------------------------------------------------------
#[test]
fn reach_position_first_time_num_steps() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(0, 0),
            navigation::Output::Steps
        ),
        format!("0")
    );
}

#[test]
fn reach_position_first_time_num_symbol_encoutered() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(0, 0),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("1")
    );
}

#[test]
fn reach_position_first_time_terminal_cell_symbol() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(0, 0),
            navigation::Output::TerminalSymbol
        ),
        format!(".")
    );
}

#[test]
fn reach_position_first_time_position() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(0, 0),
            navigation::Output::Position
        ),
        format!("{:?}", (0, 0))
    );
}

#[test]
fn reach_position_first_time_num_distinct_symbol() {
    let map = Map::from_path("./maps/first_visited.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(0, 0),
            navigation::Output::DistinctSymbol
        ),
        format!("1")
    );
}

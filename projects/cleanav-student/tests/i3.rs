use cleanav::{map::Map, navigation, run, tui};
use proptest::prelude::*;

proptest! {
    #[test]
    fn i3_test_symbolcount(steps in 0usize..1000) {
        let map = Map::from_path("./maps/basic.txt");
        // Because trash is picked up there is at most 1 trash visited
        prop_assert_eq!(run(map, (1,2), navigation::Termination::Steps(steps * 15), navigation::Output::SymbolCount(navigation::Cell::Trash)), format!("{}", steps.min(1)));
    }

    #[test]
    fn i3_test_symbolcount_buoy(steps in 0usize..1000) {
        let map = Map::from_path("./maps/lonely_buoy.txt");
        prop_assert_eq!(run(map, (1,2), navigation::Termination::Steps(steps * 15), navigation::Output::SymbolCount(navigation::Cell::Buoy)), format!("{}", steps));
    }

    #[test]
    fn i3_test_symbol(steps in 1usize..1000) {
        let map_path = "./maps/basic.txt";
        let map_content = tui::read_map(map_path);
        let map = Map::from(map_content.clone());
        let slope = (1usize,2usize);
        let terminal_position = ((slope.0 * steps) % map.width(), (slope.1 * steps) % map.height());
        let mut terminal_symbol = map_content.lines().nth(terminal_position.1).unwrap().chars().nth(terminal_position.0).unwrap();
        if terminal_symbol == 'X' {
            terminal_symbol = '.';
        }
        prop_assert_eq!(run(map, slope, navigation::Termination::Steps(steps), navigation::Output::TerminalSymbol), format!("{}", terminal_symbol));
    }

}

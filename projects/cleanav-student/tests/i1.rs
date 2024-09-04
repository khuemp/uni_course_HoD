use cleanav::{map::Map, navigation, run};
use proptest::prelude::*;

proptest! {
    #[test]
    fn i1_test_steps(steps in 0usize..1000) {
        let map = Map::from_path("./maps/basic.txt");
        prop_assert_eq!(run(map, (1,2),  navigation::Termination::Steps(steps), navigation::Output::Steps), format!("{}", steps));
    }

    #[test]
    fn i1_test_position(steps in 1usize..1000) {
        let map = Map::from_path("./maps/basic.txt");
        let slope = (1usize,2usize);
        let terminal_position = ((slope.0 * steps) % map.width(), (slope.1 * steps) % map.height());
        prop_assert_eq!(run(map, slope, navigation::Termination::Steps(steps), navigation::Output::Position), format!("{:?}", terminal_position));
    }
}

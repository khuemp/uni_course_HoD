use cleanav::{map::Map, navigation, run};

#[test]
fn i2_test_symbolcount_step1() {
    let steps = 15;
    let map = Map::from_path("./maps/basic.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Steps(steps),
            navigation::Output::SymbolCount(navigation::Cell::Trash)
        ),
        format!("{}", 1)
    );
}

#[test]
fn my_terminate_s_free() {
    let map = Map::from_path("./maps/basic.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Free),
            navigation::Output::Position
        ),
        format!("{:?}", (0, 0))
    );
}

#[test]
fn my_terminate_s_x() {
    let map = Map::from_path("./maps/basic.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::Position
        ),
        format!("{:?}", (3, 1))
    );
}

#[test]
fn my_terminate_s_x_great() {
    let map = Map::from_path("./maps/great_pacific_garbage_patch.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Symbol(navigation::Cell::Trash),
            navigation::Output::Position
        ),
        format!("{:?}", (11, 4))
    );
}

#[test]
fn my_terminate_s_o() {
    let map = Map::from_path("./maps/buoys.txt");
    assert_eq!(
        run(
            map,
            (3, 1),
            navigation::Termination::Symbol(navigation::Cell::Buoy),
            navigation::Output::Position
        ),
        format!("{:?}", (12, 1))
    );
}

#[test]
fn my_terminate_p() {
    let map = Map::from_path("./maps/basic.txt");
    assert_eq!(
        run(
            map,
            (1, 2),
            navigation::Termination::Position(2, 1),
            navigation::Output::Position
        ),
        format!("{:?}", (2, 1))
    );
}

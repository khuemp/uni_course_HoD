pub mod analysis;
pub mod error;
pub mod map;
pub mod markov;
pub mod navigation;
pub mod tui;

pub fn run(
    mut map: map::Map,
    slope: (usize, usize),
    termination: navigation::Termination,
    output: navigation::Output,
) -> String {
    let mut state = navigation::State::new();
    loop {
        if navigation::terminal(&termination, &map, &state) {
            //println!("OLocation {} {}, Cell {:#?}",state.get_ship_x(), state.get_ship_y(), map.get((state.get_ship_x(), state.get_ship_y())));
            break navigation::output(&output, (map, state));
        } else {
            //println!("SLocation {} {}, Cell {:#?}",state.get_ship_x(), state.get_ship_y(), map.get((state.get_ship_x(), state.get_ship_y())));
            navigation::step(&mut map, &mut state, slope);
        }
    }
}

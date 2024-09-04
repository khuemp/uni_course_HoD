use cleanav::map::Map;
use cleanav::navigation::Output;
use cleanav::run;
use cleanav::tui;
use std::str::FromStr;

fn main() {
    let tui::Opt {
        slope,
        termination,
        map: map_path,
        output,
    } = tui::parse_args();
    let map = Map::from_path(&map_path);
    let result = run(
        map,
        slope,
        termination.into(),
        Output::from_str(&output).unwrap(),
    );
    tui::output(&result)
}

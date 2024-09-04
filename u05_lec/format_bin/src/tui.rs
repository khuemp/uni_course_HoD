pub(super) fn output(formatted: &str) -> () {
    print!("{}", formatted);
}

pub(super) fn parse_args() -> (String, usize) {
    let args = &std::env::args().into_iter().collect::<Vec<String>>()[1..];
    match args.len() {
        2 => {
            let path = args.get(0).unwrap();
            let length = args.get(1).unwrap();
            let length = length
                .parse()
                .unwrap_or_else(|_| panic!("Couldn not parse {} to number.", length));
            (path.to_string(), length)
        }
        _ => panic!("Must be called with 2 parameters: PATH LENGTH."),
    }
}


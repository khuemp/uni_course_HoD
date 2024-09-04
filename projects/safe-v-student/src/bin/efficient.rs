fn main() {
    let tree = safe_v::trees::bus_tree();

    let start = std::time::Instant::now();
    let mcs = tree.minimal_cut_sets();
    let elapsed = start.elapsed();
    println!("Efficient Minimal Cut Set: {:#?}", mcs);
    println!("Efficient MCS Inference Time: {:?}", elapsed);
}

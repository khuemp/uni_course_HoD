fn main() {
    let tree = safe_v::trees::bus_tree();

    let start = std::time::Instant::now();
    let mcs = tree.naive_minimal_cut_sets();
    let elapsed = start.elapsed();
    println!("Naive Minimal Cut Set: {:#?}", mcs);
    println!("Naive MCS Inference Time: {:?}", elapsed);
}

fn main() {
    let style = safe_v::parse_style_from_args();
    let tree = safe_v::trees::train_tree();
    tree.report_analysis(style);
}

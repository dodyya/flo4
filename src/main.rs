fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = if args.iter().any(|a| a == "solve") {
        flo4::Mode::Solve
    } else {
        flo4::Mode::Play
    };
    let size = args.iter().skip(1).find_map(|a| a.parse::<usize>().ok()).unwrap_or(9);
    pollster::block_on(flo4::run(mode, size));
}

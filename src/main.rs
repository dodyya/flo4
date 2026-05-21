fn main() {
    let mode = match std::env::args().nth(1).as_deref() {
        Some("solve") => flo4::Mode::Solve,
        _ => flo4::Mode::Play,
    };
    pollster::block_on(flo4::run(mode));
}

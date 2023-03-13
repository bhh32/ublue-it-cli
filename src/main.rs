fn main() {
    if let Err(e) = ublue_it_cli::get_args().and_then(ublue_it_cli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

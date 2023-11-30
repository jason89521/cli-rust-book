fn main() {
    if let Err(e) = headr::get_arg().and_then(headr::run) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}

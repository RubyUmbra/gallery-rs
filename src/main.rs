fn main() {
    match gallery_rs::run() {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

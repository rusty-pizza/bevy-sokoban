fn main() {
    if let Err(err) = sokoban::run() {
        println!("Fatal error: {}", err)
    }
}

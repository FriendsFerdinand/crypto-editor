pub mod utils {
    pub fn display_options(vec: Vec<String>) {
        for (pos, e) in vec.iter().enumerate() {
            println!("{}) {}", pos + 1, e);
        }
    }
}

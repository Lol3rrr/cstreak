use stylers::build;

fn main() {
    std::fs::create_dir_all("../target/stylers/").unwrap();
    build(Some(String::from("../target/stylers/main.css")));
}

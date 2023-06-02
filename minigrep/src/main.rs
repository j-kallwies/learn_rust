use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    dbg!(query);
    dbg!(file_path);

    let contents = fs::read_to_string(file_path)
        .expect(format!("Could not open the file {file_path}").as_str());

    dbg!(contents);
}

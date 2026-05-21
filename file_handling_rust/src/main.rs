use std::fs;
fn main() {
    fs::write("database.txt","Hello Database")
        .expect("Unable to write file");
}

use std::{fs, process::Command};

fn generate_documentation() {
    fs::copy("nanoPDB.pyi", "nanoPDB.py").expect("\'nanoPDB.pyi\' file copy error!");

    Command::new("pdoc")
        .args(["--no-show-source", "./nanoPDB.py", "-o", "./docs"])
        .output()
        .expect("HTML docs error!");

    fs::remove_file("nanoPDB.py").expect("\'nanoPDB.py\' file remove error!");
}

fn main() {
    generate_documentation();
}

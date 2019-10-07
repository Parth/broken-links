use std::process::Command;

fn main() {
    let command = Command::new("git")
        .arg("ls-files")
        .output()
        .expect("Could not find git");

    let output = String::from_utf8_lossy(&command.stdout);

    println!("{}", output);
}

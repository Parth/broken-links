use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut links: HashMap<String, bool> = HashMap::new();

    get_file_names()
        .into_iter()
        .map(get_file_content)
        .map(find_links)
        .for_each(|x| x.into_iter().for_each(|link| test_url(&link, &mut links)));
}

fn test_url(uri: &String, cache: &mut HashMap<String, bool>) {
    match cache.get(uri) {
        Some(_) => return,
        None => {}
    };

    cache.insert(uri.to_string(), true);

    let request_attempt = reqwest::get(uri);

    match request_attempt {
        Ok(request) => {
            let status = request.status();

            if status.is_success() {
                println!("Pass: {} => {}", uri, status);
            } else {
                eprintln!("Fail: {} => {}", uri, status);
            }
        }
        Err(e) => eprintln!("Fail: {} => {}", uri, e),
    }
}

fn find_links(file_contents: String) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r#"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)"#).unwrap();
    }

    let mut links: Vec<String> = Vec::new();
    for link in RE.captures_iter(&file_contents) {
        let url = link.get(0).map(|s| s.as_str()).map(String::from).unwrap();

        links.push(url);
    }

    links
}

fn get_file_content(file_name: String) -> String {
    let file = File::open(&file_name);
    let mut contents = String::new();
    match file {
        Ok(mut f) => {
            let result = f.read_to_string(&mut contents);
            if result.is_err() {
                eprintln!("Warn: Failed to read file: {}", file_name);
            }
        }
        Err(e) => eprintln!("Could not read file: {}, error: {}", &file_name, e),
    }

    contents
}

fn get_file_names() -> Vec<String> {
    let command = Command::new("git")
        .arg("ls-files")
        .output()
        .expect("Could not find git");

    if command.status.code().unwrap() == 128 {
        let error = String::from_utf8_lossy(&command.stderr)
            .to_owned()
            .to_string();
        eprint!("{}", error);
        std::process::exit(128);
    }

    let output = String::from_utf8_lossy(&command.stdout)
        .to_owned()
        .to_string();

    output.lines().map(String::from).collect::<Vec<_>>()
}

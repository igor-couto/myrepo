use std::env;
use std::time::Duration;
use ureq;
use colored::*;

fn main() {
    let timeout_seconds = 5;
    let user_name = "igor-couto";

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <substring>", args[0]);
        std::process::exit(1);
    }
    let substring_to_find = &args[1];

    let mut page = 1;

    loop {
        let response : String = ureq::get(&format!("https://api.github.com/users/{}/repos?page={}&per_page=100", user_name, page))
            .set("Accept", "application/vnd.github.v3+json")
            .timeout(Duration::new(timeout_seconds, 0))
            .call()
            .expect("Error: some kind of unexpected error has occurred")
            .into_string()
            .unwrap();

        if response == "[]" {
            break;
        }

        let mut current_start = 0;
        let mut found = false;
        while let Some(start_index) = response[current_start..].find("\"full_name\":\"") {
            let start = current_start + start_index + "\"full_name\":\"".len();
            let end = &response[start..].find("\",").unwrap() + start;
            let full_name = &response[start..end];
            let repo_name = full_name.split('/').nth(1).unwrap_or("");

            if repo_name.contains(substring_to_find) {
                println!("https://github.com/{}/{}", user_name, repo_name.green());
                found = true;
            }
            current_start = end;
        }

        if !found {
            println!("Did not find any repository containing '{}'", substring_to_find.red());
        }

        page += 1;
    }
}
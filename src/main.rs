use std::env;
use std::process::{exit, Command};
use std::time::Duration;
use ureq;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() > 3 {
        eprintln!("Usage: {} [-c | --clone] <substring>", args[0]);
        exit(1);
    }
    args.remove(0);

    let clone_repos = args.contains(&"-c".to_string()) || args.contains(&"--clone".to_string());
    let substring_to_find = args.iter().find(|&arg| arg != "-c" && arg != "--clone");

    let user_name = "igor-couto"; // TODO: get this from configuration
    let timeout_seconds = 5;

    let mut page = 1;
    let mut found = false;

    loop {
        let response: String = ureq::get(&format!(
            "https://api.github.com/users/{}/repos?page={}&per_page=100",
            user_name, page
        ))
        .set("Accept", "application/vnd.github.v3+json")
        .timeout(Duration::new(timeout_seconds, 0))
        .call()
        .expect("Error: some kind of unexpected error has occurred")
        .into_string()
        .unwrap();

        if response == "[]" {
            break;
        }

        let repository_names = extract_repository_names(&response);

        for name in repository_names {
            if substring_to_find.map_or(true, |substring| name.contains(substring)) {
                found = true;
                println!("https://github.com/{}/\x1b[0;32m{}\x1b[0m", user_name, name);

                if clone_repos {
                    let repo_url = format!("https://github.com/{}/{}", user_name, name);
                    let output = Command::new("git")
                        .arg("clone")
                        .arg(&repo_url)
                        .output()
                        .expect("\x1b[0;31mFailed\x1b[0m to execute command");
                    if !output.status.success() {
                        eprintln!("\x1b[0;31mFailed\x1b[0m to clone repository: {}", repo_url);
                        exit(1);
                    }
                }
            }
        }

        page += 1;
    }

    if !found {
        println!(
            "Did not find any repository containing '\x1b[0;31m{}\x1b[0m'",
            substring_to_find.unwrap()
        );
    }
}

fn extract_repository_names(response: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut current_start = 0;

    while let Some(start_index) = response[current_start..].find("\"full_name\":\"") {
        let start = current_start + start_index + "\"full_name\":\"".len();
        let end = &response[start..].find("\",").unwrap() + start;
        let full_name = &response[start..end];
        let repo_name = full_name.split('/').nth(1).unwrap_or("");
        names.push(repo_name.to_string());
        current_start = end;
    }
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_repository_names() {
        let response = r#"[
            {"full_name":"user/repo1",...},
            {"full_name":"user/repo2",...}
        ]"#;
        let names = extract_repository_names(response);
        assert_eq!(names, vec!["repo1", "repo2"]);
    }
}

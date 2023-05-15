use std::env;
use std::process::{exit, Command};
use std::time::Duration;
use ureq;

fn main() {
    let args = parse_arguments(env::args().collect());

    let username = match std::env::var("MYREPO_GITHUB_USERNAME") {
        Ok(value) => value,
        Err(_error) => {
            eprintln!("\x1b[0;31merror:\x1b[0m myrepo does not know what your username is. Please run myrepo -u USERNAME before using it ");
            std::process::exit(0);
        }
    };
    
    let timeout_seconds = 5;
    let mut page = 1;
    let mut found = false;

    loop {
        let response = make_request(username.as_str(), page, timeout_seconds)
            .expect("\x1b[0;31merror:\x1b[0m Some kind of unexpected error has occurred");

        if response == "[]" {
            break;
        }

        let repository_names = extract_repository_names(&response);

        for name in repository_names {
            if args
                .substring_to_find
                .as_ref()
                .map_or(true, |substring| name.contains(substring))
            {
                found = true;
                println!("https://github.com/{}/\x1b[0;32m{}\x1b[0m", username, name);

                if args.clone_repos {
                    clone_repository(username.as_str(), &name);
                }
            }
        }

        page += 1;
    }

    if !found {
        println!(
            "Did not find any repository containing '\x1b[0;31m{}\x1b[0m'",
            args.substring_to_find.unwrap()
        );
    }
}

struct Args {
    clone_repos: bool,
    substring_to_find: Option<String>,
}

fn parse_arguments(mut args: Vec<String>) -> Args {
    args.remove(0);

    let mut clone_repos = false;
    let mut substring_to_find = None;
    let mut username = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-c" | "--clone" => {
                if clone_repos {
                    eprintln!("\x1b[0;31merror:\x1b[0m Duplicate argument: {}", arg);
                    print_usage_and_exit();
                }
                clone_repos = true;
            }
            "-u" | "-user" => {
                match iter.next() {
                    Some(user) => {
                        if username.is_some() {
                            eprintln!("\x1b[0;31merror:\x1b[0m Duplicate argument: {}", arg);
                            print_usage_and_exit();
                        }
                        username = Some(user.clone());
                    }
                    None => {
                        eprintln!("\x1b[0;31merror:\x1b[0m Expected username after {}", arg);
                        print_usage_and_exit();
                    }
                }
            }
            _ => {
                if substring_to_find.is_some() {
                    eprintln!("\x1b[0;31merror:\x1b[0m Unexpected argument: {}", arg);
                    print_usage_and_exit();
                }
                substring_to_find = Some(arg.clone());
            }
        }
    }

    if let Some(username) = username {
        match set_username_windows(&username) {
            Ok(_) => {
                println!("The user {} has been defined successfully", username);
            },
            Err(e) => {
                eprintln!("Failed to set the environment variable: {}", e);
            }
        }
        std::process::exit(0);
    }

    Args {
        clone_repos,
        substring_to_find,
    }
}

fn print_usage_and_exit() {
    eprintln!("Usage: <text> [-c | --clone] [-u | --user <username>]");
    std::process::exit(0);
}

fn set_username_windows(username: &str) -> std::io::Result<()> {
    let output = Command::new("setx")
        .arg("MYREPO_GITHUB_USERNAME")
        .arg(username)
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to set environment variable",
        ));
    }

    Ok(())
}

fn make_request(
    user_name: &str,
    page: i32,
    timeout_seconds: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let response = ureq::get(&format!(
        "https://api.github.com/users/{}/repos?page={}&per_page=100",
        user_name, page
    ))
    .set("Accept", "application/vnd.github.v3+json")
    .timeout(Duration::from_secs(timeout_seconds))
    .call()?;

    Ok(response.into_string()?)
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

fn clone_repository(user_name: &str, repo_name: &str) {
    let repo_url = format!("https://github.com/{}/{}", user_name, repo_name);
    let output = Command::new("git")
        .arg("clone")
        .arg(&repo_url)
        .output()
        .expect("\x1b[0;31merror:\x1b[0m Failed to execute command");

    if !output.status.success() {
        eprintln!(
            "\x1b[0;31merror:\x1b[0m Failed to clone repository: {}",
            repo_url
        );
        exit(1);
    }
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

    #[test]
    fn test_parse_arguments() {
        let args = vec![
            "program_name".to_string(),
            "-c".to_string(),
            "substring".to_string(),
        ];
        let parsed_args = parse_arguments(args);
        assert_eq!(parsed_args.clone_repos, true);
        assert_eq!(parsed_args.substring_to_find.unwrap(), "substring");
    }
}

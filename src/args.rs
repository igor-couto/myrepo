use std::process::Command;

pub struct Args {
    pub clone_repos: bool,
    pub substring_to_find: Option<String>,
}

pub fn parse_arguments(mut args: Vec<String>) -> Args {
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
            "-u" | "-user" => match iter.next() {
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
            },
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
        match set_username(&username) {
            Ok(_) => {
                println!("The user {} has been defined successfully", username);
            }
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

pub fn print_usage_and_exit() {
    eprintln!("Usage: <text> [-c | --clone] [-u | --user <username>]");
    std::process::exit(0);
}

#[cfg(target_os = "windows")]
fn set_username(username: &str) -> std::io::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

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

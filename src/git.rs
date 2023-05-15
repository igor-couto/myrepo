use std::process::{exit, Command};

pub fn clone_repository(user_name: &str, repo_name: &str) {
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

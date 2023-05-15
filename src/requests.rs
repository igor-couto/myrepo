use std::time::Duration;
use ureq;

pub fn make_request(
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

pub fn extract_repository_names(response: &str) -> Vec<String> {
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

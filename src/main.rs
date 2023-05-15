use std::env;

mod args;
mod git;
mod requests;

fn main() {
    let args = args::parse_arguments(env::args().collect());

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
        let response = requests::make_request(username.as_str(), page, timeout_seconds)
            .expect("\x1b[0;31merror:\x1b[0m Some kind of unexpected error has occurred");

        if response == "[]" {
            break;
        }

        let repository_names = requests::extract_repository_names(&response);

        for name in repository_names {
            if args
                .substring_to_find
                .as_ref()
                .map_or(true, |substring| name.contains(substring))
            {
                found = true;
                println!("https://github.com/{}/\x1b[0;32m{}\x1b[0m", username, name);

                if args.clone_repos {
                    git::clone_repository(username.as_str(), &name);
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
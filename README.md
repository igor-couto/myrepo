# myrepo

[![Build Badge](https://github.com/igor-couto/myrepo/actions/workflows/build.yml/badge.svg)](https://github.com/igor-couto/myrepo/actions/workflows/build.yml)
[![Test Badge](https://github.com/igor-couto/myrepo/actions/workflows/test.yml/badge.svg)](https://github.com/igor-couto/myrepo/actions/workflows/test.yml)

This is a command-line tool written in Rust that searches for yours GitHub repositories by a substring. It can also clone the repositories it finds.

## Features

- Search for GitHub repositories by a substring in their names.
- Optionally clone the repositories that are found.

## Installation

To clone repositories, make sure you have Git installed. If not, follow the instructions [here](https://git-scm.com/downloads)

🚧-🚧-🚧

## Usage
```bash
myrepo <substring_to_search> [-c | --clone]
```

`<substring_to_search>` is the substring you want to search for in the repository names.

If you want to clone the repositories that are found, include the `-c` or `--clone` flag.

## Examples
To search for repositories containing "rust":

```bash
myrepo rust
```

To search for and clone repositories containing "rust":

```bash
myrepo rust -c
```

## Possible Improvements

- Configuration file

- Customize the parameters: Timeout, retry a certain number of times before giving up, etc

- Parallel Cloning: Right now, the program clones repositories one by one. If we are dealing with a large number of repositories, this might be time-consuming. We could potentially speed this up by cloning repositories in parallel using threads.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## Author

Feel free to get in touch with me regarding any questions or issues about the **myrepo cli tool**.
If you are having problems, please let me know by [filing an issue](https://github.com/igor-couto/myrepo/issues)


* **Igor Couto** - [igor.fcouto@gmail.com](mailto:igor.fcouto@gmail.com)
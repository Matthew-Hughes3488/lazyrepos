use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn find_git_repos() -> Option<HashSet<PathBuf>> {
    let mut git_repos: HashSet<PathBuf> = HashSet::new();

    let home = match env::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Home directory not found");
            return None;
        }
    };

    let mut directories = collect_dirs(home);

    while !directories.is_empty() {
        directories = filter_directories(&mut directories, &mut git_repos);

        let mut new_search_dirs: Vec<PathBuf> = Vec::new();
        if !directories.is_empty() {
            while let Some(dir) = directories.pop() {
                new_search_dirs.append(&mut collect_dirs(dir));
            }
        }
        directories = new_search_dirs;
    }
    Some(git_repos)
}

fn filter_directories(
    directories: &mut Vec<PathBuf>,
    git_repos: &mut HashSet<PathBuf>,
) -> Vec<PathBuf> {
    let mut non_git_dirs: Vec<PathBuf> = Vec::new();
    while let Some(curr_dir) = directories.pop() {
        if is_git_repo(&curr_dir) {
            git_repos.insert(curr_dir);
            continue;
        }
        non_git_dirs.push(curr_dir);
    }
    non_git_dirs
}

fn collect_dirs(path: PathBuf) -> Vec<PathBuf> {
    let mut directories: Vec<PathBuf> = Vec::new();
    let read_dir = fs::read_dir(path);

    if let Ok(entries) = read_dir {
        for entry_result in entries.flatten() {
            let name = entry_result.file_name();

            if name.to_str().map_or(false, |s| s.starts_with('.')) {
                continue;
            }

            if entry_result.file_type().unwrap().is_dir() {
                directories.push(entry_result.path());
            }
        }
    }
    directories
}

fn is_git_repo(dir: &Path) -> bool {
    dir.join(".git").exists()
}

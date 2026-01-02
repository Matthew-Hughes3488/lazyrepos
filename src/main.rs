use std::{collections::HashSet, path::PathBuf};
mod index_store;
mod repo_finder;

fn main() {
    let index = "repos.txt";
    //let git_repos = repo_finder::find_git_repos().unwrap();
    //println!("{:?}", git_repos);

    let git_repos = match index_store::read_index(index) {
        Ok(repos) => repos,
        Err(_) => {
            let repos = repo_finder::find_git_repos().unwrap();
            index_store::write_index(index, &repos);
            repos
        }
    };

    print_repos(&git_repos);
}

fn print_repos(repos: &HashSet<PathBuf>) {
    for repo in repos {
        println!("{}", repo.display());
    }
}

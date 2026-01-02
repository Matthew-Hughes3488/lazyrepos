use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

pub fn write_index(path: &str, repos: &HashSet<PathBuf>) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for repo in repos {
        writeln!(writer, "{}", repo.display())?;
    }
    Ok(())
}

pub fn read_index(path: &str) -> std::io::Result<HashSet<PathBuf>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut repos: HashSet<PathBuf> = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        repos.insert(PathBuf::from(line));
    }

    Ok(repos)
}

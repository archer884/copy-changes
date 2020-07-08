use hashbrown::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{fs, io};
use structopt::StructOpt;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, StructOpt)]
struct Opt {
    from: String,
    to: String,
    #[structopt(short, long)]
    force: bool,
    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let from = WalkDir::new(&opt.from)
        .into_iter()
        .filter_map(|x| file_with_time(x.ok()?));
    let to = WalkDir::new(&opt.to)
        .into_iter()
        .filter_map(|x| file_with_time(x.ok()?));
    let to: HashMap<_, _> = to
        .filter_map(|x| {
            let unique_path = x.0.strip_prefix(&opt.to).ok()?;
            Some((unique_path.to_owned(), x))
        })
        .collect();

    let from = from.filter_map(|x| {
        let unique_path = x.0.strip_prefix(&opt.from).ok()?;
        match to.get(unique_path) {
            Some(to) if x.1 > to.1 => Some(x.0),
            None => Some(x.0),
            _ => None,
        }
    });

    let mut count = 0;
    let mut cache = HashSet::new();

    for path in from {
        count += 1;
        let new_path = change_prefix(opt.from.as_ref(), opt.to.as_ref(), &path)?;
        if opt.force {
            ensure_directories(&new_path, &mut cache)?;
            fs::copy(&path, &new_path)?;
        }

        if opt.verbose {
            println!("{}\n  -> {}", path.display(), new_path.display());
        }
    }
    println!(
        "{} {} {}",
        if opt.force { "Copied" } else { "Would copy " },
        count,
        if count == 1 { "file" } else { "files" }
    );

    Ok(())
}

fn change_prefix(
    from: &Path,
    to: &Path,
    path: &Path,
) -> Result<PathBuf, std::path::StripPrefixError> {
    let path = path.strip_prefix(from)?;
    Ok(to.join(path))
}

fn file_with_time(entry: DirEntry) -> Option<(PathBuf, SystemTime)> {
    let meta = entry.metadata().ok()?;
    if meta.is_file() {
        Some((entry.into_path(), meta.modified().ok()?))
    } else {
        None
    }
}

fn ensure_directories<'a>(path: &'a Path, cache: &mut HashSet<PathBuf>) -> io::Result<()> {
    let directory = if path.is_dir() {
        path
    } else {
        path.parent()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Parent is root"))?
    };

    if cache.insert(path.into()) {
        fs::create_dir_all(directory)?;
    }
    Ok(())
}

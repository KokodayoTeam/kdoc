use std::fs::{self, create_dir, remove_file, DirEntry, File};
use std::io;
use std::path::Path;
use std::process::exit;
extern crate fs_extra;
use clap::Parser;
use clap::{arg, command};
use fs_extra::copy_items;
use fs_extra::dir;

pub trait Cloner {
    fn clone_dir(from: &str, to: &str) -> io::Result<()>;
}
pub trait FileMapper {
    fn from_json(&mut self, path: &str) -> Self;
    fn do_map(&self);
}

const DEFAULT_DOC_DIR_NAME: &str = "doc";

struct DefaultMapper {
    strings: Vec<String>,
    target: String,
}
impl FileMapper for DefaultMapper {
    fn from_json(&mut self, path: &str) -> Self {
        todo!()
    }
    fn do_map(&self) {
        self.visit_dirs(Path::new(&self.target));
    }
}

impl DefaultMapper {
    fn from_string(target: String, strings: Vec<String>) -> Self {
        Self { strings, target }
    }

    fn map_file(&self, entry: &DirEntry) {
        let mut path = entry.path().to_str().unwrap().to_string();

        for string in &self.strings {
            if path.ends_with(string) {
                path.push_str(".md");
                File::create(path);
                break;
            }
        }
        remove_file(entry.path());
    }
    fn visit_dirs(&self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(&path)?;
                } else {
                    self.map_file(&entry);
                }
            }
        }
        Ok(())
    }
}

struct DefaultCloner {}
impl Cloner for DefaultCloner {
    fn clone_dir(from: &str, to: &str) -> io::Result<()> {
        if !Path::new(to).exists() {
            create_dir(to)?;
        } else {
            println!("doc dir already exists!");
            exit(0);
        }
        let path = Path::new(from);
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            let options = dir::CopyOptions::new();
            let mut from_paths = Vec::new();
            let path_str = path.to_str().unwrap();
            from_paths.push(path_str);
            copy_items(&from_paths, to, &options).unwrap();
        }
        Ok(())
    }
}

#[derive(Parser, Debug)]
#[command(term_width = 0)]
struct Args {
    #[arg(short = 'p')]
    pattern: Vec<String>,

    /// The path to the file to read
    #[arg(long)]
    from: std::path::PathBuf,
}

fn main() {
    //read rules from command-line
    let args = Args::parse();

    //copy dir
    DefaultCloner::clone_dir(args.from.to_str().unwrap(), DEFAULT_DOC_DIR_NAME);

    //traverse the dir and do map-job
    let mapper = DefaultMapper::from_string(DEFAULT_DOC_DIR_NAME.to_string(), args.pattern);
    mapper.do_map();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dir_clone() {
        DefaultCloner::clone_dir("src", "tmp1").unwrap();
    }

    #[test]
    fn do_map() {
        DefaultCloner::clone_dir("src", "tmp2").unwrap();
        let v = vec![".rs".to_string()];
        let dm = DefaultMapper::from_string("tmp2".to_string(), v);
        dm.do_map();
    }
}

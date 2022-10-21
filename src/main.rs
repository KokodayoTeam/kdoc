use std::fs::{self, copy, rename, DirEntry, create_dir};
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
extern crate fs_extra;
use fs_extra::copy_items;
use fs_extra::dir::{self, remove};

pub trait Cloner {
    fn clone_dir(from: &str, to: &str) -> io::Result<()>;
}
pub trait FileMapper {
    fn from_json(path: &str) -> Self;
    fn do_map();
}

struct DefaultMapper {}
impl FileMapper for DefaultMapper {
    fn from_json(path: &str) -> Self {
        DefaultMapper {}
    }

    fn do_map() {
        todo!()
    }
}

struct DefaultCloner {}
impl Cloner for DefaultCloner {
    fn clone_dir(from: &str, to: &str) -> io::Result<()> {
        if !Path::new(to).exists() {
            create_dir(to)?;
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

fn main() {
    //copy dir

    //read rules of filemapper

    //traverse the dir and do map-job
}

#[cfg(test)]
mod tests {
    use crate::{DefaultCloner, Cloner};

    #[test]
    fn dir_clone() {
        DefaultCloner::clone_dir("src", "doc").unwrap();
    }
}
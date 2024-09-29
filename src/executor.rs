use crypto::digest::Digest;
use crypto::sha2::Sha224;
use std::env;
use std::fs;
use std::fs::rename;
use std::io::Read;
use std::path::PathBuf;

use crate::args_parser::OptionFlags;

pub struct Executor<'a> {
    options: &'a OptionFlags,
}

impl<'a> Executor<'a> {
    pub fn new(options: &'a OptionFlags) -> Self {
        Executor { options }
    }

    pub fn execute(&self) {
        if self.options.get_files().is_empty() {
            self.rename_all();
        } else {
            self.execute_specific_files();
        }
    }

    fn execute_specific_files(&self) {
        if self.options.is_debug() {
            println!(
                "Starting to renaiming files from file list: {:?}",
                self.options.get_files()
            )
        }

        let mut full_paths: Vec<PathBuf> = vec![];

        for file in self.options.get_files() {
            let file_path = PathBuf::from(file).canonicalize().unwrap();

            if self.options.is_debug() {
                println!("Got full file path: {:?}", file_path);
            }

            full_paths.push(file_path);
        }

        Executor::rename_files_with_hash(self, full_paths);
    }

    fn rename_all(&self) {
        let curr_dir: PathBuf = env::current_dir().unwrap();

        println!("Current dir: {}", curr_dir.display());

        let paths = Executor::get_img_files(curr_dir);
        if paths.len() != 0 {
            Executor::rename_files_with_hash(self, paths);
        } else {
            println!("There is no img files in current folder!");
        }
    }

    fn get_img_files(curr_dir_path: PathBuf) -> Vec<PathBuf> {
        let paths = fs::read_dir(curr_dir_path).unwrap();

        let mut result_files: Vec<PathBuf> = vec![];
        for entry in paths {
            if let Ok(entry) = entry {
                if Executor::is_media_file_type(&entry.path()) {
                    result_files.push(entry.path());
                }
            }
        }
        return result_files;
    }

    fn get_file_ext(path: &PathBuf) -> String {
        match path.extension() {
            Some(val) => return val.to_str().unwrap().to_owned(),
            None => return String::default(),
        };
    }

    fn is_media_file_type(path: &PathBuf) -> bool {
        if path.is_file() {
            let ext = Executor::get_file_ext(&path);
            match ext.to_lowercase().as_str() {
                "jpg" | "jpeg" | "gif" | "bmp" | "png" | "webp" | "webm" | "tiff" | "mp4"
                | "mpg" | "mov" => return true,
                _ => return false,
            };
        }
        return false;
    }

    fn rename_files_with_hash(&self, paths: Vec<PathBuf>) {
        let mut hasher = Sha224::new();

        for path in paths {
            if self.options.is_debug() {
                println!("Opening file: {}", &path.display());
            }

            let mut file = fs::File::open(&path).unwrap();
            let file_size = file.metadata().unwrap().len();
            let mut file_buff = vec![0; file_size as usize];
            let cnt = match file.read(&mut file_buff[..]) {
                Ok(bytes) => bytes,
                Err(_err) => panic!("Cannot read file {}", &path.display()),
            };

            if self.options.is_debug() {
                println!("READED BYTES: {}", cnt);
            }

            hasher.input(&mut file_buff[..]);
            let hash: String = hasher.result_str();
            let new_file_name = format!("{}.{}", hash, Executor::get_file_ext(&path));
            match rename(&path, &new_file_name) {
                Ok(_val) => {
                    if self.options.is_debug() {
                        println!(
                            "Successfully renamed file\n {} to {:?}",
                            &path.display(),
                            &new_file_name
                        )
                    }
                }
                Err(err) => panic!(
                    "Cannot rename file {}, caused error {}",
                    &path.display(),
                    err
                ),
            };
        }
    }
}


//TODO: add tests
#[cfg(test)]
mod executor_tests {
    use super::*;

    #[test]
    fn test_path() {

    }
}

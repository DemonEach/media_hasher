extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::env::current_dir;
use std::fs;
use std::fs::rename;
use std::io::Read;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let curr_dir: PathBuf = current_dir().unwrap();
    println!("Current dir: {}", curr_dir.display());
    let paths = get_img_files(curr_dir);
    rename_files_with_hash(paths);
    Ok(())
}

fn get_img_files(curr_dir_path: PathBuf) -> Vec<PathBuf> {
    let paths = fs::read_dir(curr_dir_path).unwrap();

    let mut result_files: Vec<PathBuf> = vec![];
    println!("_______________________");
    println!("Found this image files:");
    for entry in paths {
        if let Ok(entry) = entry {
            if is_img(&entry.path()) {
                result_files.push(entry.path());
                println!("{}", entry.path().display());
            }
        }
    }
    println!("_______________________");
    return result_files;
}

fn get_file_ext(path: &PathBuf) -> String {
    return path.extension().unwrap().to_str().unwrap().to_owned();
}

fn is_img(path: &PathBuf) -> bool {
    if path.is_file() {
        let ext = get_file_ext(&path);
        match ext.as_str() {
            "jpg" | "jpeg" | "gif" | "bmp" | "png" | "webp" => return true,
            _ => return false,
        };
    }
    return false;
}

fn rename_files_with_hash(paths: Vec<PathBuf>) {
    for path in paths {
        println!("Opening file: {}", &path.display());
        let mut hasher = Md5::new();
        let mut file = fs::File::open(&path).unwrap();
        let file_size = file.metadata().unwrap().len();
        let mut file_buff = vec![0; file_size as usize];
        let cnt = match file.read(&mut file_buff[..]) {
            Ok(bytes) => bytes,
            Err(_err) => panic!("Cannot read file {}", &path.display()),
        };
        println!("READED BYTES: {}", cnt);
        hasher.input(&mut file_buff[..]);
        let hash: String = hasher.result_str();
        let new_file_name = format!("{}.{}", hash, get_file_ext(&path));
        match rename(&path, &new_file_name) {
            Ok(_val) => println!(
                "Successfully renamed file\n {} to {:?}",
                &path.display(),
                &new_file_name
            ),
            Err(err) => panic!(
                "Cannot rename file {}, caused error {}",
                &path.display(),
                err
            ),
        };
    }
}

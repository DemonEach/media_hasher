mod args_parser;
mod executor;

extern crate crypto;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;

use executor::Executor;

const RUST_LOG_ENV_KEY: &str = "RUST_LOG";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let options = args_parser::parse_args(args.clone());
    let executor = Executor::new(&options);

    if options.is_debug() {
        std::env::set_var(RUST_LOG_ENV_KEY, "debug");
    }

    debug!("Args: {:?}", args);
    debug!("Options: {:?}", options);

    if options.is_help() {
        print_help();

        return Ok(());
    }

    executor.execute();

    Ok(())
}

fn print_help() {
    println!("This program renames all files with the corresponding hashes");
    println!("By default it uses Sha224 and tries to find \"media\" (\"jpg\", \"jpeg\", \"gif\", \"bmp\", \"png\", \"webp\", \"webm\", \"tiff\", \"mp4\", \"mpg\", \"mov\") files in current directory");
    println!("Usage: ih [command] [options]");
    println!("Options:");
    println!("   -h, --help    - help information");
    println!("   -v, --verbose - print debug information");
    println!("   -f, --files    - provided list of files to proceed with renaming");
    println!("   -e, --extensions - provided list of file extensions to search for");
}

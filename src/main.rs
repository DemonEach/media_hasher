mod args_parser;
mod executor;

extern crate crypto;

use std::env;

use executor::Executor;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let options = args_parser::parse_args(args.clone());
    let executor = Executor::new(&options);

    if options.is_debug() {
        println!("Args: {:?}", args);
        println!("Options: {:?}", options);
    }

    if options.is_help() {
        print_help();
        return Ok(());
    }

    executor.execute();

    Ok(())
}

fn print_help() {
    println!("This program renames all files with the corresponding hashes");
    println!("By default it uses Sha224 and tries to find \"media\" files in current directory");
    println!("Usage: ih [command] [options]");
    println!("Options:");
    println!("   -h, --help    - help information");
    println!("   -v, --verbose - print debug information");
    println!("   -f, --files    - provided list of files to proceed with renaming");
}

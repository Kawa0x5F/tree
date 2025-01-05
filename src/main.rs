use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.path);

    recurse_folder(&args.path.to_path_buf());
}

fn recurse_folder(path: &PathBuf) {
    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let path_value = path.unwrap();
                println!("> {:?}", path_value.file_name());
                if path_value.path().is_dir() {
                    recurse_folder(&path_value.path());
                }
            }
        }
    }
}

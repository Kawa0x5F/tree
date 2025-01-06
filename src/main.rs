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

    recurse_folder(args.path.to_path_buf(), 0);
}

fn recurse_folder(path: PathBuf, level: usize) {
    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let path_value = path.unwrap();
                println!(
                    "{}{}",
                    make_hierarchy(level),
                    path_value.file_name().into_string().ok().unwrap()
                );
                if path_value.path().is_dir() {
                    recurse_folder(path_value.path(), level + 1);
                }
            }
        }
    }
}

fn make_hierarchy(level: usize) -> String {
    let mut output_string = String::new();

    if level == 0 {
        return output_string;
    }

    for _i in 0..(level - 1) {
        output_string.push_str("|  ");
    }
    output_string.push_str("|--");

    output_string
}

fn make_last_hierarchy(level: usize) -> String {
    let mut output_string = String::new();

    if level == 0 {
        return output_string;
    }

    for _i in 0..(level - 1) {
        output_string.push_str("   ");
    }
    output_string.push_str("|--");

    output_string
}

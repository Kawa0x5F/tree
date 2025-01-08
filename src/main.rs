use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let hierarchy_string = String::new();

    if args.path.is_file() || args.path.is_dir() {
        println!("{:?}", args.path.to_string_lossy());
        recurse_folder(args.path.to_path_buf(), hierarchy_string);
    } else if args.path.file_name().is_none() {
        recurse_folder(args.path.to_path_buf(), hierarchy_string);
    } else {
        println!("The path does not exist or is not accessible.");
    }
}

fn recurse_folder(path: PathBuf, hierarchy_string: String) {
    match fs::read_dir(path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            let paths_vec: Vec<_> = paths.filter_map(|f| f.ok()).collect();
            for (i, path) in paths_vec.iter().enumerate() {
                let value = path;
                let is_last = i + 1 == paths_vec.len();
                let to_path = if is_last { "`--" } else { "|--" };
                println!(
                    "{}{}{}",
                    hierarchy_string,
                    to_path,
                    value.file_name().into_string().ok().unwrap()
                );
                if value.path().is_dir() {
                    recurse_folder(
                        value.path(),
                        make_hierarchy(hierarchy_string.clone(), is_last),
                    );
                }
            }
        }
    }
}

fn make_hierarchy(hierarchy_string: String, is_last: bool) -> String {
    let mut new_hierarchy_string = String::new();
    new_hierarchy_string.push_str(&hierarchy_string);
    new_hierarchy_string.push(if is_last { ' ' } else { '|' });
    new_hierarchy_string.push_str("  ");
    new_hierarchy_string
}

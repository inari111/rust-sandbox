use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dir = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };

    let path = Path::new(dir);

    let entries = fs::read_dir(path).expect("failed to get entries");

    for entry in entries {
        let entry = entry.expect("failed get entry");
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        if !file_name.starts_with('.') {
            println!("{}", file_name);
        }
    }
}

extern crate walkdir;
extern crate glob;

use walkdir::WalkDir;
use glob::Pattern;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "find", version = "1.0.0", author = "Tri Pham")]
struct CliArg {
    /// The directory to perform search
    #[clap(long, short = 'd')]
    dir: String,
    /// The pattern of the files to find
    #[clap(long, short = 'p')]
    pattern: String,
}

fn main() {
    let args = CliArg::parse();

    let pattern = Pattern::new(&args.pattern).expect("Valid pattern");

    for entry in WalkDir::new(&args.dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && pattern.matches_path(path) {
            println!("{}", path.display().to_string());
        }
    }
}

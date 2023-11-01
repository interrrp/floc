mod args;
mod utils;

fn main() {
    let args = args::parse();

    let files = utils::find_files_with_extension(&args.directory, &args.extension);
    let mut total = 0;
    for file in files {
        let lines = utils::count_lines(&file);
        total += lines;
        println!("{}: {} LoC", file.display(), lines);
    }

    println!("Total: {} LoC", total);

    println!(
        "\nIn conclusion, the {} directory contains {} lines of code throughout all .{} files.",
        args.directory.display(),
        total,
        &args.extension,
    );
}

mod args;

fn main() {
    let args = args::parse();

    println!(
        "Counting LoC of {} files in {}...",
        args.extension, args.directory
    );
}

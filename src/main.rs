use std::sync::{atomic::AtomicU64, Arc};

use owo_colors::OwoColorize;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

mod args;
mod utils;

fn main() {
    let args = args::parse();

    let total = Arc::new(AtomicU64::new(0));
    utils::find_files_with_extensions(&args.directory, &args.extensions)
        .par_iter()
        .for_each(|path| {
            let lines = utils::count_lines(path);
            total.fetch_add(lines as u64, std::sync::atomic::Ordering::SeqCst);
            println!("{}: {} LoC", path.display().bright_black(), lines);
        });

    let total = total.load(std::sync::atomic::Ordering::SeqCst);

    println!("{}", format!("Total: {} LoC", total).green());

    println!(
        "\nIn conclusion, the {} directory contains {} lines of code throughout all {} files.",
        args.directory.display(),
        total,
        utils::seq_to_english(&args.extensions),
    );
}

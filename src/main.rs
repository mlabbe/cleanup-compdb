/*                                                                            *
 * cleanup-compdb Copyright (C) 2022-2025 Frogtoss Games, Inc                 *
 *                                                                            */

use clap::Parser;
use std::io::{self, BufWriter};
use cleanup_compdb::cleanup;
use json_compilation_db::{read, write, Entry};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// arguments to append to the end; useful for suppressing warnings
    #[arg(long, default_value = "")]
    append_arguments: String,
}

fn process_entry(entry: Entry, cli_args: &Args) -> Option<Entry> {
    if cleanup::is_noncompilation_entry(&entry) {
        return None;
    }

    //
    // directory
    let directory = cleanup::enforce_unix_slashes(&entry.directory);

    //
    // file
    let mut file = cleanup::absolute_path_to_file(&entry.file, &entry.directory);
    file = cleanup::enforce_unix_slashes(&file);

    //
    // output
    let output = match &entry.output {
        Some(o) => {
            let file = cleanup::absolute_path_to_file(&o, &entry.directory);
            Some(cleanup::enforce_unix_slashes(&file))
        },
        _ => None,
        
    };
    
    //
    // arguments
    let mut arguments = cleanup::arguments_strip_cmd_c(&entry.arguments);
    arguments = cleanup::append_string_to_arguments(&arguments, &cli_args.append_arguments);

    Some(Entry {
        file,
        arguments,
        directory,
        output,
    })

}

fn main() {

    let cli_args = Args::parse();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let reader = stdin.lock();
    let writer = BufWriter::new(stdout.lock());

    let processed_entries = read(reader).filter_map(|read_result| {
        match read_result {
            Ok(entry) => match process_entry(entry, &cli_args) {
                Some(processed) => Some(processed),
                None => None
            },
            Err(e) => {
                eprintln!("read error: {e}");
                None
            }
        }
    });


    
    if let Err(e) = write(writer, processed_entries) {
        eprintln!("json database write error: {e}");
        std::process::exit(1);
    }

}

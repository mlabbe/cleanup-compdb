
use clap::Parser;
use std::io::{self, BufRead};
use cleanup_compdb::cleanup;
use json_compilation_db;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// arguments to append to the end; useful for suppressing warnings
    #[arg(long, default_value = "")]
    append_arguments: String,
}

// assumes stdin is good actor; heap attack possible
#[allow(dead_code)]
fn read_stdin_until_eof() -> String {
    let mut buf = String::with_capacity(1024 << 12);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        buf.push_str(&line);
    }

    buf
}

fn main() {

    let cli_args = Args::parse();
    
    let entries = match json_compilation_db::from_reader(io::stdin()) {
        Ok(t) => t,

        Err(e) => {
            panic!("error: {:?}", e);
        },
    };

    let mut out_entries: json_compilation_db::Entries = Vec::with_capacity(entries.len());

    for entry in entries.iter() {

        if cleanup::is_noncompilation_entry(&entry) {
            continue;   
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

        let out_entry = json_compilation_db::Entry {
            file: file,
            arguments: arguments,
            directory: directory,
            output: output,
        };

        out_entries.push(out_entry);
    }

    let format = json_compilation_db::Format{
        command_as_array: true,
        drop_output_field: false,
    };

    if let Err(e) = json_compilation_db::to_writer(&out_entries, &format, io::stdout()) {
        eprintln!("error: {}", e);
    }
}

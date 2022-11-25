use std::path::{Path, PathBuf};
use path_absolutize::*;
use json_compilation_db::{Entry};
use std::ffi::OsStr;

use path_slash::PathBufExt as _;

/// force unix slashes, even on Windows.
/// clangd can read this and it increases readability
pub fn enforce_unix_slashes(path: &Path) -> PathBuf {
    let p = PathBuf::from(path);

    let unix_slashes = p.to_slash().unwrap();
    PathBuf::from(unix_slashes.into_owned())
}   

/// instead of a relative path in format '../../../foo.c',
/// enforce an absolute path which is more human-readable
pub fn absolute_path_to_file(file: &Path, from_dir: &Path) -> PathBuf {

    let mut p = PathBuf::from(from_dir);
    p.push(file);

    p.absolutize().unwrap().to_path_buf()
}

/// some entries in the compilation database have nothing to do with
/// compile source code, such as linking or precompile steps.
/// this uses heuristics to detect these.
pub fn is_noncompilation_entry(entry: &Entry) -> bool {

    if entry.arguments.len() == 0 {
        return true;
    }

    match &entry.output {
        Some(o) => {
            if o.extension() == Some(OsStr::new("exe")) {
                return true;
            }
        },
        None => {
            return true;
        }
    }


    false
}

/// ninjabuild in particular, on windows, prefixes the command with
/// 'cmd /c', which clangd doesn't deal with.  This removes it 
/// if it is present.
pub fn arguments_strip_cmd_c(args: &Vec<String>) -> Vec<String> {
    if args.len() < 2 {
        return args.clone();
    }

    if args[0] == "cmd" && args[1] == "/c" {
        let v = args[2..].to_vec();
        return v;
    }

    args.clone()
}

/// append a string to args[0]
/// useful for adding warnings
/// doesn't push new args onto the vector because clangd
/// is giving spurious results when I do that
pub fn append_string_to_arguments(args: &Vec<String>, arg_string: &str) -> Vec<String> {
    if args.len() == 0 {
        let mut v = Vec::new();
        v.push(arg_string.to_string());
        return v;
    }
    let mut out_args = args[0..].to_vec();
    let mut out_arg = out_args[0].clone();
    out_arg.push_str(" ");
    out_arg.push_str(arg_string);

    out_args[0] = out_arg;

    out_args
}
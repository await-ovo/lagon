mod config;
mod console;
mod deployments;
mod trpc;

use std::{
    io::{self, Error, ErrorKind},
    path::{Path, PathBuf},
};

pub use config::*;
pub use console::*;
pub use deployments::*;
pub use trpc::*;

pub fn validate_code_file(file: &Path) -> io::Result<()> {
    if !file.exists() || !file.is_file() {
        return Err(Error::new(
            ErrorKind::Other,
            format!("{} is not a file", file.to_str().unwrap()),
        ));
    }

    match file.extension() {
        Some(ext) => {
            let validate = ext == "js"
                || ext == "jsx"
                || ext == "ts"
                || ext == "tsx"
                || ext == "mjs"
                || ext == "cjs";

            match validate {
                true => Ok(()),
                false => Err(Error::new(ErrorKind::Other, format!("Extension {} is not supported (should be one of .js, .jsx, .ts, .tsx, .mjs, .cjs)", ext.to_str().unwrap()))),
            }
        }
        None => Err(Error::new(
            ErrorKind::Other,
            "No extension found for the given file.",
        )),
    }
}

pub fn validate_public_dir(public_dir: Option<PathBuf>) -> io::Result<PathBuf> {
    if let Some(dir) = public_dir {
        if !dir.is_dir() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Public directory {} does not exist.", dir.to_str().unwrap()),
            ));
        }

        return Ok(dir);
    }

    Ok(PathBuf::from("./public"))
}

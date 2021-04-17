use home;
use std::fs::read_dir;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path_to_search = match args.get(1) {
        Some(path) => PathBuf::from(path),
        None => std::env::current_dir().unwrap(), // home::home_dir().unwrap()
    };

    let count = recursive_count(&path_to_search).unwrap();
    println!(
        "Found {} files under {}",
        count,
        path_to_search.to_str().unwrap()
    )
}

fn recursive_count(path: &PathBuf) -> Result<u32, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err(DirectoryReadingError::new(format!(
            "Path {} could not be found on the file system",
            path.to_str().unwrap()
        ))
        .into());
    }
    let mut count = 0;
    for item in read_dir(path)? {
        let item_path = item?.path();
        if item_path.is_file() {
            count += 1;
        } else if item_path.is_dir() {
            count += match recursive_count(&item_path) {
                Ok(number) => number,
                Err(_) => 0,
            }
        } else {
            return Err(DirectoryReadingError::new(format!(
                "Path {} is not an file nor a directory",
                item_path.to_str().unwrap_or("UNKNOWN")
            ))
            .into());
        }
    }
    Ok(count)
}

#[derive(Debug, Clone)]
pub struct DirectoryReadingError {
    pub message: String,
}

impl DirectoryReadingError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for DirectoryReadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> std::error::Error for DirectoryReadingError {}

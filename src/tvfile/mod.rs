use std::fs;
use std::path::Path;
use std::error::Error;
use std::io::Write;
use std::io::Read;

/// The default filename for the `tvfile`
pub static FILENAME: &str = ".tvfile.csv";

// `csv` headers for the `tvfile`
static FILE_HEADERS: &str = "maze_id, title, latest";

/// Returns a created `.tvfile.csv` file
/// # Arguments
/// * `filename` - A string containing the filename
/// 
/// # Example usage
/// ```
/// tvfile::create_tvfile(tvfile::FILENAME);
/// ```
pub fn create_tvfile(filename: &str) -> fs::File {
    let path = Path::new(filename);
    let display = path.display();
    let mut file = match fs::File::create(&path) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "There was a problem creating the file {}: {}",
                display,
                error.description()
            );
        }  
    };
    match file.write_all(FILE_HEADERS.as_bytes()) {
        Ok(_) => return file,
        Err(error) => {
            panic!(
                "There was a problem writing to the file {}: {}",
                display,
                error.description()
            )
        }
    }
}

#[cfg(test)]
mod create_tvfile_tests {
    use super::*;

    #[test]
    fn test_creates_tvfile() {
        let filename = ".tvfile-1.csv";
        create_tvfile(filename);

        assert!(Path::new(filename).exists());

        tear_down(filename);
    }

    #[test]
    fn test_tvfile_headers() {
        let filename = ".tvfile-2.csv";
        create_tvfile(filename);

        let mut tvfile = fs::File::open(filename).expect("Unable to open file.");
        let mut tvfile_content = String::new();
        tvfile.read_to_string(&mut tvfile_content).expect("Unable to read file.");

        assert_eq!(tvfile_content, FILE_HEADERS);

        tear_down(filename);
    }

    fn tear_down(filename: &str) -> () {
        match fs::remove_file(filename) {
            Ok(success) => return success,
            Err(error) => {
                panic!(error);
            }
        }
    }
}

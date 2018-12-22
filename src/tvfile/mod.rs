use std::error::Error;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;

/// The default filename for the `tvfile`
pub static FILENAME: &str = ".tvfile.csv";

// `csv` headers for the `tvfile`
static FILE_HEADERS: &str = "maze_id, title, latest\n";

/// Creates a `.tvfile.csv` file
/// # Arguments
/// * `filename` - A string containing the filename
///
/// # Returns
/// The creates `.tvfile.csv` file
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
        Err(error) => panic!(
            "There was a problem writing to the file {}: {}",
            display,
            error.description()
        ),
    }
}

/// Appends a new tv series to be tracked
/// # Arguments
/// * `filename` - A string containing the filename
/// * `maze_id` - The ID of the series on the TVMaze API
/// * `title` - The series title
/// * `latest` - The airdate of the latest episode
///
/// # Example usage
/// ```
/// tvfile::store_series(tvfile::FILENAME, 82, "Game of Thrones", "2017-08-27");
/// ```
pub fn store_series(filename: &str, maze_id: i32, title: &str, latest: &str) {
    let series_string = format!("{}, {}, {}", maze_id, title, latest);

    let mut file = fs::OpenOptions::new().append(true).open(filename).unwrap();
    if let Err(e) = writeln!(file, "{}", series_string) {
        eprintln!("Couldn't write to file: {}", e);
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
        tvfile
            .read_to_string(&mut tvfile_content)
            .expect("Unable to read file.");

        assert_eq!(tvfile_content, FILE_HEADERS);

        tear_down(filename);
    }
}

#[cfg(test)]
mod store_series_tests {
    use super::*;

    #[test]
    fn test_appends_tracker_string() {
        let test_file = "test_file.csv";
        let series_string = "1, title, latest\n";
        create_test_file(test_file);

        store_series("test_file.csv", 1, "title", "latest");
        let mut file = fs::File::open(test_file).expect("Unable to open file.");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Unable to read file.");

        assert_eq!(file_content, series_string);

        tear_down(test_file)
    }
}

fn create_test_file(filename: &str) {
    let path = Path::new(filename);
    let display = path.display();
    match fs::File::create(&path) {
        Ok(file) => file,
        Err(error) => {
            panic!(
                "There was a problem creating the file {}: {}",
                display,
                error.description()
            );
        }
    };
}

fn tear_down(filename: &str) -> () {
    match fs::remove_file(filename) {
        Ok(success) => return success,
        Err(error) => {
            panic!(error);
        }
    }
}

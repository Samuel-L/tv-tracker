mod tvfile;

fn main() {
    tvfile::create_tvfile(tvfile::FILENAME);

    tvfile::store_series(tvfile::FILENAME, 82, "Hello World", "2017-08-27");
    tvfile::store_series(tvfile::FILENAME, 83, "Game of Thrones", "2017-08-27");
    tvfile::store_series(tvfile::FILENAME, 84, "Game of Thrones", "2017-08-27");

    tvfile::remove_series(tvfile::FILENAME, 83);
}

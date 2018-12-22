mod tvfile;

fn main() {
    tvfile::create_tvfile(tvfile::FILENAME);

    tvfile::store_series(tvfile::FILENAME, 82, "Game of Thrones", "2017-08-27");
}

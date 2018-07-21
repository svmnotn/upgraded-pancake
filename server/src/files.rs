use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("front-end/build/index.html")
}

#[get("/<file..>")]
fn get(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("front-end/build").join(file)).ok()
}

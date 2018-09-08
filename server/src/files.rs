use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("front-end/build/index.html")
}

#[get("/<file..>", rank = 2)]
fn get(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("front-end/build").join(file)).ok()
}

use ::std::io;
use ::std::path::{Path, PathBuf};
use ::rocket::response::NamedFile;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("../app/public/index.html")
}

#[get("/<file..>", rank = 5)]
pub fn all(file: PathBuf) -> Option<NamedFile> {
    let file = NamedFile::open(Path::new("../app/public/").join(file)).ok();
    
    // FIXME: This is just a dirty fix.
    //        That route should just return static files from the public/ folder but
    //        sometimes the user enters a whole url for the spa for example .../posts
    //        In that case we want to return index.html where the spa lies
    if file.is_some() { file } else { NamedFile::open("../app/public/index.html").ok() }
}

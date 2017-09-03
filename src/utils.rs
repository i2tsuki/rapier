use std::fmt;
use std::io;
use std::process::{self, Command};

pub enum UtilsError {
    Io(io::Error),
}

impl From<io::Error> for UtilsError {
    fn from(err: io::Error) -> UtilsError {
        UtilsError::Io(err)
    }
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UtilsError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

pub fn xdg_open(url: String) -> Result<process::ExitStatus, UtilsError> {
    Ok(Command::new("xdg-open").arg(url).status()?)
}

pub fn human_url(url: String) -> String {
    let mut url = url.replace("api.github.com/repos", "github.com");
    url = url.replace("/pulls/", "/pull/");
    url
}

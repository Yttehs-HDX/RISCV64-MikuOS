use crate::{
    config::{CURRENT_DIR, DIR_SEPARATOR, ROOT_DIR},
    task,
};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Display;

// region Path begin
pub struct PathUtil(String);

impl Display for PathUtil {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PathUtil {
    pub fn from_str(s: &str) -> Self {
        // absolute path
        // add '/' if not exists
        let path = if s.starts_with(ROOT_DIR) {
            s.to_string()
        } else {
            format!("{}{}", ROOT_DIR, s)
        };
        PathUtil(path)
    }

    pub fn from_user(s: &str) -> Self {
        if s.starts_with(DIR_SEPARATOR) {
            // absolute path
            return Self::from_str(s);
        }

        // relative path
        // remove '.' if exists
        let path = s.strip_prefix(CURRENT_DIR).unwrap_or(s);
        // add '/' if not exists
        let path = if path.starts_with(DIR_SEPARATOR) {
            path.to_string()
        } else {
            format!("{}{}", DIR_SEPARATOR, path)
        };

        // construct absolute path
        let cwd = task::get_processor().current().inner().get_cwd();
        // no need to add cwd if path is root
        let path = if cwd == ROOT_DIR {
            path
        } else {
            format!("{}{}", cwd, path)
        };
        PathUtil(path)
    }

    pub fn split(&self) -> Vec<&str> {
        // root
        if self.0 == ROOT_DIR {
            return Vec::new();
        }
        self.0
            .split(DIR_SEPARATOR)
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn parent(&self) -> String {
        let mut split = self.split();
        if split.len() == 1 || split.len() == 0 {
            return ROOT_DIR.to_string();
        }
        split.pop();
        split.join(DIR_SEPARATOR)
    }

    pub fn name(&self) -> String {
        let mut split = self.split();
        if split.len() == 0 {
            return "".to_string();
        }
        split.pop().unwrap().to_string()
    }
}
// region Path end

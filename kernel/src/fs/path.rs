use crate::{
    config::{DIR_SEPARATOR, ROOT_DIR},
    task,
};
use alloc::{
    format,
    string::{String, ToString},
};

// region Path begin
pub struct Path {
    parent: String,
    name: String,
}

impl Path {
    pub fn new(parent: &str, name: &str) -> Self {
        let parent = parent.to_string();
        let name = name.to_string();
        Self { parent, name }
    }

    pub fn from_str(path: &str) -> Self {
        // remove "./"
        let path = path.strip_prefix("./").unwrap_or(path);
        // construct a path with leading '/'
        let path = if path.starts_with(DIR_SEPARATOR) {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        // split the path into parent and name
        if let Some(pos) = path.rfind(DIR_SEPARATOR) {
            let mut parent = &path[..pos];
            if parent.is_empty() {
                parent = ROOT_DIR;
            }
            let name = &path[pos + 1..];
            Self::new(parent, name)
        } else {
            Self::new(ROOT_DIR, &path)
        }
    }

    pub fn from_relative(path: &str) -> Self {
        let cwd = task::get_processor().current().inner().get_cwd();
        // construct a path with leading "./"
        let path = if path.starts_with("./") {
            path.to_string()
        } else {
            format!("./{}", path)
        };
        // replace "." with cwd
        let path = path.replace(".", &cwd);
        Self::from_str(&path)
    }

    pub fn parent(&self) -> &str {
        &self.parent
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.parent, self.name)
    }
}
// region Path end

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
        let path = if path.starts_with("./") {
            &path[2..]
        } else {
            path
        };
        // construct a path with leading '/'
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        // split the path into parent and name
        if let Some(pos) = path.rfind('/') {
            let mut parent = &path[..pos];
            if parent.is_empty() {
                parent = "/";
            }
            let name = &path[pos + 1..];
            Self::new(parent, name)
        } else {
            Self::new("/", &path)
        }
    }

    pub fn parent(&self) -> &str {
        &self.parent
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
// region Path end

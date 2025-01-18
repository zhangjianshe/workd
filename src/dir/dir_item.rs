#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct DirItem {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

impl DirItem {
    pub fn new(name: String, size: u64, is_dir: bool) -> DirItem {
        DirItem { name, size, is_dir }
    }
}

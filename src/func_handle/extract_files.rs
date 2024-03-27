use std::hash::Hash;

use slint::Model;

use crate::{CheckedItem, ExtractFilesRun};
pub struct ExtractFilesRunInfo {
    pub method: String,
    pub origin_path: String,
    pub need_target_path: bool,
    pub target_path: String,
    pub file_es: Vec<CheckedItem>,
    pub dele_origin: bool,
}

impl From<ExtractFilesRun> for ExtractFilesRunInfo {
    fn from(value: ExtractFilesRun) -> Self {
        Self {
            method: value.method.to_string(),
            origin_path: value.origin_path.to_string(),
            need_target_path: value.need_target_path,
            target_path: value.target_path.to_string(),
            file_es: value.file_es.iter().collect(),
            dele_origin: value.dele_origin,
        }
    }
}

impl Eq for CheckedItem {}

impl Hash for CheckedItem {
    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }
    
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.r#checked.hash(state);
        self.r#item.hash(state);
    }
}

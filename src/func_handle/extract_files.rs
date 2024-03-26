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
            file_es: vec![],
            dele_origin: value.dele_origin,
        }
    }
}

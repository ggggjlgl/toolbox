use std::{
    fs::{copy, rename},
    hash::Hash,
    path::{Path, PathBuf},
    str::FromStr,
};

use rfd::{MessageButtons, MessageLevel};
use slint::Model;

use crate::{CheckedItem, ExtractFilesRun};

use super::utils::{
    get_entry, handle_copy_move_files, pop_message, public_file_filter_ok, MessageInfo,
};

use walkdir::DirEntry;
pub struct ExtractFilesRunInfo {
    pub method: String,
    pub origin_path: String,
    pub target_path: String,
    pub file_es: Vec<CheckedItem>,
    pub dele_origin: bool,
}

impl From<ExtractFilesRun> for ExtractFilesRunInfo {
    fn from(value: ExtractFilesRun) -> Self {
        Self {
            method: value.method.to_string(),
            origin_path: value.origin_path.to_string(),
            target_path: value.target_path.to_string(),
            file_es: value.file_es.iter().collect(),
            dele_origin: value.dele_origin,
        }
    }
}

impl ExtractFilesRunInfo {
    pub fn handle_run(&mut self) {
        if let Err(e) = self.check() {
            pop_message(e);
            return;
        }
        let files = self.collect_files();
        match self.run(files) {
            Err(e) => {
                pop_message(e);
            }
            Ok(_) => {
                pop_message(MessageInfo {
                    desc: "命令成功执行完毕".to_string(),
                    title: "提示".to_string(),
                    level: MessageLevel::Info,
                    buttons: MessageButtons::Ok,
                });
            }
        }
    }

    fn need_target_path(&self) -> bool {
        self.method == "提取文件至指定目录下"
    }

    fn check(&mut self) -> Result<(), MessageInfo> {
        if self.origin_path.is_empty() || !Path::new(self.origin_path.as_str()).is_dir() {
            return Err("请指定有效源路径。".into());
        }
        if self.need_target_path()
            && (self.target_path.is_empty() || !Path::new(self.target_path.as_str()).is_dir())
        {
            return Err("当前提取方式需要指定有效目标路径。".into());
        }
        Ok(())
    }

    fn collect_files(&self) -> Vec<PathBuf> {
        let mut extensions = Vec::new();
        for es in &self.file_es {
            match es {
                CheckedItem { item, checked } => {
                    if checked.to_owned() {
                        extensions.push(item.to_string())
                    }
                }
            }
        }
        if extensions.is_empty() {
            let is_target = |entry: &DirEntry| {
                public_file_filter_ok(entry) && self.origin_not_same_to_des(entry)
            };
            get_entry(&self.origin_path, is_target)
        } else {
            let is_target = |entry: &DirEntry| {
                public_file_filter_ok(entry)
                    && self.origin_not_same_to_des(entry)
                    && extensions.contains(
                        &entry
                            .path()
                            .extension()
                            .unwrap_or("".as_ref())
                            .to_str()
                            .unwrap()
                            .to_string(),
                    )
            };
            get_entry(&self.origin_path, is_target)
        }
    }

    fn origin_not_same_to_des(&self, entry: &DirEntry) -> bool {
        let origin_path = entry.path().parent().unwrap();
        if self.need_target_path() {
            origin_path
                != PathBuf::from_str(self.target_path.as_str())
                    .unwrap()
                    .as_path()
        } else {
            origin_path
                != PathBuf::from_str(self.origin_path.as_str())
                    .unwrap()
                    .as_path()
        }
    }

    fn run(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        match self.method.as_str() {
            "提取文件至源目录下" => self.handle_detail(files, &self.origin_path),
            "提取文件至指定目录下" => self.handle_detail(files, &self.target_path),
            _ => Ok(()),
        }
    }

    fn handle_detail(&self, files: Vec<PathBuf>, des: &String) -> Result<(), MessageInfo> {

        match self.dele_origin {
            true => handle_copy_move_files(files, rename, |file| {
                PathBuf::from_str(des.as_str())
                    .unwrap()
                    .join(file.file_name().unwrap())
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
            false => handle_copy_move_files(files, copy, |file| {
                PathBuf::from_str(des.as_str())
                    .unwrap()
                    .join(file.file_name().unwrap())
                    .to_str()
                    .unwrap()
                    .to_string()
            }),
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

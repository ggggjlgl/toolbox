use std::path::{Path, PathBuf};
use rfd::{MessageButtons, MessageLevel};
use super::utils::{MessageInfo, move_files, get_entry_target, get_entry, pop_message};
use crate::RenameFilesRun;


pub struct RenameFilesRunInfo {
    pub method: String,
    pub path: String,
    pub specify_characters: String,
}

impl From<RenameFilesRun> for RenameFilesRunInfo {
    fn from(value: RenameFilesRun) -> Self {
        Self {
            method: value.method.to_string(),
            path: value.path.to_string(),
            specify_characters: value.specify_characters.to_string(),
        }
    }
}


impl RenameFilesRunInfo {
    fn check(&self) -> Result<(), MessageInfo> {
        if self.specify_characters.is_empty() {
            return Err("请指定字符。".into());
        }
        if self.is_replace() && (!self.specify_characters.contains("-批量替换至-")) {
            return Err("替换字符类方法须同时提供旧字符和新字符，并使用'-批量替换至-'连接。".into());
        }
        if !Path::new(self.path.as_str()).is_dir() {
            return Err("提供的路径不是有效目录".into());
        }
        Ok(())
    }

    fn collect_files(&self) -> Vec<PathBuf> {
        match self.method.as_str() {
            "批量移除指定字符" => {
                let is_target = |entry: &_|
                    get_entry_target(entry).contains(&self.specify_characters.as_str());
                get_entry(
                    &self.path,
                    is_target,
                )
            }
            "批量移除指定前缀" | "批量替换指定前缀" => {
                let is_target = |entry: &_|
                    get_entry_target(entry).starts_with(&self.specify_characters.as_str());
                get_entry(
                    &self.path,
                    is_target,
                )
            }
            "批量移除指定后缀" | "批量替换指定后缀" => {
                let is_target = |entry: &_|
                    get_entry_target(entry).ends_with(&self.specify_characters.as_str());
                get_entry(
                    &self.path,
                    is_target,
                )
            }
            "批量新增指定前缀" | "批量新增指定后缀" => {
                get_entry(
                    &self.path,
                    |_entry: &_| true,
                )
            }
            _ => {vec![]}
        }
    }

    fn run(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        match self.method.as_str() {
            "批量替换指定前缀" | "批量替换指定后缀" => {
                let mut split_ = self.specify_characters.split("-批量替换至-");
                let old_str: &str = split_.next().unwrap_or_default();
                let new_str: &str = split_.next().unwrap_or_default();
                if old_str.is_empty() || new_str.is_empty()
                { return Err("提供的字符信息不足".into()); }
            }
            "批量移除指定字符" => {
                return self.handle_remove_specify_characters(files)
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_run(&self) {
        if let Err(e) = self.check() {
            pop_message(e);
            return
        }
        let files = self.collect_files();
        match self.run(files) {
            Err(e) => {pop_message(e);}
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

    fn handle_remove_specify_characters(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        match move_files(
            files,
            |file| {
                let mut file_c = file.clone();
                file_c.set_file_name(
                    file
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(self.specify_characters.as_str(), "")
                );
                file_c.set_extension(
                    file
                        .extension()
                        .unwrap()
                );
                file_c
            },
        ) {
            Ok(_) => {Ok(())}
            Err(v) => {
                Err(v.join(",").as_str().into())
            }
        }
    }

    fn is_replace(&self) -> bool {
        self.method == "批量替换指定前缀" || self.method == "批量替换指定后缀"
    }


}
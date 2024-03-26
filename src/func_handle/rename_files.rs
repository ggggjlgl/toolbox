use super::utils::{get_entry, get_entry_target, move_files, pop_message, MessageInfo};
use crate::RenameFilesRun;
use rfd::{MessageButtons, MessageLevel};
use std::path::{Path, PathBuf};

pub struct RenameFilesRunInfo {
    pub method: String,
    pub path: String,
    pub specify_characters: String,
    pub old_str: String,
    pub new_str: String,
}

impl From<RenameFilesRun> for RenameFilesRunInfo {
    fn from(value: RenameFilesRun) -> Self {
        Self {
            method: value.method.to_string(),
            path: value.path.to_string(),
            specify_characters: value.specify_characters.to_string(),
            old_str: String::new(),
            new_str: String::new(),
        }
    }
}

impl RenameFilesRunInfo {
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

    fn handle_names<F>(&self, files: Vec<PathBuf>, f: F) -> Result<(), MessageInfo>
        where F: Fn(&PathBuf) -> String,
    {
        match move_files(files, |file| {
            let mut file_c = file.clone();
            file_c.set_file_name(
                f(file)
            );
            if let Some(es) = file.extension() {
                file_c.set_extension(es);
            }
            file_c
        }) {
            Ok(_) => Ok(()),
            Err(v) => Err(v.join(",").as_str().into()),
        }
    }

    fn check(&mut self) -> Result<(), MessageInfo> {
        if self.specify_characters.is_empty() {
            return Err("请指定字符。".into());
        }
        if self.is_replace() && (!self.specify_characters.contains("-批量替换至-")) {
            return Err(
                "替换字符类方法须同时提供旧字符和新字符，并使用'-批量替换至-'连接。".into(),
            );
        } else if self.is_replace() {
            let v: Vec<&str> = self.specify_characters.split("-批量替换至-").collect();
            if v.len() != 2 { return Err("提供的字符信息不足以同时提取新旧字符串".into()); }
            if let [old_str, new_str] = v[..] {
                if old_str.is_empty() || new_str.is_empty() {
                    return Err("提供的字符信息不足以同时提取新旧字符串".into());
                }
                self.old_str.push_str(old_str);
                self.new_str.push_str(new_str);
            } else {
                return Err("提供的字符信息不足以同时提取新旧字符串".into());
            }

        }
        if !Path::new(self.path.as_str()).is_dir() {
            return Err("提供的路径不是有效目录".into());
        }
        Ok(())
    }

    fn collect_files(&self) -> Vec<PathBuf> {
        match self.method.as_str() {
            "批量移除指定字符" => {
                let is_target = |entry: &_| {
                    get_entry_target(entry, |entry_str| entry_str.contains(self.specify_characters.as_str()))
                };
                get_entry(&self.path, is_target)
            }
            "批量移除指定前缀" => {
                let is_target = |entry: &_| {
                    get_entry_target(entry, |entry_str| entry_str.starts_with(self.specify_characters.as_str()))
                };
                get_entry(&self.path, is_target)
            }
            "批量移除指定后缀" => {
                let is_target = |entry: &_| {
                    get_entry_target(entry, |entry_str| entry_str.ends_with(self.specify_characters.as_str()))
                };
                get_entry(&self.path, is_target)
            }
            "批量新增指定前缀" | "批量新增指定后缀" => {
                get_entry(&self.path, |entry: &_| get_entry_target(entry, |_entry_str| true))
            }

            "批量替换指定前缀" => {
                let is_target = |entry: &_| {
                    get_entry_target(entry, |entry_str| entry_str.starts_with(self.old_str.as_str()))
                };
                get_entry(&self.path, is_target)
            }
            "批量替换指定后缀" => {
                let is_target = |entry: &_| {
                    get_entry_target(entry, |entry_str| entry_str.ends_with(self.old_str.as_str()))
                };
                get_entry(&self.path, is_target)
            }

            _ => {
                vec![]
            }
        }
    }
    fn prevent_accidental_deletion(s: String, p: &PathBuf) -> String {
        if s.len() == 0 || s.trim_start_matches(".") == "" {
            p.clone().to_string_lossy().to_string()
        } else {
            s
        }
    }
    fn handle_remove_specify_characters(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let tmp = file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replacen(self.specify_characters.as_str(), "", 1);
            Self::prevent_accidental_deletion(tmp, file)
        })
    }

    fn handle_replace_pre_by_specify(&self, files: Vec<PathBuf>, old_str: &str, new_str: &str) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .replacen(old_str, new_str, 1)
        })
    }

    fn handle_replace_suf_by_specify(&self, files: Vec<PathBuf>, old_str: &str, new_str: &str) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let mut tmp = file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(old_str)
                .unwrap()
                .to_string();
            tmp.push_str(new_str);
            tmp
        })
    }

    fn handle_remove_specify_pre(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let tmp = file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_prefix(self.specify_characters.as_str())
                .unwrap()
                .to_string();
            Self::prevent_accidental_deletion(tmp, file)
        })
    }

    fn handle_remove_specify_suf(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let tmp = file.file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .strip_suffix(self.specify_characters.as_str())
                .unwrap()
                .to_string();
            Self::prevent_accidental_deletion(tmp, file)
        })
    }

    fn handle_add_specify_pre(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let mut tmp = String::from(self.specify_characters.as_str());
            tmp.push_str(
                file.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            tmp
        })
    }

    fn handle_add_specify_suf(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        self.handle_names(files, |file| {
            let mut tmp = String::from(
                file.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
            );
            tmp.push_str(self.specify_characters.as_str());
            tmp
        })
    }


    fn run(&self, files: Vec<PathBuf>) -> Result<(), MessageInfo> {
        match self.method.as_str() {
            "批量移除指定字符" => self.handle_remove_specify_characters(files),
            "批量替换指定前缀" | "批量替换指定后缀" => {
                match self.method.as_str() {
                    "批量替换指定前缀" => {
                        self.handle_replace_pre_by_specify(files, self.old_str.as_str(), self.new_str.as_str())
                    }
                    "批量替换指定后缀" => {
                        self.handle_replace_suf_by_specify(files, self.old_str.as_str(), self.new_str.as_str())
                    }
                    _ => Ok(())
                }
            }
            "批量移除指定前缀" => self.handle_remove_specify_pre(files),
            "批量移除指定后缀" => self.handle_remove_specify_suf(files),
            "批量新增指定前缀" => self.handle_add_specify_pre(files),
            "批量新增指定后缀" => self.handle_add_specify_suf(files),
            _ => Ok(())
        }
        // Ok(())
    }


    fn is_replace(&self) -> bool {
        self.method == "批量替换指定前缀" || self.method == "批量替换指定后缀"
    }
}

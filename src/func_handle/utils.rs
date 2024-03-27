use std::collections::HashSet;
use std::fs::{copy, rename};
use std::io;
use std::path::{Path, PathBuf};

use rfd::{FileDialog, MessageButtons, MessageDialogResult, MessageLevel};
use slint::SharedString;
use walkdir::{DirEntry, WalkDir};

use crate::CheckedItem;

pub fn pick_dir() -> String {
    if let Some(pb) = FileDialog::new().set_directory("/").pick_folder() {
        pb.to_string_lossy().into_owned()
    } else {
        "".to_string()
    }
}

pub fn get_entry<P, F>(dir: P, is_target: F) -> Vec<PathBuf>
where
    P: AsRef<Path>,
    F: Fn(&DirEntry) -> bool,
{
    let mut entries = Vec::new();
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|file_res| file_res.ok())
        .filter(|entry| is_target(entry))
        .for_each(|file| entries.push(file.into_path()));
    entries
}

pub fn copy_files<C>(files: Vec<PathBuf>, creat: C) -> Result<(), Vec<String>>
where
    C: Fn(&PathBuf) -> PathBuf,
{
    handle_files(files, copy, creat)
}

pub fn move_files<C>(files: Vec<PathBuf>, creat: C) -> Result<(), Vec<String>>
where
    C: Fn(&PathBuf) -> PathBuf,
{
    handle_files(files, rename, creat)
}

fn handle_files<M, C, X>(files: Vec<PathBuf>, method: M, creat: C) -> Result<(), Vec<String>>
where
    M: Fn(PathBuf, PathBuf) -> Result<X, io::Error>,
    C: Fn(&PathBuf) -> PathBuf,
{
    let mut errors = Vec::new();
    for file in files {
        let new_file = creat(&file);
        if let Err(e) = method(file.clone(), new_file.clone()) {
            errors.push(format!(
                "{}{}to{}failed: {}",
                "copy/move",
                file.to_str().unwrap(),
                new_file.to_str().unwrap(),
                e.to_string()
            ))
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

pub struct MessageInfo {
    pub desc: String,
    pub title: String,
    pub level: MessageLevel,
    pub buttons: MessageButtons,
}

impl From<&str> for MessageInfo {
    // 特殊实现，只接管仅提供了desc(错误信息)的MessageInfo
    fn from(s: &str) -> Self {
        Self {
            desc: s.to_string(),
            title: "错误".to_string(),
            level: MessageLevel::Error,
            buttons: MessageButtons::Ok,
        }
    }
}

pub fn pop_message(mes_info: MessageInfo) -> MessageDialogResult {
    rfd::MessageDialog::new()
        .set_description(mes_info.desc)
        .set_title(mes_info.title)
        .set_level(mes_info.level)
        .set_buttons(mes_info.buttons)
        .show()
}

pub fn get_entry_target<F>(entry: &DirEntry, f: F) -> bool
where
    F: Fn(&str) -> bool,
{
    if (!entry.path().is_file()) ||
     entry.file_name().to_str().unwrap().starts_with(".") ||
     (entry.path().extension().is_some() && entry.file_name() == entry.path().extension().unwrap())
    {
        return false;
    }

    f(entry
        .path()
        .file_stem()
        .unwrap_or("".as_ref())
        .to_str()
        .unwrap())
}

pub fn get_extensions_by_dir(dir: String) -> Vec<CheckedItem> {
    let mut tmp = HashSet::new();

    WalkDir::new(dir)
        .into_iter()
        .filter_map(|file_res| file_res.ok())
        .filter(
            |entry| entry.path().is_file() 
            && (!entry.file_name().to_str().unwrap().starts_with(".") &&
            entry.path().extension().is_some() && (entry.file_name() != entry.path().extension().unwrap())
        )
        )
        .for_each(|file| {
            tmp.insert(
                CheckedItem {
                    checked: false,
                    item: SharedString::from(
                        file
                        .path()
                        .extension().unwrap()
                        .to_str().unwrap()
                    ),
                }
            );
        });
    let extensions: Vec<CheckedItem> = tmp.into_iter().collect();
    extensions
}

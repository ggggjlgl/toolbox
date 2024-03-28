use std::collections::HashSet;
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



pub fn handle_copy_move_files<F, M, X>(
    files: Vec<PathBuf>,
    method: M,
    f: F,
) -> Result<(), MessageInfo>
where
    F: Fn(&PathBuf) -> String,
    M: Fn(PathBuf, PathBuf) -> Result<X, io::Error>,
{
    let mut new_files = Vec::new();
    files.iter().for_each(|file| {
        let mut file_c = file.clone();
        let mut new_file_name = f(file);
        file_c.set_file_name(new_file_name.as_str());
        if let Some(es) = file.extension() {
            file_c.set_extension(es);
        }
        if file_c.is_file() || new_files.contains(&file_c) {
            new_file_name.push_str(
                format!(
                    "来自{}",
                    file.to_str().unwrap().replace("/", "-").replace("\\", "-")
                )
                .as_str(),
            );
            file_c.set_file_name(new_file_name);
            if let Some(es) = file.extension() {
                file_c.set_extension(es);
            }
        }
        new_files.push(file_c)
    });
    match handle_files(files, method, new_files) {
        Ok(_) => Ok(()),
        Err(v) => Err(v.join(",").as_str().into()),
    }
}



fn handle_files<M, X>(
    files: Vec<PathBuf>,
    method: M,
    new_files: Vec<PathBuf>,
) -> Result<(), Vec<String>>
where
    M: Fn(PathBuf, PathBuf) -> Result<X, io::Error>,

{
    assert_eq!(files.len(), new_files.len());
    let mut errors = Vec::new();
    for (index, file_owned) in files.into_iter().enumerate() {
        let new_file = new_files.get(index).unwrap();
        let new_file_owned = new_file.clone();
        let file = file_owned.clone();
        if let Err(e) = method(file_owned, new_file_owned) {
            errors.push(format!(
                "{}{}to{}failed: {} \n",
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

pub fn get_target_str_ok<F>(entry: &DirEntry, f: F) -> bool
where
    F: Fn(&str) -> bool,
{

    if public_file_filter_ok(entry) {
        f(entry
            .path()
            .file_stem()
            .unwrap_or("".as_ref())
            .to_str()
            .unwrap())
    } else {
        false
    }
}

pub fn public_file_filter_ok(entry: &DirEntry) -> bool {
    if (!entry.path().is_file())
        || entry.file_name().to_str().unwrap().starts_with(".")
        || (entry.path().extension().is_some()
            && entry.file_name() == entry.path().extension().unwrap())
    {
        false
    } else {
        true
    }
}


pub fn get_extensions_by_dir(dir: String) -> Vec<CheckedItem> {
    let mut tmp = HashSet::new();

    WalkDir::new(dir)
        .into_iter()
        .filter_map(|file_res| file_res.ok())
        .filter(|entry| {
            entry.path().is_file()
                && (!entry.file_name().to_str().unwrap().starts_with(".")
                    && entry.path().extension().is_some()
                    && (entry.file_name() != entry.path().extension().unwrap()))
        })
        .for_each(|file| {
            tmp.insert(CheckedItem {
                checked: false,
                item: SharedString::from(file.path().extension().unwrap().to_str().unwrap()),
            });
        });
    let extensions: Vec<CheckedItem> = tmp.into_iter().collect();
    extensions
}

use slint::SharedString;

pub mod func_handle;

use func_handle::utils;

use func_handle::rename_files::RenameFilesRunInfo;
use func_handle::extract_files::ExtractFilesRunInfo;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();

    // let func: Vec<FuncInfo> = main_window.get_func_infos().iter().collect();
    // let func_model = std::rc::Rc::new(slint::VecModel::from(func));
    // main_window.set_func_infos(func_model.clone().into());

    // let main_window_weak = main_window.as_weak();
    // main_window.on_click_func(move |func_name| {
    //     func_model.iter().enumerate().for_each(|model| {
    //         let (idx, mut info) = model;
    //         if info.name == func_name {
    //             info.is_cur = true
    //         } else {
    //             info.is_cur = false
    //         }
    //         func_model.set_row_data(idx, info)
    //     });

    //     match func_name.into() {
    //         FuncName::RenameFiles => {
    //             println!("批量修改文件名")
    //         }
    //         FuncName::ExtractFiles => {
    //             println!("批量提取文件")
    //         }
    //         FuncName::CompareClipBoard => {
    //             println!("对比剪贴板")
    //         }
    //         FuncName::PushFiles => {
    //             println!("安卓传输文件")
    //         }
    //     }
    // });

    main_window.on_select_dir(|| SharedString::from(utils::pick_dir()));

    main_window.on_run_rename_files(|run_info| {
        let mut run_info = RenameFilesRunInfo::from(run_info);
        run_info.handle_run();
    });

    main_window.on_run_extract_files(|run_info| {
        let _run_info = ExtractFilesRunInfo::from(run_info);
    });

    main_window.run().unwrap();
}

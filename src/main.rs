use slint::SharedString;

pub mod func_handle;

use func_handle::utils;

use func_handle::extract_files::ExtractFilesRunInfo;
use func_handle::rename_files::RenameFilesRunInfo;

slint::include_modules!();

fn main() {
    let main_window = MainWindow::new().unwrap();



    main_window.on_select_dir(|| SharedString::from(utils::pick_dir()));
    main_window.on_get_extensions(|dir| {
        slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(
            utils::get_extensions_by_dir(dir.to_string()),
        )))
    });

    main_window.on_run_rename_files(|run_info| {
        let mut run_info = RenameFilesRunInfo::from(run_info);
        run_info.handle_run();
    });

    main_window.on_run_extract_files(|run_info| {
        let mut run_info = ExtractFilesRunInfo::from(run_info);
        run_info.handle_run();
    });

    main_window.run().unwrap();
}

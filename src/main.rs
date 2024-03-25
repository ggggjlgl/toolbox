use slint::{Model, SharedString};

slint::include_modules!();



fn main() {
    let main_window = MainWindow::new().unwrap();

    let funcs: Vec<FuncInfo> = main_window.get_funcs().iter().collect();
    let funcs_model = std::rc::Rc::new(slint::VecModel::from(funcs));
    main_window.set_funcs(funcs_model.clone().into());

    // let main_window_weak = main_window.as_weak();
    main_window.on_click_func(move |func_name| {
        funcs_model.iter().enumerate().for_each(|func_model| {
            let (idx, mut info) = func_model;
            if info.name == func_name {
                info.is_cur = true
            } else {
                info.is_cur = false
            }
            funcs_model.set_row_data(idx, info)
            // funcs_model.set_row_data(idx, info);
        });

        match func_name.into() {
            FuncName::RenameFiles => {
                // let mut func = funcs_model.iter().enumerate().filter(|(_, func)| func.name.into() == Funcs::RenameFiles);
                println!("批量修改文件名")
            }
            FuncName::ExtractFiles => {
                println!("批量提取文件")
            }
            FuncName::CompareClipBoard => {
                println!("对比剪贴板")
            }
            FuncName::PushFiles => {
                println!("安卓传输文件")
            }
        }
    });

    main_window.on_select_path( || {
        return SharedString::from("测试");
    });

    main_window.run().unwrap();
}

fn main() {
    slint_build::compile("components/common.slint").unwrap();
    slint_build::compile("components/models.slint").unwrap();
    slint_build::compile("components/rename_files_panel.slint").unwrap();
    slint_build::compile("components/extract_files_panel.slint").unwrap();
    slint_build::compile("components/func_list_panel.slint").unwrap();
    slint_build::compile("components/window.slint").unwrap();
}

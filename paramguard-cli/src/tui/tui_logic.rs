use crate::tui::components::editor::Editor;

pub fn open_editor(path: String, name: String) {
    let file_path = format!("{path}/{name}");
    let open_editor = Box::new(|initial_content: String| {
        let mut editor = Editor::new(initial_content, &file_path);
        editor.run()
    });

    drop(open_editor);
}

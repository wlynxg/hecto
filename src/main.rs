use std::io::Read;

use crate::editor::Editor;

mod editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}

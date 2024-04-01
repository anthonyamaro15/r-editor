mod editor;
use crate::editor::Editor;

// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::new();
    let _ = editor.init();
    Ok(())
}

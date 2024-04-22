mod buffer;
mod editor;
use crate::buffer::Buffer;
use crate::editor::Editor;
// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

fn main() -> anyhow::Result<()> {
    let buffer = Buffer::new("src/buffer.rs");
    let mut editor = Editor::new(buffer);
    let _ = editor.init();
    Ok(())
}

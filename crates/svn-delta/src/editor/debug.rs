//! `debug_editor.c`
//! An editor that writes the operations it does to stderr.

use svn_types::RevisionNumber;

use super::{DeltaEditor, EditorError};

pub struct EditorBaton {
    wrapped_editor: Option<Box<dyn DeltaEditor>>,

    indent_level: usize,

    prefix: String,
}

impl DeltaEditor for EditorBaton {
    fn set_target_revision(&self, target_revision: RevisionNumber) -> Result<(), EditorError> {
        todo!()
    }

    fn add_item(
        &mut self,
        path: &std::path::Path,
        parent_baton: (),
        conpyfrom_path: &std::path::Path,
        copyfrom_revison: RevisionNumber,
    ) -> Result<(), EditorError> {
        todo!()
    }

    fn delete_entry(
        &mut self,
        path: &std::path::Path,
        revision: RevisionNumber,
        parent_baton: (),
    ) -> Result<(), EditorError> {
        self.write_indent()?;
        println!("delete_entry: {} at revision {}", path.display(), revision);
        self.indent_level += 1;

        if let Some(editor) = &self.wrapped_editor {
            editor.delete_entry(path, revision, parent_baton)?;
        }

        self.indent_level -= 1;
        Ok(())
    }

    fn open_item(&self, path: &std::path::Path, parent_baton: ()) -> Result<(), EditorError> {
        todo!()
    }

    fn change_prop(&self, file_baton: (), name: &str, value: String) -> Result<(), EditorError> {
        todo!()
    }

    fn open_root(&mut self, base_revison: RevisionNumber) -> Result<(), EditorError> {
        self.write_indent()?;
        println!("open_root: {base_revison}");
        self.indent_level += 1;

        if let Some(editor) = &self.wrapped_editor {
            editor.open_root(base_revison)?;
        }

        Ok(())
    }
}

impl EditorBaton {
    fn write_indent(&self) -> Result<(), EditorError> {
        for _ in 0..self.indent_level {
            print!(" ");
        }
        Ok(())
    }
}

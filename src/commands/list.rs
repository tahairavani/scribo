//function with run name
use crate::note::Note;
use crate::Table;
use crate::cprintln;

pub fn run() {
    match Note::load_from_json() {
        Some(notes) => {
            let mut table = Table::new();
            table.set_header(vec!["id", "content", "tag", "date","time"]);
            if (notes.len() as isize) == 0 {
                cprintln!("<red>No notes found.</red>\n<yellow>Add one with:</yellow><green> notes add 'your first note'</green>")
            }
            for note in &notes {
                table.add_row(vec![
                    note.id.to_string(),
                    note.content.to_string(),
                    note.tag.to_string(),
                    note.date.to_string(),
                    note.time.to_string(),
                ]);
            }
            println!("{table}");
        }
        None => {
            cprintln!("<red>the spicific filer couldn't be found!</red>")
        }
    }
}
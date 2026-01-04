use crate::cprintln;
use crate::Note;
pub fn run(id : u32) {
    match Note::load_from_json() {
        Some(mut notes) => {
            notes.retain(|n| n.id != id);
            match Note::save_to_json(&notes) {
                Some(_) => cprintln!("<green>note {id} deleted!</green>"),
                None => cprintln!("<red>Failed to delete note {id}.</red>")
            }
        },
        None => cprintln!("<red>No notes found!</red>")
    }
}

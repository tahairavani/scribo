use crate::note::Note;
use crate::cprintln;
pub fn run(content: String, tag: String) {
    match Note::load_from_json() {
        Some(n) => {
            let mut notes = n.clone();
            notes.push(Note::new(content, tag));
            match Note::save_to_json(&notes) {
                Some(_) => cprintln!("<green>note added!</green>"),
                None =>  cprintln!("<red>note couldn't be added !</red>")
            }
        }
        None => cprintln!("<red>the specific filer couldn't be found</red>")
    }

}

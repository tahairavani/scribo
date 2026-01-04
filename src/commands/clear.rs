use crate::Note;
use crate::cprintln;


pub fn run(yes: bool) {
            if yes {
                match Note::save_to_json(&vec![]) {
                    Some(_) => cprintln!("<green>all notes cleared!</green>"),
                    None => cprintln!("<red>Failed to clear notes.</red>")
                }
            } else {
                cprintln!("<yellow>use --yes tag to confirm clear notes</yellow>")
            }
}
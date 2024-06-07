use std::io::{self, Write};

#[derive(Debug)]
struct Note {
    id: usize,
    title: String,
    content: String,
}

fn main() {
    let mut notes = Vec::new();
    println!("Welcome to the Notes Manager!");
    loop {
        println!("\n1. Create a new note");
        println!("2. View all notes");
        println!("3. Edit a note");
        println!("4. Delete a note");
        println!("5. Save notes to file");
        println!("6. Load notes from file");
        println!("7. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: usize = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => create_note(&mut notes),
            2 => view_notes(&notes),
            3 => edit_note(&mut notes),
            4 => delete_note(&mut notes),
            5 => save_notes_to_file(&notes),
            6 => load_notes_from_file(&mut notes),
            7 => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn create_note(notes: &mut Vec<Note>) {
    let id = notes.len() + 1;
    let mut title = String::new();
    let mut content = String::new();

    print!("Enter note title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();

    print!("Enter note content: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).unwrap();

    notes.push(Note {
        id,
        title: title.trim().to_string(),
        content: content.trim().to_string(),
    });

    println!("Note created successfully!");
}

fn view_notes(notes: &[Note]) {
    for note in notes {
        println!("\nID: {}", note.id);
        println!("Title: {}", note.title);
        println!("Content: {}", note.content);
    }
}

fn edit_note(notes: &mut Vec<Note>) {
    let mut id = String::new();

    print!("Enter note ID to edit: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();
    let id: usize = id.trim().parse().unwrap_or(0);

    if let Some(note) = notes.iter_mut().find(|note| note.id == id) {
        let mut title = String::new();
        let mut content = String::new();

        print!("Enter new title: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut title).unwrap();

        print!("Enter new content: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut content).unwrap();

        note.title = title.trim().to_string();
        note.content = content.trim().to_string();

        println!("Note updated successfully!");
    } else {
        println!("Note with ID {} not found.", id);
    }
}

fn delete_note(notes: &mut Vec<Note>) {
    let mut id = String::new();

    print!("Enter note ID to delete: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();
    let id: usize = id.trim().parse().unwrap_or(0);

    if let Some(pos) = notes.iter().position(|note| note.id == id) {
        notes.remove(pos);
        println!("Note deleted successfully!");
    } else {
        println!("Note with ID {} not found.", id);
    }
}

fn save_notes_to_file(notes: &[Note]) {
    use std::fs::File;
    use std::io::prelude::*;

    let file_path = "notes.txt";
    let mut file = File::create(file_path).expect("Could not create file");

    for note in notes {
        writeln!(file, "{}|{}|{}", note.id, note.title, note.content).expect("Could not write to file");
    }

    println!("Notes saved to {}", file_path);
}

fn load_notes_from_file(notes: &mut Vec<Note>) {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    let file_path = "notes.txt";
    if !Path::new(file_path).exists() {
        println!("No saved notes found.");
        return;
    }

    let file = File::open(file_path).expect("Could not open file");
    let reader = io::BufReader::new(file);

    notes.clear();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 3 {
            let id: usize = parts[0].parse().unwrap_or(0);
            let title = parts[1].to_string();
            let content = parts[2].to_string();
            notes.push(Note { id, title, content });
        }
    }

    println!("Notes loaded from {}", file_path);
}

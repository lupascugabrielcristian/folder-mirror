extern crate xml;

use std::fs;
use std::path::Path;
use std::env;
use std::any::type_name;
use std::fs::File;
use xml::writer::EventWriter;

fn main() {
    // TODO
    // Sa verific daca am un argument

    let args: Vec<String> = env::args().collect();
    let main_dir = &args[1];

    // Verific daca exista directorul
    if !Path::new(main_dir).exists() {
        println!("Cannot find specified path");
        return;
    }

    // Make output XML file
    let mut output_file = File::create("output.xml").unwrap();

    // Make the XML writer
    let mut writer = EventWriter::new();

    let p = Path::new(main_dir);
    print_dir_contents(p);
}

// Prints the type of a variable
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn print_dir_contents(path: &Path) {

    if path.is_dir() {
        println!("[DIRECTORY] {}", path.display());

        // read_dir returns Result<ReadDir>
        // paths is a ReadDir. ReadDir is Iterator over the entries in a directory
        let paths = path.read_dir().unwrap();

        for new_path in paths {
            // new_path is Result<std::fs::DirEntry, std::io::error::Error>

            // p is PathBuf
            let p = new_path.unwrap().path();

            // file_name takes the last component in the Path
            let file_name = &p.as_path().file_name().unwrap()
                .to_str().unwrap();

            if file_name.contains("node_modules") ||
                file_name.contains(".git") || 
                file_name.contains("__pycache__"){
                continue
            }

            print_dir_contents( &p.as_path() );
        }
    } else {
        let file_name = &path.file_name().unwrap()
                .to_str().unwrap();

        println!("[FILE] {}", file_name);
    }
}

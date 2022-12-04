extern crate quick_xml;

use std::{fs,str};
use std::path::Path;
use std::env;
use std::any::type_name;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::writer::Writer;
use quick_xml::reader::Reader;


fn main() {
    // TODO
    // Sa verific daca am un argument

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Pass the location of the root folder as first parameter");
        return;
    }

    if &args[1] == "import" {
        if args.len() >= 2 {
            println!("Importing {}", &args[2]);
            import_file(&args[2]);
        }
        else {
            println!("Need file name to import");
        }
        return;
    }

    let main_dir = &args[1];

    // Verific daca exista directorul
    if !Path::new(main_dir).exists() {
        println!("Cannot find specified path");
        return;
    }

    let mut file_count = 0;
    let mut dir_count = 0;

    // Creez xml writer
    let mut buffer = Vec::<u8>::new();
    let mut writer_indent = Writer::new_with_indent(&mut buffer, b' ', 4);


    let p = Path::new(main_dir);
    print_dir_contents(p, &mut writer_indent, &mut file_count, &mut dir_count );

    println!("{} files", file_count);
    println!("{} directories", dir_count);

    // Vec<utf8> in str
    //let str_result = str::from_utf8(&result);
    let str_result = str::from_utf8(&buffer);
    let output_path = Path::new("output.xml");
    let write_result = fs::write(&output_path, str_result.unwrap());
    match write_result {
        Ok(()) => println!("output.xml written"),
        Err(_) => println!("Not able to write output file"),
    }
}

// Prints the type of a variable
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn print_dir_contents(path: &Path, writer: &mut Writer<&mut Vec<u8>>, file_count: &mut i32, dir_count: &mut i32) {

    if path.is_dir() {
        *dir_count += 1;

        // dir_name este str
        let dir_name = path.file_name().unwrap()
                .to_str().unwrap();

        // creez un element de tip <my_elem> si ii pun in buffer inceputul
        let mut elem = BytesStart::new(dir_name);
        elem.push_attribute(("type", "directory"));
        match writer.write_event(Event::Start(elem)) {
            Ok(_) => (),
            Err(_) => return,
        }

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

            // Skip certain folder names
            if file_name.contains("node_modules") ||
                file_name.contains(".git") || 
                file_name.contains("__pycache__") ||
                file_name.contains("build") {
                continue
            }

            print_dir_contents( &p.as_path(), writer, file_count, dir_count );
        }

        // pun in buffer sfarsitul unui element
        let end_dir_elem = BytesEnd::new(dir_name);
        match writer.write_event(Event::End(end_dir_elem)) {
            Ok(_) => (),
            Err(_) => return,
        }

    } else {
        *file_count += 1;

        let file_name = &path.file_name().unwrap()
                .to_str().unwrap();

        match writer.create_element(file_name).write_empty() {
            Ok(_) => (),
            Err(_) => {
                println!("Unable to write element");
                return;
            }
        };
    }
}

fn import_file(import_file: &String) {
    // Check if exists
    if !Path::new(import_file).exists() {
        println!("Cannot find specified path");
        return;
    }

    // Obtain Path object to file
    let import_path = Path::new(import_file);

    // Read import file. Content este de tip String
    let content = fs::read_to_string(import_path).unwrap();

    // Get xml Reader
    let mut reader = Reader::from_str(&content);
    reader.trim_text(true);

    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => println!("Start"),
            Err(e) => println!("Error at position),
            _ => ()
        }
        buf.clear();
    }
    print_type(&reader);
}

extern crate quick_xml;

use std::{fs,str};
use std::path::Path;
use std::env;
use std::any::type_name;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use quick_xml::writer::Writer;
use std::io::{Cursor};

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

    // Creez xml writer
    let mut writer = Writer::new(Cursor::new(Vec::new()));

//  Model de scris cod xml
//    // creez un element de tip <my_elem> si ii pun in buffer inceputul
//    let mut elem = BytesStart::new("my_elem");
//    // la elem adaug o proprietate si devine <my_elem my-key="some value" >
//    elem.push_attribute(("my-key", "some value"));
//    writer.write_event(Event::Start(elem));
//
//    // Scriu continutul unui element
//    let content = BytesText::new("content");
//    writer.write_event(Event::Text(content));
//
//    // pun in buffer sfarsitul unui element
//    let end_elem = BytesEnd::new("my_elem");
//    writer.write_event(Event::End(end_elem));
//
//    // extrag textul din bufferul writer-ului
//    let result = writer.into_inner().into_inner();
//    // Vec<utf8> in str
//    let str_result = str::from_utf8(&result);
//    println!("{:?}", str_result.unwrap()); // Am text aici


    // Creates single element
    //writer.create_element("my_elem").write_empty();

    let p = Path::new(main_dir);
    print_dir_contents(p, &mut writer);


    // extrag textul din bufferul writer-ului
    let result = writer.into_inner().into_inner();
    // Vec<utf8> in str
    let str_result = str::from_utf8(&result);
    let output_path = Path::new("output.xml");
    fs::write(&output_path, str_result.unwrap());

}

// Prints the type of a variable
fn print_type<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn print_dir_contents(path: &Path, writer: &mut Writer<Cursor<Vec<u8>>>) {

    if path.is_dir() {
        writer.create_element("directory").write_empty();
        //println!("[DIRECTORY] {}", path.display());

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

            print_dir_contents( &p.as_path(), writer );
        }
    } else {
        let file_name = &path.file_name().unwrap()
                .to_str().unwrap();

        //println!("[FILE] {}", file_name);
        writer.create_element("file").write_empty();
    }
}

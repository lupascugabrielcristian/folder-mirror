extern crate quick_xml;

use std::{fs,str};
use std::path::Path;
use std::env;
use std::any::type_name;
use quick_xml::events::{Event, BytesStart, BytesEnd};
use quick_xml::writer::Writer;


fn main() {
    // TODO
    // Sa verific daca am un argument

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Pass the location of the root folder as first parameter");
        return;
    }

    let main_dir = &args[1];

    // Verific daca exista directorul
    if !Path::new(main_dir).exists() {
        println!("Cannot find specified path");
        return;
    }

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
fn _print_type<T>(_: &T) {
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

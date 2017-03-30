use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::string::string;
use serde::de::Deserialize;
use serde_protobuf::descriptor::Descriptors;
use serde_protobuf::de::Deserializer;
use serde_value::Value;
use protobuf::{CodedInputStream, parse_from_reader};

pub fn process_single(read: &mut Read) {
    for mut fdset_file, mut named_message in discover_fdsets() {
        let proto = parse_from_reader(&mut fdset_file).unwrap();
        let descriptors = Descriptors::from_proto(&proto);
        let byte_is = CodedInputStream::new(read);

        let mut deserializer = Deserializer::for_named_message(&descriptors, ".com.example.dog.Dog", byte_is).unwrap();
        let value = Value::deserialize(&mut deserializer).unwrap();
        println!("{:?}", value);
    }
}

pub fn process_stream(read: &mut Read) {
    let mut stream = CodedInputStream::new(read);

    loop {
        match stream.eof() {
            Err(e) => panic!(e),
            Ok(true) => break,
            Ok(false) => break, 
            //todo: actually do deserialization here
        }
    }
}

fn discover_fdsets() -> Vec<(File, String)> {
    let mut fdset_files = Vec::new();

    let mut home = env::home_dir().expect("Could not find $HOME");
    home.push(".pq");
    let paths = read_dir(home.as_path()).unwrap();

    for p in paths {
        let mut named_messages = Vec::new();
        let path = match p {
            Ok(p) => p.path(),
            Err(_) => continue,
        };
        match path.extension() {
            Some(x) => {
                if x != "fdset" {
                    if x == "names" {
                        let mut f = File::open(path).unwrap();
                        for l in f.lines() {
                            named_messages.push(l);
                        }
                    }
                    continue;
                }
            },
            None => continue,
        }
        fdset_files.push((File::open(path.as_path()).unwrap(),
                          named_messages.first());
    }
    return fdset_files;
}

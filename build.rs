
use std::{env, ffi::OsStr, fs, io::prelude::*, path::Path};
#[cfg(feature = "pb-rs")]
use pb_rs::types::FileDescriptor;

#[cfg(feature = "pb-rs")]
fn proto_modules(proto_dir: &Path) -> Vec<String> {
    fs::read_dir(proto_dir)
        .expect("Could not read protobuf directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("proto")) {
                path.file_stem().and_then(|n| n.to_os_string().into_string().ok())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(feature = "pb-rs")]
fn main() {
    let in_dir = "./s2client-proto/s2clientprotocol";
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Read list of all input protobuf files
    let input_mods = proto_modules(Path::new(in_dir));
    let input_files: Vec<String> = input_mods
        .iter()
        .map(|s| format!("{}/{}.proto", in_dir, s))
        .collect();
    let configs = pb_rs::ConfigBuilder::new(&input_files,
                                            None,
                                            Some(&out_dir),
                                            &["s2client-proto".to_string()])
        .unwrap()
        .single_module(true)
        .build();
    FileDescriptor::run(&configs).unwrap();
    println!("protobufs were generated successfully");
}

#[cfg(not(feature = "pb-rs"))]
fn main() {
    println!("using pre-generated *.rs files in 'src/'");
}


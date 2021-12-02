use std::env;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;

macro_rules! get_days_template {
    () => {
        r#"
use aoc_core::AoCDay;

pub fn get_days() -> Vec<Box<dyn AoCDay>> {{
    let mut days = vec![
        {}
    ];
    // Enforce sorting by day at runtime
    days.sort_by_key(|d: &Box<dyn AoCDay>| d.day());
    days
}}
"#
    };
}

macro_rules! get_inputs_template {
    () => {
        r#"
use std::collections::HashMap;

pub fn get_inputs() -> HashMap<usize, &'static str> {{
    let mut map = HashMap::new();
    {}
    map
}}
"#
    };
}

pub fn generate_get_days() {
    let in_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let days_path = Path::new(&in_dir).join("src/day/");
    let days: Vec<String> = read_dir(days_path)
        .expect("Unable to access $CARGO_MANIFEST_DIR/src/day/")
        .map(|e| {
            e.expect("Error occured while iterating $CARGO_MANIFEST_DIR/src/day")
                .path()
        })
        .filter(|e| e.is_file())
        .filter(|e| e.extension().contains(&"rs"))
        .map(|e| {
            e.file_stem()
                .unwrap()
                .to_str()
                .expect("Invalid filename")
                .to_string()
        })
        .filter(|f| f.starts_with("day_"))
        .collect();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_get_days.rs");
    let mut f = File::create(&dest_path).unwrap();

    let returns: String = days
        .iter()
        .map(|d: &String| format!("crate::day::{}::get_day(),\n", d))
        .collect();

    f.write_all(format!(get_days_template!(), returns).as_bytes())
        .unwrap();
}

pub fn generate_get_inputs() {
    //let inputs_path = Path::new(&in_dir).join("src/input/");
    let mut buf = std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap().as_str());
    buf.push("src");
    buf.push("input");

    let inputs: Vec<String> = read_dir(buf.as_path())
        .expect("Unable to access $CARGO_MANIFEST_DIR/src/input/")
        .map(|e| {
            e.expect("Error occured while iterating $CARGO_MANIFEST_DIR/src/input")
                .path()
        })
        .filter(|e| e.is_file())
        .filter(|e| e.extension().contains(&"txt"))
        .map(|e| {
            e.file_stem()
                .unwrap()
                .to_str()
                .expect("Invalid filename")
                .to_string()
        })
        .filter(|f| f.starts_with("day_"))
        .collect();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_get_inputs.rs");
    let mut f = File::create(&dest_path).unwrap();

    // with_file_name replaces the last item on the PathBuf. I need to do this
    // in order to prevent it from clobbering /input
    buf.push("_");
    let returns: String = inputs
        .iter()
        .map(|d|
            format!("map.insert({}, include_str!(\"{}\"));\n",
            d[4..].parse::<usize>().expect("Invalid usize").to_string(),
            buf.with_file_name(d)
                .with_extension("txt")
                .to_str()
                .expect("Error generating get_inputs"),
        ))
        .collect();

    f.write_all(format!(get_inputs_template!(), returns).as_bytes())
        .unwrap();
}

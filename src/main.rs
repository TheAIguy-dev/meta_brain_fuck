use std::{collections::HashMap, env, fs, io::Result};

use regex::Regex;
use walkdir::WalkDir;

/// Recursively get all BF and MBF files in the project
fn get_project_files() -> Result<Vec<String>> {
    let mut files: Vec<String> = vec![];

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file = entry.file_name().to_string_lossy().to_string();

        if file.ends_with(".mbf") || file.ends_with(".bf") {
            files.push(file);
        }
    }

    Ok(files)
}

/// Recursively expand files to their content
fn expand(mut program: String, files: &HashMap<String, String>) -> String {
    for (file_name, file_contents) in files {
        if file_name.ends_with(".bf") {
            program = program.replace(file_name, &file_contents);
        } else {
            while let Some(_) = program.find(file_name) {
                program = program.replace(file_name, &expand(file_contents.clone(), files));
            }
        }
    }

    program
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file: String = args.get(1).cloned().unwrap_or("main.mbf".to_string());
    if !input_file.ends_with(".mbf") {
        panic!("{} is not a MetaBrainFuck file", input_file);
    }
    let output_file: String = args.get(2).cloned().unwrap_or("out.bf".to_string());
    if !output_file.ends_with(".bf") {
        panic!("{} is not a BrainFuck file", output_file);
    }

    let mut program: String =
        fs::read_to_string(&input_file).expect(&format!("{} not found!", input_file));

    let mut files: HashMap<String, String> = HashMap::new();
    for file_name in get_project_files().unwrap_or_default() {
        if file_name != input_file && file_name != output_file {
            let contents: String =
                fs::read_to_string(&file_name).expect(&format!("Could not read {}", file_name));
            files.insert(file_name, contents);
        }
    }

    program = expand(program.clone(), &files);

    // Apply the repeat macro
    let re: Regex = Regex::new(r"repeat\(([\S\s]*?); ([0-9]+)\)").unwrap();
    for (m, [p, n]) in re.captures_iter(&program.clone()).map(|c| c.extract()) {
        program = program.replacen(
            m,
            &expand(p.to_string(), &files).repeat(usize::from_str_radix(n, 10).unwrap_or_default()),
            1,
        );
    }

    fs::write(&output_file, program).expect(&format!("Could not write {}", output_file));
}

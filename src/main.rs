use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

use regex::Regex;
use walkdir::WalkDir;

/// Recursively get all BF and MBF files in the project
fn get_project_files() -> HashMap<String, String> {
    let mut files: HashMap<String, String> = HashMap::new();

    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name: String = entry.file_name().to_string_lossy().to_string();
        let file_path: String = entry.path().to_string_lossy().to_string();

        if file_name.ends_with(".mbf") || file_name.ends_with(".bf") {
            let contents: String =
                fs::read_to_string(&file_path).expect(&format!("Could not read {}", file_path));
            files.insert(file_name, contents);
        }
    }

    files
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
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    let (strip_comments, charset) = match args.get(0).cloned().unwrap_or_default().as_str() {
        "-c" | "--charset" => {
            args.pop_front();
            (
                true,
                args.pop_front()
                    .expect("The option --charset requires a charset as the next argument"),
            )
        }
        _ => (false, "[]<>+-,.".to_string()),
    };

    let input_file: String = args.pop_front().unwrap_or("main.mbf".to_string());
    if !input_file.ends_with(".mbf") {
        panic!("{} is not a MetaBrainFuck file", input_file);
    }
    let output_file: String = args.pop_front().unwrap_or("out.bf".to_string());
    if !output_file.ends_with(".bf") {
        panic!("{} is not a BrainFuck file", output_file);
    }

    let files: HashMap<String, String> = get_project_files();
    let mut program: String = files.get(&input_file).unwrap().clone();

    // Apply all mappings
    let re: Regex = Regex::new(r"map\(([\S\s]*?); ([\S\s]*?)\)").unwrap();
    for (m, [c, r]) in re.captures_iter(&program.clone()).map(|c| c.extract()) {
        program = program.replace(m, "").replace(c, r);
    }

    // Apply the repeat macro
    let re: Regex = Regex::new(r"repeat\(([\S\s]*?); ([0-9]+)\)").unwrap();
    for (m, [c, n]) in re.captures_iter(&program.clone()).map(|c| c.extract()) {
        program = program.replacen(
            m,
            &c.repeat(usize::from_str_radix(n, 10).unwrap_or_default()),
            1,
        );
    }

    // Expand numbers
    let re: Regex = Regex::new(r"(?:\+|-)([0-9]+)").unwrap();
    for (m, [_]) in re.captures_iter(&program.clone()).map(|c| c.extract()) {
        let r = if m.starts_with("+") || m.starts_with("-") {
            m.chars()
                .nth(0)
                .unwrap()
                .to_string()
                .repeat(usize::from_str_radix(&m[1..m.len()], 10).unwrap_or_default())
        } else {
            "+".repeat(usize::from_str_radix(&m, 10).unwrap_or_default())
        };
        program = program.replacen(m, &r, 1);
    }

    program = expand(program.clone(), &files);

    // Strip comments
    if strip_comments {
        program = program.chars().filter(|c| charset.contains(*c)).collect();
    }

    fs::write(&output_file, program).expect(&format!("Could not write {}", output_file));
}

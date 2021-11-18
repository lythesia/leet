#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod problem;

use std::env;
use std::fs;
use std::path::Path;
use std::io::{Read,Seek,SeekFrom,Write};

/// main() helps to generate the submission template .rs
fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut nextf = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(Path::new(&cwd).join(".next"))
        .expect("open `.next' failed");
    let mut next = String::new();
    nextf.read_to_string(&mut next);

    let id = if args.len() == 1 {
        &next
    } else {
        &args[1]
    };
    let id = id.trim().parse::<u32>().expect(&format!("not a number: {}", id));

    let problem = problem::get_problem(id)
        .expect(&format!("problem #{} not found", id));
    let code = problem.code_definition.iter()
        .filter(|&d| { d.value == "rust" })
        .next()
        .expect("problem has no rust support yet");
    println!("Got: {}({})", problem.title, problem.difficulty);

    let file_name = problem.title_slug.replace("-", "_");
    let file_path = Path::new(&cwd).join("src").join("quests").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let template = fs::read_to_string(Path::new(&cwd).join("template.rs")).unwrap();
    let source = template
        .replace("__PROBLEM_TITLE__", &problem.title)
        .replace("__PROBLEM_DESC__", &problem.content)
        .replace("__PROBLEM_DEFAULT_CODE__", &code.default_code)
        .replace("__PROBLEM_ID__", &format!("{}", id))
        .replace("__EXTRA_USE__", &parse_extra_use(&code.default_code));

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);

    let mut lib_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(Path::new(&cwd).join("src").join("quests.rs"))
        .unwrap();
    writeln!(lib_file, "mod {};", file_name);

    nextf.seek(SeekFrom::Start(0)).expect(".next seek");
    writeln!(nextf, "{}", id + 1);
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse super::super::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse super::super::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse super::super::util::point::Point;")
    }
    extra_use_line
}

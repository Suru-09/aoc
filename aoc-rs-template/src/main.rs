pub mod args;

use args::args::Args;
use clap::Parser;
use std::{env, fs};
use std::path::Path;
use std::process::Command;

fn write_template(args: &Args) {
  let mut rsrc_dir = env::current_dir()
      .expect("Can't find the executable!");

  rsrc_dir.push("template");

  let template_path = rsrc_dir.join("template.rs");
  println!("template dir : {:?}", template_path);

  let template_string = 
      fs::read_to_string(template_path)
      .expect("Could not read template.rs!");

  let aoc_year = &args.year;
  let aoc_day = &args.day;

  let relative_path = Path::new("../");
  if let Err(e) = env::set_current_dir(&relative_path) {
      eprintln!("Error moving up one level: {}", e);
  }

  let mut curr_dir = match env::current_dir() {
      Ok(path) => {
        println!("Now in directory: {}", path.display());
        path
      },
      Err(_) => panic!("Error retrieving current directory")
  };

  curr_dir.push(aoc_year);

  if !curr_dir.exists() {
    if let Err(e) = fs::create_dir_all(&curr_dir) {
      eprintln!("Failed to create directory: {}", e);
    } else {
        println!("Directory created successfully: {}", curr_dir.display());
    }
  }

  if let Err(e) = env::set_current_dir(&curr_dir) {
    eprintln!("Could not set current_dir to: {:?}, err: {}", curr_dir.to_str(),  e);
  }

  let formated_day = &format!("Day-{}", aoc_day);

  let output = Command::new("cargo")
    .arg("new")
    .arg(formated_day)
    .current_dir(&curr_dir) 
    .output()
    .expect("Failed to execute `cargo new`");

  if output.status.success() {
    println!(
        "Successfully created new project: {}/{}",
        curr_dir.display(),
        formated_day
    );

    let main_rs_path = curr_dir
      .join(formated_day)
      .join("src")
      .join("main.rs");

    println!("Main rs path: {:?}", main_rs_path);

    match fs::write(&main_rs_path, template_string) {
        Ok(_) => println!("Successfully updated: {}", main_rs_path.display()),
        Err(e) => eprintln!("Failed to update `src/main.rs`: {}", e),
    }
  } else {
    eprintln!(
        "Failed to create project. Error: {}",
        String::from_utf8_lossy(&output.stderr)
    );
  }
}

pub fn download_input_file(_args: &Args) -> String {
  String::new()
}


fn main() {
  let args = Args::parse();
  write_template(&args);
}

pub mod args;

use args::args::Args;
use clap::Parser;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, CONNECTION, COOKIE, HOST};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

fn write_template(args: &Args) {
    let mut rsrc_dir = env::current_dir().expect("Can't find the executable!");

    rsrc_dir.push("template");

    let template_path = rsrc_dir.join("template.rs");
    println!("template dir : {:?}", template_path);

    let template_string = fs::read_to_string(template_path).expect("Could not read template.rs!");

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
        }
        Err(_) => panic!("Error retrieving current directory"),
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
        eprintln!(
            "Could not set current_dir to: {:?}, err: {}",
            curr_dir.to_str(),
            e
        );
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

        let main_rs_path = curr_dir.join(formated_day).join("src").join("main.rs");

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

pub async fn download_input(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cookie = env::var("COOKIE").expect("COOKIE env variable was not set!");
    let client = reqwest::Client::new();
    let response: reqwest::Response = client
        .get(path)
        .header(COOKIE, cookie)
        .header(ACCEPT, "*/*")
        .header(ACCEPT_ENCODING, "gzip, deflate, br")
        .header(HOST, "adventofcode.com")
        .header(CONNECTION, "keep-alive")
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());

    if !response.status().is_success() {
        eprintln!("Failed to download the file: HTTP {}", response.status());
        return Err(Box::from("Failed to download file"));
    }

    let text = response.text().await?;
    println!("Text: {}", text);

    Ok(text)
}

pub async fn create_input_files(args: &Args) {
    let aoc_day = &args.day;
    let aoc_year = &args.year;
    let download_path = format!(
        "https://www.adventofcode.com/{}/day/{}/input",
        aoc_year, aoc_day
    );
    println!("Download path: {}", download_path);
    match download_input(&download_path).await {
        Ok(input) => {
            std::fs::write(format!("./Day-{}/example.txt", aoc_day), "")
                .expect("Should be able to write example file");
            std::fs::write(format!("./Day-{}/input.txt", aoc_day), input)
                .expect("Should be able to write input file");
        }
        Err(e) => panic!("Could not download input file err: {:?}", e),
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    write_template(&args);
    create_input_files(&args).await;

    Ok(())
}

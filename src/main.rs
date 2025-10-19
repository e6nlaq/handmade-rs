#[macro_use]
extern crate clap;

use clap::Parser;
use dialoguer::Input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The folder name of test cases
    #[arg(short, long, default_value = "test")]
    test: String,

    // The prefix to add to test cases
    #[arg(short, long, default_value = "z_handmade")]
    case_prefix: String,
}

use std::error::Error;
use std::fs;
use std::path;

fn read_dir(path: &str) -> Result<Vec<path::PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<path::PathBuf> = Vec::new();
    for item in dir.into_iter() {
        files.push(item?.path());
    }
    Ok(files)
}

fn main() {
    let args = Args::parse();

    println!("handmade v{} / by {}\n", crate_version!(), crate_authors!());

    print!("Searching for test cases in {}... ", args.test);

    let files_buf = read_dir(&args.test).unwrap();
    let mut files = files_buf
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap())
        .collect::<Vec<_>>();
    files.sort();

    println!("{} case(s) found!", files.len());

    let mut x = 1;
    while files
        .binary_search(&format!("{}-{}.in", args.case_prefix, x).as_str())
        .is_ok()
    {
        x += 1;
    }

    println!("{} handmade case(s) found!", x - 1);

    let input_name = format!("{}-{}.in", args.case_prefix, x);
    let output_name = format!("{}-{}.out", args.case_prefix, x);
    println!("{} and {} will be created.\n", input_name, output_name);

    let mut inp=Vec::<String>::new();
    let mut out=Vec::<String>::new();

    println!("{}",input_name);
    loop{
        let text:String=Input::new().allow_empty(true).interact_text().unwrap();
        inp.push(text.clone());
        if text.is_empty(){
            println!();
            break;
        }

    }

    println!("{}",output_name);
    loop{
        let text:String=Input::new().allow_empty(true).interact_text().unwrap();
        out.push(text.clone());
        if text.is_empty(){
            println!();
            break;
        }

    }

    print!("Saving... ");
    fs::write(format!("{}/{}", args.test, input_name), inp.join("\n")).unwrap();
    fs::write(format!("{}/{}", args.test, output_name), out.join("\n")).unwrap();
    println!("Done!");
}

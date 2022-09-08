use colored::Colorize;
use regex::Regex;
use std::env::args;
use std::io::Write;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
use tempfile::NamedTempFile;
use cli_clipboard::{ClipboardProvider, ClipboardContext};

fn main() {
    let template_regex = Regex::new(r"\{\{\d+\}\}").unwrap();
    let mut cmd_template: Vec<String> = args().skip(1).collect();
    let argc = cmd_template.len();
    let mut template_mapping: Vec<Vec<usize>> = vec![Vec::new(); argc];
    for (index, arg) in cmd_template.iter().enumerate().skip(1) {
        if template_regex.is_match(arg) {
            if let Ok(curry_index) = arg[2..(arg.len() - 2)].parse::<usize>() {
                if curry_index >= argc {
                    panic!("{arg} is not a valid placeholder index");
                }
                template_mapping[curry_index].push(index);
            } else {
                panic!("{arg} is not a valid placeholder index");
            }
        }
    }

    let zero_indexed = template_mapping.get(0).unwrap().len() > 0;
    let mut end = None;
    let mut noop = true;
    for (index, mapping) in
        template_mapping
            .iter()
            .enumerate()
            .skip(if zero_indexed { 0 } else { 1 })
    {
        if mapping.len() == 0 {
            end = Some(index);
        } else {
            noop = false;
            if let Some(endex) = end {
                panic!("Template indicies must increase monotonically, a template with value {} was found but no template of value {} exists", index, endex);
            }
        }
    }

    if noop {
        println!("No replacement templates found in command, doing nothing");
        exit(0);
    }

    println!("Copy the text you want to use for the highlighted arguments into your clipboard. Press enter to continue");
    let mut tempfiles = Vec::new();
    let mut ctx = ClipboardContext::new().unwrap();
    for mapping in template_mapping
        .iter()
        .skip(if zero_indexed { 0 } else { 1 })
    {
        if mapping.len() == 0 {
            break;
        }
        print!("{}", '\r');
        for (index, arg) in cmd_template.iter().enumerate() {
            print!(
                " {}",
                if mapping.contains(&index) {
                    arg.red().underline()
                } else {
                    arg.white()
                }
            );
            
        }
        let _ = std::io::stdout().flush();
        let mut dummy = String::new();
        let _ = std::io::stdin().read_line(&mut dummy);
        let mut file = NamedTempFile::new().unwrap();
        let _ = file.write_all(ctx.get_contents().unwrap().as_bytes());
        tempfiles.push(file);
        print!("\u{001b}[1A");
    }
    println!("");
    for (mapping, file) in template_mapping
        .iter()
        .skip(if zero_indexed { 0 } else { 1 }).zip(tempfiles.iter())
    {
        if mapping.len() == 0 {
            break;
        }
        for arg_index in mapping {
            cmd_template[*arg_index] = file.path().to_string_lossy().to_string();
        }
    }
    Command::new(cmd_template.get(0).unwrap().clone()).args(&mut cmd_template[1..]).exec();
}

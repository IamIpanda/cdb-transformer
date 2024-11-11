mod card;
mod constants;
mod transformers;

use std::fs::write;

use card::CardTransformer;
use clap::Parser;
use constants::{Format, OT};
use transformers::*;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Format of source files.
    /// 
    /// .txt file is always xyyz.
    /// .sql file is always sql.
    /// .lua file is always script.
    /// .cdb file is always cdb.
    /// If none of file appendix matches, program will use value of this field.
    #[arg(long)]
    from_format: Option<Format>,
    /// Source files.
    /// 
    /// If no value given, source is stdin, format is xyyz.
    #[arg(short, long)]
    from: Vec<String>,
    /// Format of target files.
    /// 
    /// Program will guess from filename if not provided. Default format is xyyz.
    #[arg(long, default_value_t=Format::Xyyz)]
    to_format: Format,
    /// Target file. 
    /// 
    /// You can use {id} to set target lua name when target format is script.
    /// If no value given, target is stdout, format is xyyz.
    #[arg(short, long, default_value="/dev/stdout")]
    to: String,
    /// strings.conf path. 
    /// 
    /// These files is used to format set name inner xyyz format.
    #[arg(long)]
    strings: Vec<String>,
    /// If set, program will allow draft cards goto result
    /// 
    /// Draft card is an extra OT value only available in xyyz format.
    #[arg(long)]
    allow_draft: bool,
    /// Max line characters for script mode. 
    #[arg(long, default_value_t=100)]
    max_line_length: usize
}

fn guess_format(path: &String, default: Option<Format>) -> Format {
    if path.ends_with(".cdb") { Format::CDB }
    else if path.ends_with(".sql") { Format::SQL }
    else if path.ends_with(".txt") { Format::Xyyz }
    else if path.ends_with(".xyyz") { Format::Xyyz }
    else if path.ends_with(".lua") {Format::Script }
    else if let Some(format) = default { format }
    else { println!("Cannot determain the format by filename {}. Make it as xyyz...", path); Format::Xyyz }
}

fn preprocess() -> Vec<String> {
    let mut prcoessd_args = Vec::new();
    let mut controlling = String::new();
    let mut argument_count = 0;
    for arg in wild::args() {
        if arg.starts_with("-") {
            controlling = arg.clone();
            argument_count = 0;
        } else {
            if prcoessd_args.len() == 0 {
                controlling = "--from".to_string()
            }
            argument_count += 1;
            if argument_count > 1 {
                prcoessd_args.push(controlling.clone())
            }
        }
        prcoessd_args.push(arg)
    }
    prcoessd_args
}

fn main() {
    let mut args = Args::parse_from(preprocess());
    read_string_conf(&args.strings);
    MAX_LINE_LENGTH.set(args.max_line_length).expect("MAX_LINE_LENGTH already inited.");
    let mut cards = Vec::new();
    if args.from.len() == 0 { args.from = vec!["/dev/stdin".to_string()] }
    for source in args.from {
        print!("Reading {}... ", source);
        let card_parts = match guess_format(&source, args.from_format) {
            Format::Xyyz => Xyyz::from_string(&std::fs::read_to_string(&source).expect(&format!("Read file {} failed", source))),
            Format::SQL  =>  SQL::from_string(&std::fs::read_to_string(&source).expect(&format!("Read file {} failed", source))),
            #[cfg(not(target_arch = "wasm32"))]
            Format::CDB  =>  CDB::from_string(&source),
            Format::Script => Script::from_string(&std::fs::read_to_string(&source).expect(&format!("Read file {} failed", source))),
            _ => unimplemented!("Unimplemented type of source.")
        };
        cards.extend(card_parts)
    };
    if !(args.allow_draft) {
        cards = cards.into_iter().filter(|c| !c.ot.contains(OT::Draft)).collect();
    }
    match guess_format(&args.to, Some(args.to_format)) {
        Format::Xyyz => write(args.to, cards.iter().map(|c| Xyyz::to_string(c)).collect::<Vec<_>>().join("\n\n")),
        Format::SQL => write(args.to, cards.iter().map(|c| SQL::to_string(c)).collect::<Vec<_>>().join("\n\n")),
        #[cfg(not(target_arch = "wasm32"))]
        Format::CDB => Ok(CDB::save_to(&cards, &args.to)),
        Format::Script => Ok(Script::save_to(&cards, &args.to)),
        _ => unimplemented!("Unimplemented type of target.")
    }.expect("Write file failed");
}

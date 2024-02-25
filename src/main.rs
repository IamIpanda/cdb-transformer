mod card;
mod constants;
mod transformers;

use std::fs::write;

use card::CardTransformer;
use clap::{Parser, ValueEnum};
use constants::OT;
use transformers::*;

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Xyyz,
    CDB,
    SQL
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// will guess from filename if not provided.
    #[arg(long)]
    from_format: Option<Format>,
    #[arg(short, long)]
    from: String,
    /// will guess from filename if not provided.
    #[arg(long)]
    to_format: Option<Format>,
    #[arg(short, long)]
    to: String,
    /// strings.conf path. This file is used to format set name inner xyyz format.
    #[arg(long)]
    strings: Vec<String>,
    /// allow draft cards goto result.
    #[arg(long)]
    allow_draft: bool
}

fn guess_format(path: &String) -> Format {
    if path.ends_with(".cdb") { Format::CDB }
    else if path.ends_with(".sql") { Format::SQL }
    else if path.ends_with(".txt") { Format::Xyyz }
    else if path.ends_with(".xyyz") { Format::Xyyz }
    else { println!("Cannot determain the format by filename. Make it as text..."); Format::Xyyz }
}

fn main() {
    let args = Args::parse();
    read_string_conf(&args.strings);
    
    let mut cards = match args.from_format.map_or_else(|| guess_format(&args.from), |f| f) {
        Format::Xyyz => Xyyz::from_string(&std::fs::read_to_string(args.from).expect("Read file failed")),
        Format::SQL => SQL::from_string(&std::fs::read_to_string(args.from).expect("Read file failed")),
        Format::CDB => CDB::from_string(&args.from),
    };
    if !(args.allow_draft) {
        cards = cards.into_iter().filter(|c| !c.ot.contains(OT::Draft)).collect();
    }
    match args.to_format.map_or_else(|| guess_format(&args.to), |f| f) {
        Format::Xyyz => write(args.to, cards.iter().map(|c| Xyyz::to_string(c)).collect::<Vec<_>>().join("\n\n")),
        Format::SQL => write(args.to, cards.iter().map(|c| SQL::to_string(c)).collect::<Vec<_>>().join("\n\n")),
        Format::CDB => Ok(CDB::save_to(&cards, &args.to)),
    }.expect("Write file failed");
}

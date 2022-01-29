use std::{time::Instant, fs};

mod mdma;
mod bindings;
mod match_finder;
mod entropy_ranking;
mod counting;
mod splitting;
mod parser;

fn main() -> std::io::Result<()> {
    let file_name = "/data/calgary/book1";

    println!("Building dict for: {}", file_name);
    let timer = Instant::now();
    let buf = fs::read(file_name)?;
    let mut index = mdma::initialize(buf);
    let dict = mdma::build_dictionary(&mut index);
    println!("Dict took: {:?}", timer.elapsed());

    // TODO: Move encode dict and decode dict to a new file
    parser::encode_dict(&dict, &index, "dict.bin");
    parser::parse(&dict, &mut index, "parsed.bin");

    dbg!(dict.len());
    if dict.len() > 0 {
        dbg!(dict[0].location);
        dbg!(dict[0].len);
    }

    return Ok(());
}

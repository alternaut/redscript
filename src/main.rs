mod decode;
mod error;
mod script;

use script::bundle::ScriptBundle;
use script::definition::AnyDefinition;
use script::print::write_definition;

use gumdrop::Options;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug, Options)]
struct Configuration {
    #[options(required, short = "i")]
    input: PathBuf,
    #[options(required, short = "o")]
    output: PathBuf,
}

fn main() {
    let config: Configuration = Configuration::parse_args_default_or_exit();

    let mut input = BufReader::new(File::open(config.input).expect("Failed to open file"));
    let cache: ScriptBundle = ScriptBundle::load(&mut input).expect("Failed to decode script bundle");

    let mut output = BufWriter::new(File::create(config.output).expect("Failed to create output file"));
    let pool = cache.pool();

    for def in pool
        .definitions()
        .iter()
        .filter(|def| matches!(&def.value, AnyDefinition::Class(_)))
    {
        match write_definition(&mut output, def, pool) {
            Ok(()) => {}
            Err(err) => println!("Failed to process definition {:?}: {:?}", def, err),
        }
    }
}
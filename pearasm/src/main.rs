extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::{Parser, iterators::Pair};
use std::{
    fs::{self, File},
    io::Write, env::Args,
};

#[derive(Parser)]
#[grammar = "pearasm.pest"]
pub struct PearAsmParser;

type Argument = u64;

fn generate_argument(arg: Pair<Rule>) -> Argument {
    match arg.as_rule() {
        Rule::integer => {
            arg.as_str().parse().unwrap()  
        },
        _ => unreachable!()
    }
}

fn translate_assembly(assembly: String) -> Vec<u8> {
    let file = PearAsmParser::parse(Rule::file, &assembly)
        .expect("Parsing error.");
        
    let bytecode = Vec::new();

    for line in file {
        match line.as_rule() {
            Rule::EOI => (),
            Rule::instr => {
                let mut rules = line.into_inner(); // { instr_name ~ instr_arg? } 
                
                let name: &str = rules.next().unwrap().as_str();
                let arg = match rules.next() {
                    Some(arg) => Some(generate_argument(arg.into_inner().next().unwrap())),
                    None => None,
                }; 

                println!("{} {:?}", name, arg);  
            },
            _ => unreachable!()
        } 
    }

    bytecode
}

fn main() {
    // pearasm <input .prasm> -o <output .prbc>
    // pearasm <input .prasm>

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => {
            if !&args[1].ends_with(".prasm") {
                eprintln!(
                    "Error: Input file does not have an extension .prasm: {}",
                    &args[1]
                )
            }
            let input_data =
                fs::read_to_string(&args[1]).expect("Error: Can not read the input file.");
            let assembly = translate_assembly(input_data);
            let mut output_file = File::create(&args[1].replacen(".prasm", ".prbc", 1))
                .expect("Error: Can not create output file.");
            output_file
                .write_all(&assembly)
                .expect("Can not write into the output file");
        },

        4 => {
            if !&args[1].ends_with(".prasm") {
                eprintln!(
                    "Error: Input file does not have an extension .prasm: {}",
                    &args[1]
                )
            }
            if !&args[3].ends_with(".prbc") {
                eprintln!(
                    "Error: Input file does not have an extension .prbc: {}",
                    &args[3]
                )
            }
            let input_data =
                fs::read_to_string(&args[1]).expect("Error: Can not read the input file.");
            let assembly = translate_assembly(input_data);
            let mut output_file = File::create(&args[3])
                .expect("Error: Can not create output file.");
            output_file
                .write_all(&assembly)
                .expect("Can not write into the output file");

        },

        _ => eprintln!(
            "pearasm help: 
    {} <input path (.prasm)> [-o <output path (.prbc)>]
    
    <input path> - Pear assembly file (to be converted into bytecode)
    <output path> - Generated Pear bytecode file",
            args[0]
        ),
    }
}

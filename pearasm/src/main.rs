extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};
use std::{
    fs::{self, File},
    io::Write, intrinsics::transmute,
};

#[derive(Parser)]
#[grammar = "pearasm.pest"]
pub struct PearAsmParser;

type Argument = u16;

fn generate_argument(arg: Pair<Rule>) -> Argument {
    match arg.as_rule() {
        Rule::integer => arg.as_str().parse().unwrap(),
        _ => unreachable!(),
    }
}

fn instruction_code(name: &str) -> Option<u8> {
    match name {
        "nop" => Some(0),
        "push" => Some(1),
        "pop" => Some(2),
        "add" => Some(3),
        "print" => Some(4),
        "halt" => Some(5),
        _ => None,
    }
}

fn requires_argument(name: &str) -> bool {
    ["push"].contains(&name)
}

fn translate_assembly(assembly: String) -> Vec<u8> {
    let file = PearAsmParser::parse(Rule::file, &assembly).expect("Parsing error.");

    let mut bytecode = Vec::new();

    for line in file {
        match line.as_rule() {
            Rule::EOI => (),
            Rule::instr => {
                let mut rules = line.into_inner(); // { instr_name ~ instr_arg? }
                let name  = rules.next().unwrap(); // Get a name rule

                let name_span = name.as_span(); // Get a span of the name of the instruction
                let name = name.as_str(); // Get a name of the instruction

                let arg = match rules.next() {
                    Some(arg) => Some(generate_argument(arg.into_inner().next().unwrap())),
                    None => None,
                };
               
                let instr_code = match instruction_code(name) {
                    Some(code) => code,
                    None => panic!("Error: Unknown instruction: {}", name),
                };

                if requires_argument(name) && arg.is_none() {
                    panic!("No argument provided in the instruction: {} {:?}", name, name_span);
                }

                bytecode.push(instr_code);    
                match arg {
                    Some(arg) => { 
                        let bytes: [u8; 2] = unsafe { transmute(arg.to_le()) };
                        bytecode.extend(bytes.iter().cloned());
                    }, 
                    None => {},
                }
            }
            _ => unreachable!(),
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
                panic!(
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
        }

        4 => {
            if !&args[1].ends_with(".prasm") {
                panic!(
                    "Error: Input file does not have an extension .prasm: {}",
                    &args[1]
                )
            }
            if !&args[3].ends_with(".prbc") {
                panic!(
                    "Error: Input file does not have an extension .prbc: {}",
                    &args[3]
                )
            }
            let input_data =
                fs::read_to_string(&args[1]).expect("Error: Can not read the input file.");
            let assembly = translate_assembly(input_data);
            let mut output_file =
                File::create(&args[3]).expect("Error: Can not create output file.");
            output_file
                .write_all(&assembly)
                .expect("Can not write into the output file");
        }

        _ => eprintln!(
            "pearasm help: 
    {} <input path (.prasm)> [-o <output path (.prbc)>]
    
    <input path> - Pear assembly file (to be converted into bytecode)
    <output path> - Generated Pear bytecode file",
            args[0]
        ),
    }
}

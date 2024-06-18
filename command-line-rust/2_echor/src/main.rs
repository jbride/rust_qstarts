use clap::{Arg, ArgAction ,Command, value_parser};

const TEXT: &str = "text";
const OMIT_NEWLINE: &str = "omit_newline";

fn main() {

    // Specify curly brackets that will serve as placeholder for the printed value
    // Printed value will be an Arg type which implemented Debug trait ( indicated by: :? )
    // First argument is the path of the program itself
    println!("{:?}", std::env::args());

    // The leading underscore in the variable name _matches is functional.
    // It tells the Rust compiler that the variable is not used.
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("jbride <jbride@jbride.net")
        .about("Rust version of echo")
        .arg(
            Arg::new(TEXT)
                .value_name(TEXT)
                .help("Input text")
                .required(true)
                .num_args(1..)
                .action(ArgAction::Append)
                .value_parser(value_parser!(String)),

        )
        .arg(
            Arg::new(OMIT_NEWLINE)
            .short('n')
            .action(ArgAction::SetTrue) // Set the arg to true when the flag is present or false when absent
            .help("Do not print newline"),

        )
        .get_matches();

    // Need to use cloned() (rather than collect()) since these args are stored on heap
    let _text_args: Vec<String>  = _matches.get_many(TEXT).unwrap().cloned().collect();
    println!("text count = {}, text = \t{:?}", _text_args.len(), _text_args);

    // data type of omit_newline is a boolean, so can be copied on stack
    let _omit_newline_arg: Vec<bool> = _matches.get_many(OMIT_NEWLINE).unwrap().copied().collect();
    println!("omit newline count = {}, omit_newline = \t{:?}", _omit_newline_arg.len(), _omit_newline_arg);
}

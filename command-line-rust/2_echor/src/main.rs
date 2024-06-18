use clap::{parser::ValuesRef, value_parser, Arg, ArgAction, Command};

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

    // Clap will require at least one arg of this Id so it is safe to use unwrap()
    // Need to use cloned() (rather than collect()) since these args are stored on heap
    let _text_args: Vec<String>  = _matches.get_many(TEXT).unwrap().cloned().collect();
    println!("text count = {}, text = \t{}", _text_args.len(), _text_args.join(" "));

    // Clap will set a default value arg of this Id (if not passed by user), so it is safe to use unwrap()
    // data type of omit_newline is a boolean, so can be copied on stack
    let _omit_newline_arg_vec: Vec<bool>  = _matches.get_many(OMIT_NEWLINE).unwrap().copied().collect();
    println!("omit_newline_vec = \t{:?}", _omit_newline_arg_vec);

    let _omit_newline_arg_flag: bool = _matches.get_flag(OMIT_NEWLINE);
    println!("omit_newline = \t{}", _omit_newline_arg_flag);




}

use clap::{command, Arg, Command};
use wfc::wfc::cli::{compile::compile, generate::generate};

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("compile")
                .about("Compiles a text file into rules json")
                .arg(
                    Arg::new("input_file")
                        .short('i')
                        .help("the text file to compile to rules"),
                )
                .arg(Arg::new("output_file").short('o').help("the output file")),
        )
        .subcommand(
            Command::new("generate")
                .about("Compiles a text file into rules json")
                .arg(
                    Arg::new("rules")
                        .short('r')
                        .help("the rules for the algorithm"),
                )
                .arg(
                    Arg::new("tokens")
                        .short('t')
                        .help("how many tokens to generate")
                        .value_parser(clap::value_parser!(usize)),
                ),
        )
        .get_matches();

    let result = match matches.subcommand() {
        Some(("compile", compile_args)) => compile(
            compile_args.get_one::<String>("input_file"),
            compile_args.get_one::<String>("output_file"),
        ),
        Some(("generate", generate_args)) => generate(
            generate_args.get_one::<String>("rules"),
            generate_args.get_one::<usize>("tokens"),
        ),
        _ => unreachable!(),
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

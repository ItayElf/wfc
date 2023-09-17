use clap::{command, Arg, Command};
use wfc::wfc::cli::compile::compile;

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
        .get_matches();

    let result = match matches.subcommand() {
        Some(("compile", compile_args)) => compile(
            compile_args.get_one::<String>("input_file"),
            compile_args.get_one::<String>("output_file"),
        ),
        _ => unreachable!(),
    };

    match result {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}

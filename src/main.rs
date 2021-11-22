use clap::{App, load_yaml};

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let args = App::from(cli_cnf).get_matches();

    match args.subcommand() {
        Some(("addresses", &sub_args)) => oa_validate::addresses::main(sub_args),
        _ => {
            println!("Invalid Subcommand: ./rai-toolkit --help for valid options");
            std::process::exit(1);
        },
    }
}

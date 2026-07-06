use std::process::ExitCode;

use elderheim::target::TargetSpec;

fn main() -> ExitCode {
    run(std::env::args().skip(1))
}

fn run<I>(mut args: I) -> ExitCode
where
    I: Iterator<Item = String>,
{
    match args.next().as_deref() {
        None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("--help" | "-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some("--list-targets") => {
            if args.next().is_some() {
                eprintln!("E-CLI-TRAILING-ARG: --list-targets does not accept extra arguments");
                return ExitCode::FAILURE;
            }

            list_targets();
            ExitCode::SUCCESS
        }
        Some("--target") => validate_target_arg(args.next(), args.next()),
        Some(_) => {
            eprintln!("E-CLI-ARG: unsupported command line argument");
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!("elderheim 0.2.0: compiler target matrix scaffold");
    println!("usage:");
    println!("  elderheim --list-targets");
    println!("  elderheim --target <os-architecture-format>");
}

fn list_targets() {
    for target in TargetSpec::SUPPORTED_1_0 {
        if let Some(name) = target.cli_name() {
            println!("{name}");
        }
    }
}

fn validate_target_arg(target: Option<String>, trailing: Option<String>) -> ExitCode {
    let Some(target) = target else {
        eprintln!("E-CLI-MISSING-TARGET: --target requires a target name");
        return ExitCode::FAILURE;
    };

    if trailing.is_some() {
        eprintln!("E-CLI-TRAILING-ARG: --target accepts exactly one target name");
        return ExitCode::FAILURE;
    }

    match TargetSpec::parse_cli_name(target.as_str()) {
        Ok(validated) => {
            if let Some(name) = validated.cli_name() {
                println!("{name}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}: {}", error.code(), error.message());
            ExitCode::FAILURE
        }
    }
}

mod args;
use args as tcargs;
use args::Args;

fn main() -> std::process::ExitCode {
    if std::env::args().len() == 1 {
        println!("tcobalt Command Line Utility; run `tc help` for help");
        return std::process::ExitCode::SUCCESS;
    }

    let placeholder = "https://www.youtube.com/watch?v=zn5sTDXSp8E";
    let args = match Args::get().parse() {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Invalid command syntax");
            eprintln!("{}", err.print());
            return std::process::ExitCode::FAILURE;
        },
    };
    if let Some(help_flag) = args.help_flag {
        match help_flag {
            args::types::Help::Help => println!("{}", tcargs::strings::get_help()),
            args::types::Help::List => println!("{}", tcargs::strings::get_mod_help("list")),
            args::types::Help::Bulk => println!("{}", tcargs::strings::get_mod_help("bulk")),
            args::types::Help::Get => println!("{}", tcargs::strings::get_mod_help("get")),
        }
        return std::process::ExitCode::SUCCESS;
    }
    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests;

mod args;
use args as tcargs;
use args::Args;

fn main() -> std::process::ExitCode {
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
            args::types::Help::Get => ,
            args::types::Help::List => todo!(),
            args::types::Help::Bulk => todo!(),
            args::types::Help::Help => todo!(),
        }
        return std::process::ExitCode::SUCCESS;
    }
    return std::process::ExitCode::SUCCESS;
}

#[cfg(test)]
mod tests;

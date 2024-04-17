mod json;
mod args;
use args as tcargs;
use args::Args;

#[tokio::main]
async fn main() -> std::process::ExitCode {
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
            args::types::Help::List => println!("{}", tcargs::strings::get_mod("list")),
            args::types::Help::Bulk => println!("{}", tcargs::strings::get_mod("bulk")),
            args::types::Help::Get => println!("{}", tcargs::strings::get_mod("get")),
            args::types::Help::Examples => println!("{}", tcargs::strings::get_mod("examples")),
        }
        return std::process::ExitCode::SUCCESS;
    }
    match args.method.clone().expect("Failed to catch invalid method early") {
        args::types::Method::Get => {
            println!("{:#?}", args);
        },
        args::types::Method::List => println!("{}", tcargs::strings::get_mod("supported")),
        args::types::Method::Bulk => todo!(),
        args::types::Method::Help => unreachable!(),
        args::types::Method::Version => println!("{}", tcargs::strings::get_mod("version")),
        args::types::Method::CobaltVersion => {
            let ver = reqwest::get("https://co.wuk.sh/api/serverInfo").await.unwrap().text().await.unwrap();
            let stats = json::parse(ver).expect("cobalt server returned invalid json");
            let version = stats.get("version").unwrap().get_str().unwrap();
            let commit = stats.get("commit").unwrap().get_str().unwrap();
            let branch = stats.get("branch").unwrap().get_str().unwrap();
            println!("Cobalt (by wukko) version {version}");
            println!("Latest commit on branch \"{branch}\": {commit}");
        },
    }
    std::process::ExitCode::SUCCESS
}

#[cfg(test)]
mod tests;

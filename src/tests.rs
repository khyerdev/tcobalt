#[test]
fn helptext() {
    let args = crate::args::Args::_override_args(&["help"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Help));
    let args = crate::args::Args::_override_args(&["help", "get"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Get));
    let args = crate::args::Args::_override_args(&["help", "list"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::List));
    let args = crate::args::Args::_override_args(&["help", "bulk"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Bulk));
}

#[test]
fn helptext() {
    let args = crate::args::Args::_override_args(&["help"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Help));
}

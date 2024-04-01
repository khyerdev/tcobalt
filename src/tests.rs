#[test]
fn helptext() {
    let args = crate::args::Args::_override_args(&["help"]).parse().unwrap();
    let args = crate::args::Args::_override_args(&["help", "get"]).parse().unwrap();
    let args = crate::args::Args::_override_args(&["help", "list"]).parse().unwrap();
    let args = crate::args::Args::_override_args(&["help", "bulk"]).parse().unwrap();
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Help));
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Get));
    assert_eq!(args.help_flag, Some(crate::args::types::Help::List));
    assert_eq!(args.help_flag, Some(crate::args::types::Help::Bulk));
}

#[test]
fn json_parse() {
    let inline_str = "{\"key\":\"val\",\"num\":42,\"bool\":true,\"none\":null,\"list\":[\"member1\",\"member2\"]}";
    let block_str = "{
        \"key\": \"val\",
        \"num\": 42,
        \"bool\": true,
        \"none\": null,
        \"list\": [
            \"member1\",
            \"member2\"
        ],
        \"inline list\": [\"member1\", \"menber2\"]
    }";
}

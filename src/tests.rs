#[test]
fn helptext() {
    let help_args = crate::args::Args::_override_args(&["help"]).parse().unwrap();
    let get_args = crate::args::Args::_override_args(&["help", "get"]).parse().unwrap();
    let list_args = crate::args::Args::_override_args(&["help", "list"]).parse().unwrap();
    let bulk_args = crate::args::Args::_override_args(&["help", "bulk"]).parse().unwrap();
    assert_eq!(help_args.help_flag, Some(crate::args::types::Help::Help));
    assert_eq!(get_args.help_flag, Some(crate::args::types::Help::Get));
    assert_eq!(list_args.help_flag, Some(crate::args::types::Help::List));
    assert_eq!(bulk_args.help_flag, Some(crate::args::types::Help::Bulk));
}

#[test]
fn json_parse() {
    let inline_str = "{\"key\":\"val\",\"num\":42,\"bool\":true,\"none\":null,\"list\":[\"member1\",\"member2\"],\"subset\":{\"subkey\":\"val\"}}";
    let block_str = "{
        \"key\": \"val\",
        \"num\": 42,
        \"bool\": true,
        \"none\": null,
        \"list\": [
            \"member1\",
            \"member2\"
        ],
        \"inline list\": [\"member1\", \"menber2\", 99, false, {\"member key\": \"val\"}]
        \"float\": 6752.88
        \"subset\": {
            \"subkey\": \"val\",
            \"sublist\": [\"only member\"]
        }
    }";
}

#[test]
#[should_panic]
fn space_is_control() {
    assert!(' '.is_control());
}

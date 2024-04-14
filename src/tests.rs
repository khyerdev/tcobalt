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
    use std::collections::HashMap;
    use crate::json::{self, JsonValue as Val};

    let inline_str = "{\"key\":\"val\",\"num\":42,\"bool\":true,\"none\":null,\"list\":[\"member1\",\"member2\"],\"subobj\":{\"subkey\":\"val\"}}";
    let block_str = "{
        \"key\": \"val\",
        \"num\": 42,
        \"bool\": true,
        \"none\": null,
        \"list\": [
            \"member1\",
            \"member2\"
        ],
        \"inline list\": [\"member1\", \"member2\", 99, false, {\"member key\": \"val\", \"ruhhhh\": [\"nuh uh\"]}, null],
        \"float\": 6752.88,
        \"subobj\": {
            \"subkey\": \"val\",
            \"sublist\": [\"only member\"]
        }
    }";

    let inline_parsed = json::parse(inline_str).unwrap();
    let block_parsed = json::parse(block_str).unwrap();

    let proper_inline: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Str("val".into())),
        ("num".into(), Val::Int(42)),
        ("bool".into(), Val::Bool(true)),
        ("none".into(), Val::Null),
        ("list".into(), Val::Array(vec![
            Val::Str("member1".into()),
            Val::Str("member2".into())
        ])),
        ("subobj".into(), Val::Object(HashMap::from([
            ("subkey".into(), Val::Str("val".into()))
        ])))
    ]);
    let proper_block: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Str("val".to_string())),
        ("num".into(), Val::Int(42)),
        ("bool".into(), Val::Bool(true)),
        ("none".into(), Val::Null),
        ("list".into(), Val::Array(vec![
            Val::Str("member1".into()), 
            Val::Str("member2".into())
        ])),
        ("inline list".into(), Val::Array(vec![
            Val::Str("member1".into()),
            Val::Str("member2".into()),
            Val::Int(99),
            Val::Bool(false),
            Val::Object(HashMap::from([
                ("member key".into(), Val::Str("val".into())),
                ("ruhhhh".into(), Val::Array(vec![
                    Val::Str("nuh uh".into())
                ]))
            ])),
            Val::Null
        ])),
        ("float".into(), Val::Float(6752.88)),
        ("subobj".into(), Val::Object(HashMap::from([
            ("subkey".into(), Val::Str("val".into())),
            ("sublist".into(), Val::Array(vec![
                Val::Str("only member".into())
            ]))
        ])))
    ]);

    eprintln!("Inline parsed hashmap: {:#?}", inline_parsed.clone());
    eprintln!("Expected inline hashmap: {:#?}", proper_inline.clone());
    eprintln!("Block parsed hashmap: {:#?}", block_parsed.clone());
    eprintln!("Proper block hashmap: {:#?}", proper_block.clone());

    assert_eq!(proper_inline.get("key".into()), inline_parsed.get("key".into()));
    assert_eq!(proper_inline.get("num".into()), inline_parsed.get("num".into()));
    assert_eq!(proper_inline.get("bool".into()), inline_parsed.get("bool".into()));
    assert_eq!(proper_inline.get("none".into()), inline_parsed.get("none".into()));
    assert_eq!(proper_inline.get("list".into()), inline_parsed.get("list".into()));
    assert_eq!(proper_inline.get("subobj".into()), inline_parsed.get("subobj".into()));

    assert_eq!(proper_block.get("key".into()), block_parsed.get("key".into()));
    assert_eq!(proper_block.get("num".into()), block_parsed.get("num".into()));
    assert_eq!(proper_block.get("bool".into()), block_parsed.get("bool".into()));
    assert_eq!(proper_block.get("none".into()), block_parsed.get("none".into()));
    assert_eq!(proper_block.get("list".into()), block_parsed.get("list".into()));
    assert_eq!(proper_block.get("inline list".into()), block_parsed.get("inline list".into()));
    assert_eq!(proper_block.get("float".into()), block_parsed.get("float".into()));
    assert_eq!(proper_block.get("subobj".into()), block_parsed.get("subobj".into()));
}

#[test]
#[should_panic]
fn space_is_control() {
    assert!(' '.is_control());
}

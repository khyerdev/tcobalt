#[test]
fn args_help() {
    use crate::args::*;

    let help_args = Args::override_args(&["help"]).parse().unwrap();
    let help_get_args = Args::override_args(&["help", "get"]).parse().unwrap();
    let help_list_args = Args::override_args(&["help", "list"]).parse().unwrap();
    let help_bulk_args = Args::override_args(&["help", "bulk"]).parse().unwrap();
    let help_examples = Args::override_args(&["help", "examples"]).parse().unwrap();

    assert_eq!(help_args.help_flag, Some(types::Help::Help));
    assert_eq!(help_get_args.help_flag, Some(types::Help::Get));
    assert_eq!(help_list_args.help_flag, Some(types::Help::List));
    assert_eq!(help_bulk_args.help_flag, Some(types::Help::Bulk));
    assert_eq!(help_examples.help_flag, Some(types::Help::Examples));
}

#[test]
fn args_get() {
    use crate::args::*;
    let url = "https://www.youtube.com/watch?v=zn5sTDXSp8E";

    let args1 = Args::override_args(&["get", url]).parse().unwrap();
    let args2 = Args::override_args(&["get", url, "--vcodec", "av1"]).parse().unwrap();
    let args3 = Args::override_args(&["get", "--vquality", "1440", url]).parse().unwrap();
    let args4 = Args::override_args(&["get", "-cq", "vp9", "720", url]).parse().unwrap();
    let args5 = Args::override_args(&["get", url, "-af", "ogg", "--output", "foo.ogg"]).parse().unwrap();
    let args6 = Args::override_args(&["get", url, "-gmo", "bar.gif"]).parse().unwrap();

    assert_eq!(args1.method, Some(types::Method::Get));
    assert_eq!(args1.c_url, Some(url.to_string()));
    assert_eq!(args1.c_video_codec, types::VideoCodec::H264);
    assert_eq!(args1.c_video_quality, 1080);
    assert_eq!(args1.c_audio_format, types::AudioFormat::MP3);
    assert_eq!(args1.c_audio_only, false);
    assert_eq!(args1.c_audio_muted, false);
    assert_eq!(args1.c_twitter_gif, false);

    assert_eq!(args2.c_video_codec, types::VideoCodec::AV1);
    assert_eq!(args3.c_video_quality, 1440);
    assert_eq!(args3.c_url, Some(url.to_string()));
    assert_eq!(args4.c_video_codec, types::VideoCodec::VP9);
    assert_eq!(args4.c_video_quality, 720);
    assert_eq!(args5.c_audio_only, true);
    assert_eq!(args5.c_audio_format, types::AudioFormat::OGG);
    assert_eq!(args5.out_filename, Some("foo.ogg".into()));
    assert_eq!(args6.c_twitter_gif, true);
    assert_eq!(args6.c_audio_muted, true);
    assert_eq!(args6.out_filename, Some("bar.gif".into()));
}

#[test]
fn args_get_incorrect() {
    use crate::args::*;
    let url = "https://www.youtube.com/watch?v=zn5sTDXSp8E";

    Args::override_args(&["get", url, url]).parse().unwrap_err();
    Args::override_args(&["get", "-q", "1080"]).parse().unwrap_err();
    Args::override_args(&["get", "-q", "1081", url]).parse().unwrap_err();
    Args::override_args(&["get", "-q", url]).parse().unwrap_err();
    Args::override_args(&["get", url, "-af"]).parse().unwrap_err();
    Args::override_args(&["get", url, "-cafamgo"]).parse().unwrap_err();
    Args::override_args(&["get"]).parse().unwrap_err();
}

#[test]
fn args_bulk_get() {
    use crate::args::*;
    let url1 = "https://www.youtube.com/watch?v=zn5sTDXSp8E";
    let url2 = "https://www.youtube.com/watch?v=OnrbdAAokS0";

    let bulk1 = Args::override_args(&["bulk", "get", url2, url1,]).parse().unwrap();
    let mut dummy_get_1 = Args::override_args(&["get", "https://"]).parse().unwrap();
    dummy_get_1.c_url = Some(url1.into());
    let mut dummy_get_2 = Args::override_args(&["get", "https://"]).parse().unwrap();
    dummy_get_2.c_url = Some(url2.into());
    eprintln!("{:#?}", bulk1.bulk_array.clone().unwrap());
    assert_eq!(bulk1.bulk_array.clone().unwrap()[0], dummy_get_1);
    assert_eq!(bulk1.bulk_array.unwrap()[1], dummy_get_2);
    
    let bulk2 = Args::override_args(&["bulk", "get", url2, "-cqm", "av1", "1440", url1,]).parse().unwrap();
    let mut dummy_get_1 = Args::override_args(&["get", "https://", "-cqm", "av1", "1440"]).parse().unwrap();
    dummy_get_1.c_url = Some(url1.into());
    let mut dummy_get_2 = Args::override_args(&["get", "https://", "-cqm", "av1", "1440"]).parse().unwrap();
    dummy_get_2.c_url = Some(url2.into());
    eprintln!("{:#?}", bulk2.bulk_array.clone().unwrap());
    assert_eq!(bulk2.bulk_array.clone().unwrap()[0], dummy_get_1);
    assert_eq!(bulk2.bulk_array.unwrap()[1], dummy_get_2);
}

#[test]
fn json_parse() {
    use std::collections::HashMap;
    use crate::json::{self, JsonValue as Val};

    let inline_str = "{'key':\"val\",\"num\":42,\"bool\":true,'none':null,\"list\":[\"member1\",\"member2\"],\"subobj\":{\"subkey\":\"val\"}}";
    let block_str = "{
        \"key\": \"val\",
        \"num\": 42,
        \"bool\": true,
        \"none\": null,
        \"list\": [
            \"member1\",
            \"member \\\"two\\\"\"
        ],
        \"inline list\": [\"member1\", 'member2', 99, false, {\"member key\": \"val\", \"ruhhhh\": [\"nuh uh\"]}, null],
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
            Val::Str("member \"two\"".into())
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
// #[ignore]
fn empty_json() {
    use std::collections::HashMap;
    use crate::json::{self, JsonValue as Val};

    let empty_json = "{}";
    let empty_object = "{\"key\":{}}";
    let empty_array = "{\"key\":[]}";
    let obj_empty_array = "{\"key\":{\"arr1\":[],\"arr2\":[]}}";
    let arr_empty_object = "{\"key\":[{},2,{}]}";

    let parsed_ej = json::parse(empty_json).unwrap();
    let parsed_eo = json::parse(empty_object).unwrap();
    let parsed_ea = json::parse(empty_array).unwrap();
    let parsed_oea = json::parse(obj_empty_array).unwrap();
    let parsed_aeo = json::parse(arr_empty_object).unwrap();

    let proper_ej: HashMap<String, Val> = HashMap::new();
    let proper_eo: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Object(HashMap::new()))
    ]);
    let proper_ea: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Array(Vec::new()))
    ]);
    let proper_oea: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Object(HashMap::from([
            ("arr1".into(), Val::Array(Vec::new())),
            ("arr2".into(), Val::Array(Vec::new()))
        ])))
    ]);
    let proper_aeo: HashMap<String, Val> = HashMap::from([
        ("key".into(), Val::Array(vec![
            Val::Object(HashMap::new()),
            Val::Int(2),
            Val::Object(HashMap::new())
        ]))
    ]);

    eprintln!("proper ej: {proper_ej:?}");
    eprintln!("proper eo: {proper_eo:?}");
    eprintln!("proper ea: {proper_ea:?}");
    eprintln!("proper oea: {proper_oea:?}");
    eprintln!("proper aeo: {proper_aeo:?}");

    assert_eq!(proper_ej, parsed_ej);
    assert_eq!(proper_eo, parsed_eo);
    assert_eq!(proper_ea, parsed_ea);
    assert_eq!(proper_oea, parsed_oea);
    assert_eq!(proper_aeo, parsed_aeo);
}

#[test]
fn incorrect_json() {
    use crate::json;

    let missing_val_1 = "{\"key\"}";
    let missing_val_2 = "{\"key\":}";
    let missing_val_3 = "{\"key\":\"}";
    let missing_bracket_1 = "{\"key\":{\"key1\": \"val\"}";
    let missing_bracket_2 = "{\"key\":\"key1\": \"val\"}}";
    let missing_bracket_3 = "{\"key\":[\"val1\", \"val2\"}";
    let missing_bracket_4 = "{\"key\":\"key1\", \"val\"]}";
    let no_comma_1 = "{\"key\": [\"val1\" \"val2\"]}";
    let no_comma_2 = "{\"key\": [\"val1\", \"val2\"] \"key2\": \"val3\"}";
    let no_comma_3 = "{\"key\": {\"key1\": \"val1\"} \"key2\": \"val2\"}";
    let colon_in_array = "{\"key\": [\"foo\", \"bar\": \"baz\"]}";
    let two_colons = "{\"foo\" : \"bar\" : \"baz\"}";

    assert!(json::parse(missing_val_1).is_err());
    assert!(json::parse(missing_val_2).is_err());
    assert!(json::parse(missing_val_3).is_err());
    assert!(json::parse(missing_bracket_1).is_err());
    assert!(json::parse(missing_bracket_2).is_err());
    assert!(json::parse(missing_bracket_3).is_err());
    assert!(json::parse(missing_bracket_4).is_err());
    assert!(json::parse(no_comma_1).is_err());
    assert!(json::parse(no_comma_2).is_err());
    assert!(json::parse(no_comma_3).is_err());
    assert!(json::parse(colon_in_array).is_err());
    assert!(json::parse(two_colons).is_err());
}

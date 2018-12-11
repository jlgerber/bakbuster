extern crate bakbuster;
extern crate chrono;
use self::chrono::{Local, NaiveDateTime};
use std::str::FromStr;
use std::path::{Path, PathBuf};
use bakbuster::{get_file_version_on, FileVersion, BAKTIMEFMT, stack_history_from_path};

#[macro_use] mod common;

fn setup() {}

test! {
    stack_history_parser {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <elt is_current="False" version="20161213-093146_r575055" />
    <elt is_current="False" version="20181102-144204" />
    <elt is_current="True" version="20181105-103813" />
    <elt is_current="False" version="20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), Local::now().naive_local());
        let expected = FileVersion::from_str("20181105-103813");
        assert_eq!(result, expected);
    }
}


test! {
    stack_history_parser_from_early {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <elt is_current="False" version="20161213-093146_r575055" />
    <elt is_current="False" version="20181102-144204" />
    <elt is_current="True" version="20181105-103813" />
    <elt is_current="False" version="20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDateTime::parse_from_str("20181102-144204", BAKTIMEFMT).unwrap());
        let expected = FileVersion::from_str("20181102-144204");
        assert_eq!(result, expected);
    }
}


test! {
    stack_history_parser_from_late {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <elt is_current="False" version="20161213-093146_r575055" />
    <elt is_current="False" version="20181102-144204" />
    <elt is_current="True" version="20181105-103813" />
    <elt is_current="False" version="20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDateTime::parse_from_str("20181106-104603", BAKTIMEFMT).unwrap());
        let expected = FileVersion::from_str("20181105-103813");
        assert_eq!(result, expected);
    }
}

test! {
    stack_history_from_path_given_str {
        let result = stack_history_from_path("./foo_preference.yaml");
        let expect =
        PathBuf::from("./bak/foo_preference.yaml/foo_preference.yaml_swinstall_stack".to_string());
        assert_eq!(result, Ok(expect));
    }
}


test! {
    stack_history_from_path_given_pathbuf {
        let pb = PathBuf::from("./foo_preference.yaml".to_string());
        let result = stack_history_from_path(pb);
        let expect =
        PathBuf::from("./bak/foo_preference.yaml/foo_preference.yaml_swinstall_stack".to_string());
        assert_eq!(result, Ok(expect));
    }
}


test! {
    stack_history_from_path_given_path {
        let p = Path::new("./foo_preference.yaml");
        let result = stack_history_from_path(p);
        let expect =
        PathBuf::from("./bak/foo_preference.yaml/foo_preference.yaml_swinstall_stack".to_string());
        assert_eq!(result, Ok(expect));
    }
}
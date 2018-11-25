extern crate bakbuster;
extern crate chrono;
use self::chrono::{Local, NaiveDateTime};
use std::str::FromStr;

use bakbuster::{get_file_version_on, FileVersion, BAKTIMEFMT};

#[macro_use] mod common;

fn setup() {}

test! {
    stack_history_parser {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
    <alt is_current="False" version="packages.xml.20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), Local::now().naive_local());
        let expected = FileVersion::from_str("packages.xml.20181105-103813");
        assert_eq!(result, expected);
    }
}


test! {
    stack_history_parser_from_early {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
    <alt is_current="False" version="packages.xml.20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDateTime::parse_from_str("20181102-144204", BAKTIMEFMT).unwrap());
        let expected = FileVersion::from_str("packages.xml.20181102-144204");
        assert_eq!(result, expected);
    }
}


test! {
    stack_history_parser_from_late {
let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
    <alt is_current="False" version="packages.xml.20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDateTime::parse_from_str("20181106-104603", BAKTIMEFMT).unwrap());
        let expected = FileVersion::from_str("packages.xml.20181105-103813");
        assert_eq!(result, expected);
    }
}
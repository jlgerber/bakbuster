//! stack_history_parser
//!
//! Exports a public method, stack_history_from_path, which retrieves the path to
//! the swinstall stack file for the supplied swinstalled file. Or an error of course.
use chrono::prelude::*;
use chrono::naive::NaiveDateTime;
use errors::BBError;
use fileversionparser::FileVersion;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use xml::{
    reader::EventReader,
    reader::XmlEvent,
    attribute::OwnedAttribute
};

/// Given a path to an swinstalled file, return the path to its swinstall_stack file
/// within the bak directory.
///
/// # Example
///
/// Given:
///
///  ```text,ignore
/// ./foo_preference.yaml
/// ```
///
/// Return:
///
/// ```text,ignore
/// ./bak/foo_preference.yaml/foo_preference.yaml_swinstall_stack
/// ```
///
/// as a PathBuf wrapped in a result.
pub fn stack_history_from_path<F: Into<PathBuf>>(file: F) -> Result<PathBuf, BBError> {
    let mut pb = file.into();
    let filename = pb.file_name().ok_or(BBError::ConversionError(format!("Unable to get filename from '{:?}'", pb)))?
                   .to_str().ok_or(BBError::ConversionError("failed to convert filename  to str".to_string()))?
                   .to_string();
    // remove filename
    pb.pop();

    // build up path to swinstall stack file
    // ie given
    //   ./foo.yaml
    // return
    //   ./bak/foo.yaml/foo.yaml_swinstall_stack
    pb.push("bak");
    pb.push(filename.as_str());
    let swinstall_stack = format!("{}_swinstall_stack", filename.as_str());
    pb.push(swinstall_stack);
    Ok(pb)
}

/// Given an input which implements the Read trait, and a datetime, find the
/// latest FileVersion at or before the supplied datetime, which is also prior to or
/// at the current file. (ie no fileversions after the one marked as current will be considered)
///
/// # Example
/// ```rust,ignore
/// let filehandle = File::open(file).unwrap();
/// let file = BufReader::new(filehandle);
/// let result = get_file_version_on(file, Local::now().naive_local());
/// ```
pub fn get_file_version_on<R: Read>(input: R, datetime: NaiveDateTime) -> Result<FileVersion, BBError> {

    let parser = EventReader::new(input);

    // set initial state variables

    // we upbdate file_version only if we find one which matches the
    // datetime constraint
    let mut file_version: Option<FileVersion> = None;
    // if an elt tag is marked current, we set this variable to true
    let mut current = false;

    for xml_event in parser {
        match xml_event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name.as_str() == "elt" {

                    let mut matched_version = false;
                    for attr in attributes {
                        let namestr = attr.name.local_name.as_str();
                        match namestr {
                            "is_current" => {
                                current = match_current_str(attr.value.as_str())?;
                            },
                            "version" => {
                                matched_version = true;
                                update_file_version_if_in_range(attr.value.as_str(),&mut file_version, &datetime)?;
                            },
                            _ => {
                                return Err(BBError::ParseError(format!("attribute {} not valid", namestr)))
                            }
                        }
                        // we need to wait until we have matched the version attribute as well as
                        // evaluated whether we are current. Otherwise, we will return early
                        if current && matched_version {
                            // if the elt tag has attribute is_current=true and we have
                            // a matched version in terms of time, we return early. This covers
                            // the case where someone has rolled back an install, and there are
                            // additional elt tags after the current one which would potentially match the
                            // datetime constraint.
                            return file_version
                                   .ok_or(BBError::ParseError("No fileversion found".to_string()));
                        }
                    }
                }
            },

            Err(e) => {
                return Err(BBError::ParseError(format!("problem: {}",e)));
            },
            _ => {}
        }
    }

    if current {
        return file_version.ok_or(BBError::ParseError("No fileversion found".to_string()));
    }

    Err(BBError::ParseError("No current fileversion found".to_string()))
}

// Given a str reference, convert it to a bool. If successful
// return Ok of bool. If unsuccessful, return an Err of BBError.
fn match_current_str(current: &str) -> Result<bool, BBError> {
    match current {
        "True" | "true" => Ok(true),
        "False" | "false" => Ok(false),
        _ => Err(BBError::ParseError(
            format!("Unable to parse is_current value : '{}'", current)
        )),
    }
}

// Given the name of a file as a &str, a mutable Option wrapped FileVersion instance, and
// a reference to a NaiveDateTime, update the FileVersion with a new FileVersion
// constructed from the file name &str, if the value of the datetime is greater than or
// equal to the datetime of the FileVersion built from the supplied file name.
// Of course, things can fail. For instance, the FileVersion::from_str call can return a
// BBError. So, the method returns a Result wrapping a unit (), on success, and a BBError,
// on failure.
fn update_file_version_if_in_range(
    file_name: &str,
    file_version: &mut Option<FileVersion>,
    datetime: &NaiveDateTime)
-> Result<(),BBError> {
    let fv = FileVersion::from_str(file_name)?;
    if fv.date_time <= *datetime {
        debug!("version: {}", fv);
        *file_version = Some(fv);
    }
    Ok(())
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
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


    #[test]
    fn parse_old() {
        let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <elt is_current="False" version="20161213-093146_r575055" />
    <elt is_current="False" version="20181102-144204" />
    <elt is_current="True" version="20181105-103813" />
    <elt is_current="False" version="20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDate::from_ymd(2018,11,2).and_hms(14,42,4));
        let expected = FileVersion::from_str("20181102-144204");
        assert_eq!(result, expected);
    }
}
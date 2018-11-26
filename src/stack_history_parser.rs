
use chrono::prelude::*;
use chrono::naive::NaiveDateTime;
use errors::BBError;
use fileversionparser::FileVersion;
use std::io::Read;
use xml::reader::{EventReader, XmlEvent};
use std::str::FromStr;
use std::path::PathBuf;

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
    let filename = match pb.file_name() {
        Some(fname) => match fname.to_str(){
            Some(n) => n.to_string(),
            None => {return Err(BBError::ConversionError("failed to convert filename  to str".to_string()))},
        },
        None => {return Err(BBError::ConversionError(format!("Unable to get filename from '{:?}'", pb)))},
    };
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
    let mut file_version: Option<FileVersion> = None;
    let mut current = false;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name.as_str() == "alt" {
                    let mut matched_version = false;
                    for attr in attributes {
                        let namestr = attr.name.local_name.as_str();

                        match namestr {
                            "is_current" => {
                                current = if attr.value == "True" {
                                    true
                                } else if attr.value == "False" {
                                    false
                                } else {
                                    return Err(BBError::ParseError(
                                        format!("Unable to parse is_current value : '{}'", attr.value)
                                    ));
                                };
                            },
                            "version" => {
                                let fv = FileVersion::from_str(attr.value.as_str())?;
                                matched_version = true;
                                if fv.date_time <= datetime {
                                    debug!("version: {}", fv);
                                    file_version = Some(fv);
                                }
                            },
                            _ => {
                                return Err(BBError::ParseError(format!("attribute {} not valid", namestr)))
                            }
                        }
                        // we need to wait until we have matched the version attribute as well as
                        // evaluated whether we are current. Otherwise, we will return early
                        if current && matched_version {
                            return match file_version {
                                None => Err(BBError::ParseError("No fileversion found".to_string())),
                                Some(file_version) => {
                                    Ok(file_version)
                                },
                            }
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
        return match file_version {
            None => Err(BBError::ParseError("No fileversion found".to_string())),
            Some(file_version) => {
                Ok(file_version)
            },
        }
    }

    Err(BBError::ParseError("No current fileversion found".to_string()))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
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


    #[test]
    fn parse_old() {
        let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
    <alt is_current="False" version="packages.xml.20181106-104603" />
</stack_history>"#;

        let result = get_file_version_on(xml.as_bytes(), NaiveDate::from_ymd(2018,11,2).and_hms(14,42,4));
        let expected = FileVersion::from_str("packages.xml.20181102-144204");
        assert_eq!(result, expected);
    }
}
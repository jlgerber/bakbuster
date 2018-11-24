use pest::Parser;
use errors::BBError;
use chrono::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileVersion {
    pub name: String,
    pub extension: String,
    pub date_time: NaiveDateTime,
    pub revision: Option<String>
}

impl FileVersion {
    pub fn new(name: String, extension: String, date_time: NaiveDateTime, revision: Option<String>) -> FileVersion {
        FileVersion {
            name,
            extension,
            date_time,
            revision,
        }
    }

    pub fn from_str(name: &str) -> Result< FileVersion, BBError> {
        FileVersionParser::parse(name)
    }
}

impl fmt::Display for FileVersion {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let revision =
        if let Some(ref revision) = self.revision {
            format!("_r{}", revision)
        } else {
            "".to_string()
        };

        write!(f, "({}.{}.{}{}{}-{}{}{}{})",
            self.name,
            self.extension,
            self.date_time.year(),self.date_time.month(), self.date_time.day(),
            self.date_time.hour(), self.date_time.minute(), self.date_time.second(),
            revision
        )
    }
}

// The pest parser is not exposed directly.
#[derive(Parser)]
#[grammar = "fileversion.pest"]
struct _FileVersionParser;

// IndexParser is a convenience struct which provides a parse method that is more suited
// to the api than the raw pest _IndexParser.

/// A dataless struct which provides an api for parsing an Index from an input &str
pub struct FileVersionParser;

impl FileVersionParser {
    /// parse an elasticsearch index, of the form ```name-YYYY.MM.DD``` and return
    /// a Result - either an Ok Index instance, or an Err String.
    pub fn parse(input: &str ) -> Result<FileVersion, BBError> {
        let file_version =  _FileVersionParser::parse(Rule::fileversion, input).map_err(|e| BBError::ParseError(format!("unable to parse: '{}' error: '{}'",input, e)))?;

        // parsing guarantees that these vars are going to get set. we just choose arbitrary
        // values for now.
        let mut name = None;
        let mut extension = None;
        let mut year = None;
        let mut month = None;
        let mut day = None;
        let mut hour = None;
        let mut minute = None;
        let mut second = None;
        let mut revision = None;

        for idx_piece in file_version {

            // A idx_piece can be converted to an iterator of the tokens which make it up:
            for inner_idx_piece in idx_piece.into_inner() {
                let inner_span = inner_idx_piece.clone().into_span();

                match inner_idx_piece.as_rule() {
                    Rule::base => {
                        name = Some(inner_span.as_str().to_string());
                    },
                    Rule::extension => {
                        extension = Some(inner_span.as_str().to_string());
                    },
                    Rule::revision => {
                        for revision_piece in inner_idx_piece.into_inner() {
                            let inner_span = revision_piece.clone().into_span();
                            match revision_piece.as_rule() {
                                Rule::revision_id  => {
                                    revision = Some(inner_span.as_str().to_string());
                                },
                                _ => unreachable!()
                            }
                        }
                    },
                    Rule::datetime => {
                        for date_piece in inner_idx_piece.into_inner() {
                            let inner_span = date_piece.clone().into_span();
                            match date_piece.as_rule() {
                                Rule::year  => {
                                    year = Some(inner_span.as_str().parse::<i32>().unwrap());
                                },
                                Rule::month => {
                                    month = Some(inner_span.as_str().parse::<u32>().unwrap());
                                },
                                Rule::day   => {
                                    day = Some(inner_span.as_str().parse::<u32>().unwrap());
                                },
                                Rule::hour => {
                                    hour = Some(inner_span.as_str().parse::<u32>().unwrap());
                                },
                                Rule::minute => {
                                    minute = Some(inner_span.as_str().parse::<u32>().unwrap());
                                },
                                Rule::second => {
                                    second = Some(inner_span.as_str().parse::<u32>().unwrap());
                                },
                                _ => unreachable!()
                            }
                        }
                    },
                    _ => unreachable!()
                };
            }
        }
        let dt =
           NaiveDate::from_ymd(year.unwrap(), month.unwrap(), day.unwrap())
              .and_hms(hour.unwrap(), minute.unwrap(), second.unwrap());

        let fileversion = FileVersion::new(name.unwrap(), extension.unwrap(), dt, revision);
        Ok(fileversion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_file_version() {
        let fvstr = "packages.xml.20181105-103813";
        let fv = FileVersion::from_str(&fvstr);
        let expect = FileVersion {
            name: "packages".to_string(),
            extension: "xml".to_string(),
            date_time: NaiveDate::from_ymd(2018, 11, 5).and_hms(10,38,13),
            revision: None,
        };
        assert_eq!(fv, Ok(expect));
    }

    #[test]
    fn parse_file_version_revision() {
        let fvstr = "packages.xml.20181105-103813_r12431345";
        let fv = FileVersion::from_str(&fvstr);
        let expect = FileVersion {
            name: "packages".to_string(),
            extension: "xml".to_string(),
            date_time: NaiveDate::from_ymd(2018, 11, 5).and_hms(10,38,13),
            revision: Some("12431345".to_string()),
        };
        assert_eq!(fv, Ok(expect));
    }
}
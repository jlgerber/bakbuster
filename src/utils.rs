use errors::BBError;
use std::env;
use std::path::PathBuf;

/// Retrieve the current exe directory as a PathBuf or return a ConversionError
pub fn path_to_executable() -> Result<PathBuf, BBError> {
    let this_file = env::current_exe().map_err(|e| BBError::ConversionError(format!("Unable to get current_exe: {}",e)))?;
    //let this_file = this_file.into_os_string().into_string().map_err(|os_str| BBError::ConversionError(format!("Unable to convert {:?} to string", os_str)) )?;
    Ok(this_file)
}

///Convert path to string. Takes a Path or PathBuf and taeks ownership before converting to string
pub fn pathbuf_to_string<I: Into<PathBuf>>(path: I) -> Result<String, BBError> {
    let pathbuf = path.into();
    let pb = pathbuf.into_os_string().into_string().map_err(|os_str| BBError::ConversionError(format!("Unable to convert {:?} to string", os_str)) )?;
    Ok(pb)
}

/// Returns the path to the executable as a string, or returns a BBError::ConversionError if failure encountered.
pub fn path_to_executable_string() -> Result<String, BBError> {
    let pb = path_to_executable()?;
    let pb = pathbuf_to_string(pb)?;
    Ok(pb)
}


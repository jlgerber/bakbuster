extern crate bakbuster;
extern crate chrono;
#[macro_use] extern crate log;
extern crate env_logger;

//use bakbuster::*;

//use std::fs;
//use std::env;
//use std::io::Read;
//use std::process;
use chrono::Local;
use bakbuster::stack_history_parser::*;
use env_logger::Env;
use bakbuster::utils::*;

fn main() {
    let env =
    Env::default()
    .filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

     let xml =
r#"<stack_history path="/dd/facility/etc/bak/packages.xml/packages.xml_swinstall_stack">
    <alt is_current="False" version="packages.xml.20161213-093146_r575055" />
    <alt is_current="False" version="packages.xml.20181102-144204" />
    <alt is_current="True" version="packages.xml.20181105-103813" />
    <alt is_current="False" version="packages.xml.20181106-104603" />
</stack_history>"#;

    let result = get_file_version_on(xml.as_bytes(), Local::now().naive_local());
    info!("get_file_version_on(...) results: {:?}", result);

    let dt = Local::now();
    let ndt = dt.naive_local();
    info!("{:?} {:?}", dt, ndt);
    //let this_file = std::env::current_exe().unwrap(); //have to split into two lines for lifetime issue.
    //let this_file = this_file.as_os_str().to_str().unwrap();
    let this_file = path_to_executable().unwrap();
    let this_file = pathbuf_to_string(this_file).unwrap();
    println!("executable path: {}", this_file);
    let mut first = this_file.split("bakbuster");
    let next = first.next().unwrap();
    println!("defined in file: {}/backbuster", next);
}


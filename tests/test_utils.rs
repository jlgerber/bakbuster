extern crate bakbuster;
use bakbuster::utils::pathbuf_to_string;

use std::path::{Path, PathBuf};

#[macro_use] mod common;

fn setup() {}

test! {
   pathbuf_to_string_using_pathbuf {
       let p = PathBuf::from(r"/foo/bar");
       let pstr = pathbuf_to_string(p);
       assert_eq!(pstr, Ok("/foo/bar".to_string()));
   }
}

test! {
    pathbuf_to_string_using_path {
        let p = Path::new("/foo/bar");
        let pstr = pathbuf_to_string(p);
        assert_eq!(pstr, Ok("/foo/bar".to_string()));
    }
}
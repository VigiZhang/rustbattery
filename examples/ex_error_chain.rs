#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors {
    #![allow(deprecated)] // std::error::Error::cause is deprecated since 1.33.0. see https://github.com/rust-lang-nursery/error-chain/issues/254
    error_chain!{
        foreign_links {
            Io(std::io::Error);
        }
    }
}

use errors::*;
use std::fs::File;

fn run() -> Result<()> {
    let _file = File::open("unknown_file")?;
    Ok(())
}

quick_main!(run);

pub use std::collections::HashMap;
pub use std::io::{self, Write,Read};
use std::fs::File;
pub use encoding_rs::UTF_8;
use hex::FromHex;
pub use colored::Colorize;
mod incl {
    pub mod interpreter;
    pub mod formatter;
    pub mod basicfunctions;
}
pub use incl::interpreter::*;
pub use incl::formatter::*;
pub use incl::basicfunctions::*;
pub use std::sync::{mpsc, Arc, Mutex};
pub use std::thread;
pub use std::env;
// test
fn main() {
    testmain();
}

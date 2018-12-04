extern crate rust_bucket;

use rust_bucket::dom::{Attrs, Element, Node};
use rust_bucket::html_parser::parse_html_file;
use rust_bucket::*;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("src/page.html").expect("cannot read file");

    let html: Node = parse_html_file(&unparsed_file).expect("unsuccessful parse");

    // dom::pretty_print(&html, 1)
}

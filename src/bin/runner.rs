extern crate rust_bucket;

use rust_bucket::dom::{Document, pretty_print};
use rust_bucket::html_parser::{parse_document};
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("src/page.html").expect("cannot read file");

    let _html: Document = parse_document(&unparsed_file).expect("unsuccessful parse");

    pretty_print(&_html, 1)
}

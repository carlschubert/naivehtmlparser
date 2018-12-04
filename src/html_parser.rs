use dom::{Attrs, Element, Node};

use pest::Parser;

use pest::iterators::{Pair, Pairs};

use pest::error::Error;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

pub fn parse_html_file(file: &str) -> Result<Node, Error<Rule>> {
    let html = HTMLParser::parse(Rule::document, file)?;
    // println!("{:#?}", html);
    let nodes = parse_pairs(html);
    // println!("{:#?}", nodes);
    nodes
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<Node, Error<Rule>> {
    // println!("----------------------");

    // println!("pairs {:#?}", &pairs);
    // println!("----------------------");

    let mut foo;
    for pair in pairs {
        foo = match pair.as_rule() {
            Rule::element => {
                // println!("{:?} {}", pair.as_rule(), pair.as_str());

                Node::Element((
                    Element.new("foo"),
                    pair.map(parse_pairs).collect(),
                ))
            }
            Rule::text => {
                // println!("{:?} {}", pair.as_rule(), pair.as_str());

                Node::Text(pair.as_str().to_string())
            }
            Rule::document => parse_pairs(pair.into_inner())?,
            Rule::tag => parse_pairs(pair.into_inner())?,
            Rule::insides => parse_pairs(pair.into_inner())?,
            _ => {
                // println!("{:?} {}", pair.as_rule(), pair.as_str());
                ()
            }
        };
        //println!("foo {:#?}", &foo);
    }

    Ok(foo)
}

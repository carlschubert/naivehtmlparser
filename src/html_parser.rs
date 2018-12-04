use dom::*;

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

fn parse_document(pairs: Pairs<Rule>) -> Result<Document, Error<Rule>> {
    Ok(Document::Strict((
        Node::Doctype("html".to_string()),
        Node::Element((Element::new("html", None), vec![])),
    )))
}

fn parse_pair(pair: Pair<Rule>) -> Result<Node, Error<Rule>> {
    Ok(Node::Text("Shut up".to_string()))
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
                    Element::new("foo", None),
                    pair.into_inner().map(parse_pair).collect::<Result<Vec<Node>, Error<Rule>>>()?,
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
                Node::Text("Shut up".to_string())
            }
        };
        //println!("foo {:#?}", &foo);
    }

    Ok(Node::Text("foo".to_string()))
}

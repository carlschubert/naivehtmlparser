use dom::*;

use pest::Parser;

use pest::iterators::{Pair, Pairs};

use errors::Error;

use errors::invalid_html;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

pub fn parse_document(file: &str) -> Result<Document, Error> {
    let html: Vec<Pair<Rule>> = HTMLParser::parse(Rule::document, file)?.collect();

    if html.len() > 2 {
        return Err(invalid_html());
    }
    let root: Node;
    if html.len() == 2 {
        let doctype: Node;

        let mut i = 0;

        match html[i].as_rule() {
            Rule::doctype => doctype = Node::Doctype(html[i].as_str().to_string()),
            _ => return Err(invalid_html()),
        }

        i += 1;
        match html[i].as_rule() {
            Rule::element => root = parse_pair(html[i].clone())?,
            _ => return Err(invalid_html()),
        }
        Ok(Document::Strict((doctype, root)))
    } else {
        let i = 0;
        match html[i].as_rule() {
            Rule::document => root = parse_pair(html[i].clone())?,
            _ => return Err(invalid_html()),
        }
        Ok(Document::Quirks(root))
    }
}

fn parse_pair(pair: Pair<Rule>) -> Result<Node, Error> {
    Ok(Node::Text("Shut up".to_string()))
}

fn parse_pairs(pairs: Pairs<Rule>) -> Result<Node, Error> {
    // println!("----------------------");

    // println!("pairs {:#?}", &pairs);
    // println!("----------------------");



        match pair.as_rule() {
            Rule::element => {
                // println!("{:?} {}", pair.as_rule(), pair.as_str());

                Node::Element((
                    Element::new("foo", None),
                    pair.into_inner()
                        .map(parse_pair)
                        .collect::<Result<Vec<Node>, Error>>()?,
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

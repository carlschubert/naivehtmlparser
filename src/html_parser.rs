use dom::*;

use pest::Parser;

use pest::iterators::{Pair, Pairs};

use errors::Error;

use errors::invalid_html;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

pub fn parse_document(file: &str) -> Result<Document, Error> {
    let html = HTMLParser::parse(Rule::document, file)?
        .next()
        .unwrap()
        .into_inner()
        .enumerate();

    let mut strict = false;
    let mut doctype: Node = Node::Doctype("html".to_string());
    let mut root: Node = Node::ElementSingleton(Element::new("html", None));
    for (i, part) in html {
        if i > 2 {
            return Err(invalid_html());
        }
        match part.as_rule() {
            Rule::doctype => {
                strict = true;
                doctype = Node::Doctype(part.as_str().to_string());
            }
            Rule::element => {
                let mut inner = part.into_inner();
                root = Node::Element((
                    Element::new(inner.next().unwrap().as_str(), None),
                    inner
                        .filter(|p| p.as_rule() == Rule::element)
                        .map(parse_pair)
                        .collect::<Result<Vec<Node>, Error>>()?,
                ))
            }
            _ => return Err(invalid_html()),
        }
    }
    if strict {
        Ok(Document::Strict((doctype, root)))
    } else {
        Ok(Document::Quirks(root))
    }
}

fn parse_pair(pair: Pair<Rule>) -> Result<Node, Error> {
    println!("foo {:#?}", &pair.as_rule());

    let foo = match pair.as_rule() {
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
        // Rule::document => parse_pair(pair.into_inner())?,
        Rule::tag => parse_pairs(pair.into_inner())?,
        Rule::insides => parse_pairs(pair.into_inner())?,
        _ => {
            // println!("{:?} {}", pair.as_rule(), pair.as_str());
            Node::Text("Shut up".to_string())
        }
    };
    Ok(foo)
}

fn parse_pairs(mut pairs: Pairs<Rule>) -> Result<Node, Error> {
    Ok(parse_pair(pairs.next().unwrap())?)
}

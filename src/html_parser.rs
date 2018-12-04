use dom::*;

use pest::Parser;

use pest::iterators::{Pair, Pairs};

use pest::error::Error;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

fn parse_document(file: &str) -> Result<Document, Error<Rule>> {
    let html: Vec<Pair<Rule>> = HTMLParser::parse(Rule::document, file)?.collect();
    if html.len() > 2 {
        println!("Oops shouldn't happen");
    }
    let doctype: Node;
    let root: Node;
    let mut i = 0;
    if html.len() == 2 {
        match html[i].as_rule() {
            Rule::doctype => doctype = Node::Doctype(html[i].as_str().to_string()),
            _ => {
                println!("Oops shouldn't happen");
            }
        }
        i += 1;
    }
    match html[i].as_rule() {
        Rule::element => root = parse_pair(html[i])?,
        _ => {
            println!("Oops shouldn't happen");
        }
    }
    if html.len() == 2 {
        Ok(Document::Strict((doctype, root)))
    } else {
        Ok(Document::Quirks(root))
    }
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

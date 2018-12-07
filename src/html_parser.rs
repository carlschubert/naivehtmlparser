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
                doctype = Node::Doctype(part.into_inner()
                        .next()
                        .unwrap() // doctype_type
                        .as_str()
                        .to_string());
            }
            Rule::node => {
                root = parse_node(part)?;
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

fn parse_node(pair: Pair<Rule>) -> Result<Node, Error> {
    let mut children = Nodes::new();
    let mut tagname = "";
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::text => {
                return Ok(Node::Text(inner.as_str().to_string()));
            }
            Rule::comment => return Ok(Node::Comment(inner.as_str().to_string())),
            Rule::tag => {
                for tag_inner in inner.into_inner() {
                    match tag_inner.as_rule() {
                        Rule::opentag => {
                            tagname = tag_inner.into_inner().next().unwrap().as_str();
                        }
                        Rule::closetag => {
                            if tagname != tag_inner.into_inner().next().unwrap().as_str() {
                                return Err(invalid_html());
                            }
                        }
                        Rule::single => {
                            tagname = tag_inner.into_inner().next().unwrap().as_str();
                            children.push(Node::ElementSingleton(Element::new(tagname, None)));
                        }
                        Rule::nodes => {
                            children.extend(
                                tag_inner
                                    .into_inner()
                                    .map(parse_node)
                                    .collect::<Result<Nodes, Error>>()?,
                            );
                        }
                        _ => return Err(invalid_html()),
                    }
                }
            }
            _ => return Err(invalid_html()),
        }
    }
    Ok(Node::Element((Element::new(tagname, None), children)))
}

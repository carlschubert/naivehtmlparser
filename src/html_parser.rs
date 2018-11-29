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
    Ok(Node::Text("Whatever man".to_string()))
}

// fn parse_value(pair: Pair<Rule>) -> Node {
//     // println!("pair {:#?}", &pair);
//     match pair.as_rule() {
//         Rule::element => {
//             let mut inner_pairs = pair.into_inner();
//             println!("tokens {:#?}", &inner_pairs);
//             // println!("inner_pairs {:#?}", &inner_pairs);
//             let mut children: Vec<Node> = Vec::new();
//             while let Some(child) = inner_pairs.next() {
//                 // println!("child {:?}", &child.as_str());
//                 // let closetag = 

//                 children.push(parse_value(child));
//             }

//             // let props = ElementProps::new(inner_pairs.into_inner().as_str().to_string(), None);
                        
//             let props = Element::new("".to_string(), None);

//             Node::Element((props, children))
//         }
//         Rule::text => Node::new(NodeType::Text(pair.as_str().to_string()), Vec::new()),
//         // Rule::tag => NodeType::Element(pair.into_inner().next().unwrap().as_str()),
//         // Rule::closetag => NodeType::Element(pair.into_inner().next().unwrap().as_str()),
//         _ => Node::new(NodeType::Comment(pair.as_str().to_string()), Vec::new())
//     }
// }
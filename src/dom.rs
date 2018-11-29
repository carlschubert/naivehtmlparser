use std::collections::{HashMap, HashSet};
use std::fmt;
use std::convert::AsRef;

pub trait Tree {
    fn tree(&self) -> Node;
}

pub enum Document {
    Strict((Node, Node)),
    Quirks(Node)
}

impl Tree for Document {
    fn tree(&self) -> Node {
        match self {
            Document::Strict((d, e)) => Node::Element((Element::new("NaiveHTMLParserDocument".to_string(), None), vec![d.clone(), e.clone()])),
            Document::Quirks(e) => Node::Element((Element::new("NaiveHTMLParserDocument".to_string(), None), vec![e.clone()])),
        }
    }
}

/*
There are twelve different node types.  I will start with the most common.
*/
#[derive(Clone)]
pub enum Node {
    Doctype(String),
    Text(String),
    Element((Element, Nodes)),
    Comment(String),
}

impl Tree for Node {
    fn tree(&self) -> Node {
        self.clone()
    }
}

pub type Nodes = Vec<Node>;

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Text(t) | Node::Comment(t) | Node::Doctype(t) => write!(f, "TextType: {}", t),
            Node::Element((e, _)) => write!(f, "ElementType: {:?}", e),
        }
    }
}

/*
An element is has a tag and any number of Attrs.  For example a 'div' tag may have a 
class or id attr.
*/
#[derive(Clone)]
pub struct Element {
    tag_name: String,
    attributes: Option<Attrs>,
}

impl Element {
    pub fn new<T>(tag_name: T, attributes: Option<Attrs>) -> Element
    where
        T: AsRef<str>
    {
        Element {
            tag_name: tag_name.as_ref().to_string(),
            attributes,
        }
    }

    //fn get_id(&self) -> Option<&String> {
    //    self.attributes.get("id")
    //}

    //fn get_classes(&self) -> Vec<&str> {
    //     match self.attributes.get("class") {
    //         Some(c) => c.split(' ').collect::<Vec<&str>>(),
    //         None => Vec::new(),
    //     }
    // }
}

pub type Attrs = HashMap<String, String>;

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attr_str = String::new();

        //for (attr, val) in self.attributes.iter() {
        //    attr_str.push_str(&format!("attr: {} val: {}", attr, val));
        //}

        write!(f, "Tag: {} \n Attrs: {}", self.tag_name, attr_str)
    }
}

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Doctype(ref e) => println!("doctype:{}", e),
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
        _ => {}
    }
}
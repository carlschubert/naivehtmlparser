use std::collections::{HashMap, HashSet};
use std::fmt;
use std::convert::AsRef;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node: {:?}, Children: {:?}", self.node_type, self.children)
    }
}

/*
There are twelve different node types.  I will start with the most common.
*/
pub enum NodeType {
    Doctype(String),
    Text(String),
    Element(ElementProps),
    Comment(String),
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeType::Text(t) | NodeType::Comment(t) | NodeType::Doctype(t) => write!(f, "TextType: {}", t),
            NodeType::Element(e) => write!(f, "ElementType: {:?}", e),
        }
    }
}

/*
An element is has a tag and any number of Attrs.  For example a 'div' tag may have a 
class or id attr.
*/
pub struct ElementProps {
    tag_name: String,
    attributes: Option<Attrs>,
}

impl ElementProps {
    pub fn new<T>(tag_name: T, attributes: Option<Attrs>) -> ElementProps
    where
        T: AsRef<str>
    {
        ElementProps {
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

impl fmt::Debug for ElementProps {
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
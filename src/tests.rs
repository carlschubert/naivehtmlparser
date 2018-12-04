use super::*;
use dom::*;

#[test]
fn twoplustwo() {
    let tree = Document::Strict((
        Node::Doctype("html".to_string()),
        Node::Element((
            Element::new("html", None),
            vec![Node::Element((
                Element::new("body", None),
                vec![
                    Node::Element((
                        Element::new("h1", None),
                        vec![Node::Text("Hello Word".to_string())],
                    )),
                    Node::Element((
                        Element::new("p", None),
                        vec![Node::Text("Foo Bar".to_string())],
                    )),
                ],
            ))],
        )),
    ));
    println!("------------------\n");
    pretty_print(&tree, 0);
    println!("\n------------------\n");
    assert!(false);
}

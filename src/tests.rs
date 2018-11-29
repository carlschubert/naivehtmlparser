use super::*;
use dom::*;

#[test]
fn twoplustwo() {
    let tree = Node::new(
        NodeType::Doctype("html".to_string()),
        vec![Node::new(
            NodeType::Element(
                ElementProps::new("html", None)
            ), vec![
                Node::new(
                    NodeType::Element(
                        ElementProps::new("body", None)
                    ),
                    vec![
                        Node::new(
                            NodeType::Element(
                                ElementProps::new("h1", None)
                            ),
                        vec![
                            Node::new(
                                NodeType::Text("Hello World".to_string()),
                                vec![],
                            )
                        ])
                    ],
                ),
            ]
        )
        ]);
    println!("------------------\n");
    pretty_print(&tree, 2);
    println!("\n------------------\n");
    assert!(false);
}
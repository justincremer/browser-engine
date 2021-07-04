#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(PartialEq, Eq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> Self {
        ElementData {
            tag_name,
            attributes,
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Self {
        Node {
            node_type,
            children,
        }
    }

    pub fn pretty_print(&self, indent_size: usize) {
        let indent = (0..indent_size).map(|_| " ").collect::<String>();
        match self.node_type {
            NodeType::Text(ref t) => println!("{}{}", indent, t),
            NodeType::Element(ref e) => println!("{}{:?}", indent, e),
            NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
        }

        for child in self.children.iter() {
            child.pretty_print(indent_size + 2);
        }

        match self.node_type {
            NodeType::Element(ref e) => println!("{}<{}/>", indent, e.tag_name),
            _ => {}
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();
        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        write!(f, "<{}{}>", self.tag_name, attributes_string)
    }
}

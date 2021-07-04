extern crate browser_engine;
use browser_engine::{command, css, css_parser, dom, html_parser, layout, render, style};

use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let nodes = get_html();
    for n in nodes.iter() {
        n.pretty_print(0);
    }

    let ref root_node = nodes[0];

    let stylesheet = get_css();
    println!("{:?}", stylesheet);

    let style_tree_root = style::StyledNode::new(&root_node, &stylesheet);
    style_tree_root.pretty_print(0);

    let mut viewport = layout::Dimensions::default();
    viewport.content.width = 1024.0;
    viewport.content.height = 768.0;

    let layout_tree = layout::layout_tree(&style_tree_root, viewport);
    layout_tree.pretty_print(0);
    println!(
        "Took {} milliseconds to render",
        (Instant::now() - start_time).as_millis()
    );

    let display_commands = command::build_display_commands(&layout_tree);
    render::render_loop(&display_commands);
    println!(
        "Took {} milliseconds to complete",
        (Instant::now() - start_time).as_millis()
    );
}

fn get_html() -> Vec<dom::Node> {
    let mut path = env::current_dir().unwrap();
    path.push("examples/index.html");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };

    let mut html_input = String::new();
    file_reader.read_to_string(&mut html_input).unwrap();

    let nodes = html_parser::HtmlParser::new(&html_input).parse_nodes();
    nodes
}

fn get_css() -> css::Stylesheet {
    let mut path = env::current_dir().unwrap();
    path.push("examples/index.css");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}, error: {}", path.display(), e),
    };

    let mut css_input = String::new();
    file_reader.read_to_string(&mut css_input).unwrap();

    let stylesheet = css_parser::CssParser::new(&css_input).parse_stylesheet();
    stylesheet
}

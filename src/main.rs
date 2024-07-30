use chem::{
    self,
    html::{node::HTMLNode, HTMLNodeT},
};
fn main() {
    let text = HTMLNode::create_text("text");
    let anchor = HTMLNode::create_anchor(Some("hello, world"), Some("/about"));
    anchor.add_child(&text).expect("");
    println!("{}", anchor.render());
}

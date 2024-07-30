use chem::{
    self,
    html::{node::HTMLNode, HTMLNodeT},
};
fn main() {
    let text = HTMLNode::create_text("text");
    println!("{}", text.render());
}

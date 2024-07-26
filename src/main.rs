use chem::{
    self,
    html::{
        node::element::{anchor::AnchorBuilder, GeneralAttributeBuilderT},
        HTMLNodeT,
    },
};

fn main() {
    let a1 = AnchorBuilder::default()
        .set_href("/about")
        .add_class("page-link")
        .add_class("highlight")
        .set_content("About")
        .add_data_attr("role", "navigation")
        .build();

    println!("{}", a1.render())
}

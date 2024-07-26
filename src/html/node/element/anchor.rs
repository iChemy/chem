use std::{cell::RefCell, rc::Rc};

use crate::html::{node::HTMLNode, HTMLNodeBaseInner, HTMLNodeInnerT, HTMLNodeT};

use super::{ElementInner, GeneralAttribute, GeneralAttributeBuilder, GeneralAttributeBuilderT};

pub struct AnchorElementInner {
    html_node_base: HTMLNodeBaseInner,
    href: Option<String>,
    general_attr: GeneralAttribute,
}

impl AnchorElementInner {}

impl HTMLNodeInnerT for AnchorElementInner {
    fn as_html_node_inner(&self) -> &HTMLNodeBaseInner {
        &self.html_node_base
    }

    fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner {
        &mut self.html_node_base
    }

    fn render(&self) -> String {
        let mut res = String::from("<a");

        if let Some(href_val) = &self.href {
            res.push_str(&format!(" href=\"{}\"", href_val));
        }

        res.push_str(&self.general_attr.render_general_attr());

        res.push('>');

        for child in (&self.as_html_node_inner().children).into_iter() {
            res.push_str(&child.borrow().render());
        }

        res.push_str("</a>");

        return res;
    }
}

#[derive(Default)]
pub struct AnchorBuilder {
    href: Option<String>,
    content: Option<String>,
    gen_attr_builder: GeneralAttributeBuilder,
}

impl AnchorBuilder {
    pub fn new() -> Self {
        Self {
            href: None,
            content: None,
            gen_attr_builder: GeneralAttributeBuilder::new(),
        }
    }

    pub fn set_content(mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self
    }

    pub fn set_href(mut self, href: &str) -> Self {
        self.href = Some(String::from(href));
        self
    }

    pub fn build(self) -> HTMLNode {
        let (href, content, gen_attr) = (self.href, self.content, self.gen_attr_builder.build());
        let res = HTMLNode::Element(Rc::new(RefCell::new(ElementInner::Anchor(
            AnchorElementInner {
                html_node_base: HTMLNodeBaseInner {
                    parent: None,
                    children: vec![],
                    leaf: false,
                },
                href: href,
                general_attr: gen_attr,
            },
        ))));

        if let Some(content) = &content {
            let text = HTMLNode::create_text(content);
            let _ = res.add_child(&text);
        }

        return res;
    }
}

impl GeneralAttributeBuilderT for AnchorBuilder {
    fn as_general_attribute_builder_mut(&mut self) -> &mut GeneralAttributeBuilder {
        &mut self.gen_attr_builder
    }
}

#[cfg(test)]
mod tests {
    use crate::html::{node::element::GeneralAttributeBuilderT, HTMLNodeT};

    use super::AnchorBuilder;

    #[test]
    fn test_anchor() {
        let anchor_1 = AnchorBuilder::default()
            .set_href("/page1")
            .set_id("id_1")
            .add_class("class1")
            .add_class("class2")
            .build();

        let anchor_2 = AnchorBuilder::default()
            .set_href("#me")
            .set_id("id_2")
            .add_data_attr("role", "button")
            .build();

        assert!(anchor_1.add_child(&anchor_2).is_ok());
        assert_eq!(
            anchor_1.render(),
            String::from(
                r##"<a href="/page1" class="class1 class2 " id="id_1"><a href="#me" id="id_2" data-role="button"></a></a>"##
            )
        );
    }
}

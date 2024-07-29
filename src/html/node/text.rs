use std::{cell::RefCell, rc::Rc};

use crate::html::{html_impl::HTMLNodeInnerTImpl, HTMLNodeBaseInner, HTMLNodeInnerT, HTMLNodeT};

use super::HTMLNode;

pub struct TextInner {
    html_node_base: HTMLNodeBaseInner,
    content: String,
}

impl HTMLNodeT for HTMLNode {}

impl HTMLNodeInnerTImpl for TextInner {
    fn as_html_node_inner(&self) -> &HTMLNodeBaseInner {
        &self.html_node_base
    }

    fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner {
        &mut self.html_node_base
    }

    fn inner_render_impl(&self) -> String {
        self.content.clone()
    }
}

impl HTMLNodeInnerT for TextInner {}

pub(crate) fn create_text_impl(content: &str) -> HTMLNode {
    HTMLNode::Text(Rc::new(RefCell::new(TextInner {
        html_node_base: HTMLNodeBaseInner {
            parent: None,
            children: vec![],
            leaf: true,
        },
        content: String::from(content),
    })))
}

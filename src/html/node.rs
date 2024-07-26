pub mod element;

use element::ElementInner;

use std::{cell::RefCell, rc::Rc};

use super::{HTMLNodeBase, HTMLNodeBaseInner, HTMLNodeInnerT, HTMLNodeT};

pub enum HTMLNode {
    Text(Rc<RefCell<TextInner>>),
    Element(Rc<RefCell<ElementInner>>),
}

struct TextInner {
    html_node_base: HTMLNodeBaseInner,
    content: String,
}

impl HTMLNodeT for HTMLNode {
    fn inner_ptr(&self) -> Rc<RefCell<dyn HTMLNodeInnerT>> {
        match self {
            HTMLNode::Text(tect_inner) => {
                return tect_inner.clone();
            }
            HTMLNode::Element(element_inner) => {
                return element_inner.clone();
            }
            _ => {
                unimplemented!("")
            }
        }
    }
}

impl HTMLNodeInnerT for TextInner {
    fn as_html_node_inner(&self) -> &HTMLNodeBaseInner {
        &self.html_node_base
    }

    fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner {
        &mut self.html_node_base
    }

    fn render(&self) -> String {
        self.content.clone()
    }
}

impl HTMLNode {
    pub fn create_text(content: &str) -> Self {
        Self::Text(Rc::new(RefCell::new(TextInner {
            html_node_base: HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: true,
            },
            content: String::from(content),
        })))
    }
}

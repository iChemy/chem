pub mod element;

pub mod text;

use element::ElementInner;
use text::{create_text_impl, TextInner};

use std::{cell::RefCell, rc::Rc};

use super::{HTMLNodeInnerT, HTMLNodeTImpl};

pub enum HTMLNode {
    Text(Rc<RefCell<TextInner>>),
    Element(Rc<RefCell<ElementInner>>),
}

impl HTMLNodeTImpl for HTMLNode {
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

impl HTMLNode {
    pub fn create_text(content: &str) -> Self {
        create_text_impl(content)
    }
}

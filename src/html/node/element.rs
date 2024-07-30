use anchor::AnchorElementInner;

use crate::html::HTMLNodeInnerT;

pub mod anchor;
pub mod attribute;

pub enum ElementInner {
    Anchor(AnchorElementInner),
}

impl HTMLNodeInnerT for ElementInner {
    fn as_html_node_inner(&self) -> &crate::html::HTMLNodeBaseInner {
        match self {
            Self::Anchor(anchor_element_inner) => anchor_element_inner.as_html_node_inner(),
        }
    }

    fn as_html_node_inner_mut(&mut self) -> &mut crate::html::HTMLNodeBaseInner {
        match self {
            Self::Anchor(anchor_elemnt_inner) => anchor_elemnt_inner.as_html_node_inner_mut(),
        }
    }

    fn inner_render(&self) -> String {
        match self {
            Self::Anchor(anchor_element_inner) => {
                return anchor_element_inner.inner_render();
            }

            _ => {
                unimplemented!("")
            }
        }
    }
}

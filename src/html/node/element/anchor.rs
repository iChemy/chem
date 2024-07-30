use crate::html::{HTMLNodeBaseInner, HTMLNodeInnerT};

use super::attribute::{AttributeBase, AttributeBaseT};

struct AnchorAttribute {
    href: Option<String>,
    attr_base: AttributeBase,
}

impl AttributeBaseT for AnchorAttribute {
    fn attr_base(&self) -> &AttributeBase {
        &self.attr_base
    }

    fn attr_base_mut(&mut self) -> &mut AttributeBase {
        &mut self.attr_base
    }
}

pub struct AnchorElementInner {
    html_node_base: HTMLNodeBaseInner,
    atrr: AnchorAttribute,
}

impl AnchorElementInner {}

impl HTMLNodeInnerT for AnchorElementInner {
    fn as_html_node_inner(&self) -> &HTMLNodeBaseInner {
        &self.html_node_base
    }

    fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner {
        &mut self.html_node_base
    }

    fn inner_render(&self) -> String {
        let mut res = String::from("<a");

        if let Some(href_val) = &self.atrr.href {
            res.push_str(&format!(" href=\"{}\"", href_val));
        }

        res.push_str(&self.atrr.attr_base().render_attr_base());

        res.push('>');

        for child in (&self.as_html_node_inner().children).into_iter() {
            res.push_str(&child.borrow().inner_render());
        }

        res.push_str("</a>");

        return res;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_anchor() {}
}

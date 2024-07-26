use std::collections::HashMap;

use anchor::AnchorElementInner;

use crate::html::HTMLNodeInnerT;

mod anchor;

pub enum ElementInner {
    Anchor(AnchorElementInner),
}

#[derive(Default)]
struct GeneralAttribute {
    class: Vec<String>,
    id: Option<String>,
    data: HashMap<String, String>,
}

#[derive(Default)]
struct GeneralAttributeBuilder {
    gen_attr: GeneralAttribute,
}

impl GeneralAttributeBuilder {
    fn new() -> Self {
        Self {
            gen_attr: GeneralAttribute {
                class: vec![],
                id: None,
                data: HashMap::new(),
            },
        }
    }

    fn build(self) -> GeneralAttribute {
        self.gen_attr
    }

    fn add_class(mut self, class: &str) -> Self {
        self.gen_attr.class.push(String::from(class));

        self
    }

    fn set_id(mut self, id: &str) -> Self {
        self.gen_attr.id = Some(String::from(id));

        self
    }

    fn add_data_attr(mut self, key: &str, val: &str) -> Self {
        self.gen_attr
            .data
            .insert(String::from(key), String::from(val));
        self
    }
}

trait GeneralAttributeBuilderT: Sized {
    fn as_general_attribute_builder_mut(&mut self) -> &mut GeneralAttributeBuilder;
    fn add_class(mut self, class: &str) -> Self {
        self.as_general_attribute_builder_mut()
            .gen_attr
            .class
            .push(String::from(class));
        self
    }

    fn set_id(mut self, id: &str) -> Self {
        self.as_general_attribute_builder_mut().gen_attr.id = Some(String::from(id));
        self
    }

    fn add_data_attr(mut self, key: &str, val: &str) -> Self {
        self.as_general_attribute_builder_mut()
            .gen_attr
            .data
            .insert(String::from(key), String::from(val));
        self
    }
}

trait GeneralAttributeT {
    fn general_attr(&self) -> &GeneralAttribute;
    fn general_attr_mut(&mut self) -> &mut GeneralAttribute;

    fn get_class(&self) -> &Vec<String> {
        &self.general_attr().class
    }

    fn get_id(&self) -> &Option<String> {
        &self.general_attr().id
    }

    fn get_data_map(&self) -> &HashMap<String, String> {
        &self.general_attr().data
    }

    fn get_data(&self, key: &String) -> Option<&String> {
        self.general_attr().data.get(key)
    }
}

impl GeneralAttribute {
    fn render_general_attr(&self) -> String {
        let mut res = String::new();

        if self.class.len() > 0 {
            res.push_str(" class=\"");
            for class_val in (&self.class).into_iter() {
                res.push_str(&format!("{} ", class_val));
            }
            res.push('"');
        }

        if let Some(id_val) = &self.id {
            res.push_str(&format!(" id=\"{}\"", id_val));
        }

        for (k, v) in (&self.data).into_iter() {
            res.push_str(&format!(" data-{}=\"{}\"", k, v));
        }

        return res;
    }
}

impl GeneralAttributeT for ElementInner {
    fn general_attr(&self) -> &GeneralAttribute {
        match self {
            Self::Anchor(anchor_element_inner) => &anchor_element_inner.general_attr,
            _ => {
                unimplemented!("")
            }
        }
    }

    fn general_attr_mut(&mut self) -> &mut GeneralAttribute {
        match self {
            Self::Anchor(anchor_element_inner) => &mut anchor_element_inner.general_attr,
            _ => {
                unimplemented!("")
            }
        }
    }
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

    fn render(&self) -> String {
        match self {
            Self::Anchor(anchor_element_inner) => {
                return anchor_element_inner.render();
            }

            _ => {
                unimplemented!("")
            }
        }
    }
}

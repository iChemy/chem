use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct AttributeBase {
    class: Vec<String>,
    id: Option<String>,
    data: HashMap<String, String>,
}

pub(crate) trait AttributeBaseT {
    fn attr_base(&self) -> &AttributeBase;
    fn attr_base_mut(&mut self) -> &mut AttributeBase;

    fn get_class(&self) -> &Vec<String> {
        &self.attr_base().class
    }

    fn get_id(&self) -> &Option<String> {
        &self.attr_base().id
    }

    fn get_data_map(&self) -> &HashMap<String, String> {
        &self.attr_base().data
    }

    fn get_data(&self, key: &String) -> Option<&String> {
        self.attr_base().data.get(key)
    }
}

impl AttributeBase {
    pub fn render_attr_base(&self) -> String {
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

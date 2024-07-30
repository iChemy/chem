use std::{cell::RefCell, rc::Rc};

use html_private::{HTMLNodeBaseInner, HTMLNodeInnerT, HTMLNodeTImpl};
pub mod node;

mod html_private {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    use super::NodeError;

    pub trait HTMLNodeInnerT {
        fn inner_render(&self) -> String;
        fn as_html_node_inner(&self) -> &HTMLNodeBaseInner;
        fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner;
    }

    pub trait HTMLNodeTImpl {
        fn inner_ptr(&self) -> Rc<RefCell<dyn HTMLNodeInnerT>>;

        fn remove_child_impl(
            me: &Rc<RefCell<dyn HTMLNodeInnerT>>,
            target: &Rc<RefCell<dyn HTMLNodeInnerT>>,
        ) -> Result<(), NodeError> {
            let mut idx = None;
            for (i, my_child) in me.borrow().as_html_node_inner().children.iter().enumerate() {
                if Rc::ptr_eq(target, my_child) {
                    idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = idx {
                me.borrow_mut()
                    .as_html_node_inner_mut()
                    .children
                    .remove(idx);
                Ok(())
            } else {
                Err(NodeError::NotChild)
            }
        }

        fn is_ancestor_of_impl(
            me: &Rc<RefCell<dyn HTMLNodeInnerT>>,
            descendant: &Rc<RefCell<dyn HTMLNodeInnerT>>,
        ) -> Result<bool, NodeError> {
            if Rc::ptr_eq(me, descendant) {
                return Err(NodeError::SameNodeCompare);
            }
            if let Some(descendant_parent_weak) = &descendant.borrow().as_html_node_inner().parent {
                if let Some(descendant_parent_ptr) = descendant_parent_weak.upgrade() {
                    if Rc::ptr_eq(me, &descendant_parent_ptr) {
                        return Ok(true);
                    } else {
                        Self::is_ancestor_of_impl(me, &descendant_parent_ptr)
                    }
                } else {
                    Err(NodeError::GetParentPtr)
                }
            } else {
                return Ok(false);
            }
        }

        fn is_descendant_of_impl(
            me: &Rc<RefCell<dyn HTMLNodeInnerT>>,
            ancestor: &Rc<RefCell<dyn HTMLNodeInnerT>>,
        ) -> Result<bool, NodeError> {
            if Rc::ptr_eq(me, ancestor) {
                return Err(NodeError::SameNodeCompare);
            }
            if let Some(my_parent_weak) = &me.borrow().as_html_node_inner().parent {
                if let Some(my_parent_ptr) = my_parent_weak.upgrade() {
                    if Rc::ptr_eq(&ancestor, &my_parent_ptr) {
                        return Ok(true);
                    } else {
                        Self::is_descendant_of_impl(&my_parent_ptr, &ancestor)
                    }
                } else {
                    Err(NodeError::GetParentPtr)
                }
            } else {
                return Ok(false);
            }
        }

        fn add_child_impl(
            me: Rc<RefCell<dyn HTMLNodeInnerT>>,
            you: Rc<RefCell<dyn HTMLNodeInnerT>>,
        ) -> Result<(), NodeError> {
            if me.borrow().as_html_node_inner().leaf {
                return Err(NodeError::AddToLeaf);
            }

            let is_descendant_res = Self::is_descendant_of_impl(&me, &you);
            if let Err(err) = is_descendant_res {
                return Err(err);
            } else if let Ok(res) = is_descendant_res {
                if res {
                    return Err(NodeError::AddAncestorToDescendant);
                }
            }

            if let Some(your_parent_weak) = &you.borrow().as_html_node_inner().parent {
                if let Some(your_parent_ptr) = &your_parent_weak.upgrade() {
                    if let Err(err) = Self::remove_child_impl(your_parent_ptr, &you) {
                        return Err(err);
                    }
                } else {
                    return Err(NodeError::GetParentPtr);
                }
            }

            you.borrow_mut().as_html_node_inner_mut().parent = Some(Rc::downgrade(&me));
            me.borrow_mut().as_html_node_inner_mut().children.push(you);

            return Ok(());
        }
    }

    pub struct HTMLNodeBaseInner {
        pub parent: Option<Weak<RefCell<dyn HTMLNodeInnerT>>>,
        pub children: Vec<Rc<RefCell<dyn HTMLNodeInnerT>>>,
        pub leaf: bool,
    }

    impl Default for HTMLNodeBaseInner {
        fn default() -> Self {
            HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            }
        }
    }
}
pub trait HTMLNodeT: HTMLNodeTImpl {
    fn is_ancestor_of(&self, descendant: &impl HTMLNodeT) -> Result<bool, NodeError> {
        let me = self.inner_ptr();
        let descendant_ptr = descendant.inner_ptr();
        Self::is_ancestor_of_impl(&me, &descendant_ptr)
    }

    fn is_descendant_of(&self, ancestor: &impl HTMLNodeT) -> Result<bool, NodeError> {
        let me = self.inner_ptr();
        let ancestor_ptr = ancestor.inner_ptr();
        Self::is_descendant_of_impl(&me, &ancestor_ptr)
    }

    fn remove_child(&self, child: &impl HTMLNodeT) -> Result<(), NodeError> {
        let me = self.inner_ptr();
        let child_ptr = child.inner_ptr();
        Self::remove_child_impl(&me, &child_ptr)
    }

    fn add_child(&self, node: &impl HTMLNodeT) -> Result<(), NodeError> {
        let me = self.inner_ptr();
        let you = node.inner_ptr();
        Self::add_child_impl(me, you)
    }

    fn render(&self) -> String {
        self.inner_ptr().borrow().inner_render()
    }
}
struct HTMLNodeBase {
    ptr: Rc<RefCell<HTMLNodeBaseInner>>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum NodeError {
    #[error("failed to get parent pointer")]
    GetParentPtr,
    #[error("node is not child")]
    NotChild,
    #[error("same node comparision")]
    SameNodeCompare,
    #[error("cannot add ancestor to descendant node")]
    AddAncestorToDescendant,
    #[error("cannot add to leaf node")]
    AddToLeaf,
}

impl HTMLNodeTImpl for HTMLNodeBase {
    fn inner_ptr(&self) -> Rc<RefCell<dyn HTMLNodeInnerT>> {
        self.ptr.clone()
    }
}

impl HTMLNodeT for HTMLNodeBase {}

impl HTMLNodeInnerT for HTMLNodeBaseInner {
    fn as_html_node_inner(&self) -> &HTMLNodeBaseInner {
        self
    }
    fn as_html_node_inner_mut(&mut self) -> &mut HTMLNodeBaseInner {
        self
    }
    fn inner_render(&self) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_node() {
        let node_1 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_2 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_3 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_4 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };

        assert!(node_1.add_child(&node_2).is_ok());
        assert!(node_3.add_child(&node_4).is_ok());
        if let Ok(res) = node_3.is_ancestor_of(&node_4) {
            assert!(res)
        } else {
            unreachable!("unexpected result!!");
        }
        assert!(node_1.add_child(&node_3).is_ok());
        if let Ok(res) = node_1.is_ancestor_of(&node_4) {
            assert!(res);
        } else {
            unreachable!("unexpected result!!");
        }
    }

    #[test]
    fn test_add_child_to_descendant() {
        let node_1 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_2 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_3 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };

        let _ = node_1.add_child(&node_2);
        let _ = node_2.add_child(&node_3);
        if let Err(err) = node_3.add_child(&node_1) {
            assert_eq!(err, NodeError::AddAncestorToDescendant)
        } else {
            unreachable!("unexpected result!!")
        }
    }

    #[test]
    fn test_switch_parent() {
        let node_1 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_2 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };
        let node_3 = HTMLNodeBase {
            ptr: Rc::new(RefCell::new(HTMLNodeBaseInner {
                parent: None,
                children: vec![],
                leaf: false,
            })),
        };

        let _ = node_1.add_child(&node_2);
        if let Ok(res) = node_1.is_ancestor_of(&node_2) {
            assert!(res);
        } else {
            unreachable!("unexpected result!!");
        }

        let _ = node_3.add_child(&node_2);
        if let Ok(res) = node_3.is_ancestor_of(&node_2) {
            assert!(res);
        } else {
            unreachable!("unexpected result!!");
        }
        if let Ok(res) = node_1.is_ancestor_of(&node_2) {
            assert!(!res);
        } else {
            unreachable!("unexpected result!!");
        }
    }
}

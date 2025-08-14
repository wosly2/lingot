use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::ui::base::Element;

pub fn get_child<'a>(children: &HashMap<String, Rc<RefCell<Element<'a>>>>, id: &str) -> Option<Rc<RefCell<Element<'a>>>> {
    for child in children {
        if child.0 == id {
            return Some(child.1.clone());
        }
    };
    return None;
}

/// easiest way to get the child of something
pub fn with_element<'a, T, F>(
    children: &HashMap<String, Rc<RefCell<Element<'a>>>>,
    key: &str,
    matcher: for<'b> fn(element: &'b mut Element<'a>) -> Option<&'b mut T>,
    mut f: F
) 
    where F: FnMut(&mut T),
{
    if let Some(e) = children.get(key) {
        let mut borrowed = e.borrow_mut();
        if let Some(target) = matcher(&mut *borrowed) {
            f(target)
        }
    }
}

pub mod matcher {
    use crate::ui::*;

    pub fn panel<'a, 'b>(element: &'b mut Element<'a>) -> Option<&'b mut Panel<'a>> {
        if let Element::Panel(e) = element {
            Some(e)
        } else {
            None
        }
    }

    pub fn label<'a, 'b>(element: &'b mut Element<'a>) -> Option<&'b mut Label<'a>> {
        if let Element::Label(e) = element {
            Some(e)
        } else {
            None
        }
    }

    pub fn text_edit<'a, 'b>(element: &'b mut Element<'a>) -> Option<&'b mut TextEdit<'a>> {
        if let Element::TextEdit(e) = element {
            Some(e)
        } else {
            None
        }
    }
}
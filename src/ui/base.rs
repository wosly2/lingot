use std::{cell::RefCell, rc::Rc};

use raylib::prelude::*;

use crate::draw;
use super::*;


pub enum Element<'a> {
    Panel(Panel<'a>),
    Label(Label<'a>),
    TextEdit(TextEdit<'a>)
}

pub trait ElementTraits<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle);
    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>>;
    fn add_child(&mut self, child: Element<'a>, id: &str);
}

impl<'a> ElementTraits<'a> for Element<'a> {
    fn render(&self, parent_x: i32, parent_y: i32, context: &mut draw::Context, draw_handle: &mut RaylibDrawHandle) {
        match self {
            Element::Panel(e) => e.render(parent_x, parent_y, context, draw_handle),
            Element::Label(e) => e.render(parent_x, parent_y, context, draw_handle),
            Element::TextEdit(e) => e.render(parent_x, parent_y, context, draw_handle),
        }
    }

    fn get_child(&self, id: &str) -> Option<Rc<RefCell<Element<'a>>>>{
        match self {
            Element::Panel(e) => e.get_child(id),
            Element::Label(e) => e.get_child(id),
            Element::TextEdit(e) => e.get_child(id),
        }
    }

    fn add_child(&mut self, child: Element<'a>, id: &str) {
        match self {
            Element::Panel(e) => e.add_child(child, id),
            Element::Label(e) => e.add_child(child, id),
            Element::TextEdit(e) => e.add_child(child, id),
        }
    }
}

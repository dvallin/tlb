use std::collections::{ HashMap };
use std::ops::{ Deref, DerefMut };
use engine::tcod::{ Tcod };
use geometry::{ Rect };

use tcod::colors::{ self };

#[derive(Clone, Debug, PartialEq)]
pub enum UiData {
    Empty,
    Text { text: String },
    MultiLine { text: Vec<String> },
    Border { content: Box<UiElement> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiElement {
    data: UiData,
    rect: Rect,
}

impl UiElement {
    fn render(&self, tcod: &mut Tcod) {
        match self.data {
            UiData::Text { ref text } => {
                tcod.render_text((self.rect.left(), self.rect.top()),
                                 colors::BLACK, colors::LIGHT_GREY, text);
            }
            UiData::MultiLine { ref text } => {
                for i in 0..text.len() {
                    tcod.render_text((self.rect.left(), self.rect.top() + i as i32),
                                     colors::BLACK, colors::LIGHT_GREY,
                                     text.get(i).unwrap());
                }
            }
            UiData::Border { ref content } => {
                tcod.render_box(&self.rect, colors::BLACK, colors::LIGHT_GREY);
                content.deref().render(tcod);
            }
            _ => {}
        }
    }

    fn update(&mut self, inner: UiData) {
        match self.data {
            UiData::Border { ref mut content } => {
                content.deref_mut().update(inner);
            },
            _ => self.data = inner
        }
    }

    fn text(rect: Rect, text: String) -> Self {
        UiElement {
            data: UiData::Text { text: text },
            rect: rect,
        }
    }

    fn empty(rect: Rect) -> Self {
        UiElement {
            data: UiData::Empty,
            rect: rect,
        }
    }

    fn border(content: UiElement) -> Self {
        let outer_rect = content.rect.grow(1);
        UiElement {
            data: UiData::Border { content: Box::new(content) },
            rect: outer_rect,
        }
    }
}

pub struct Ui {
    elements: HashMap<String, UiElement>,
}

impl Ui {
    pub fn new() -> Self {
        Ui {
            elements: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, rect: Rect) {
        self.elements.insert(name, UiElement::border(UiElement::empty(rect)));
    }

    pub fn update(&mut self, name: String, element: UiData) {
        self.elements.get_mut(&name).map(|e| e.update(element));
    }

    pub fn draw(&self, tcod: &mut Tcod) {
        for element in self.elements.values() {
            element.render(tcod);
        }
    }
}

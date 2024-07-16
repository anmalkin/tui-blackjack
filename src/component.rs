use ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
    Frame,
};

pub struct Component<'a> {
    widget: Paragraph<'a>,
    block: Block<'a>,
    rect: Rect,
}

impl<'a> Component<'a> {
    pub fn new() -> Self {
        Component::default()
    }

    pub fn widget(self, widget: Paragraph<'a>) -> Self {
        Self { widget, block: self.block, rect: self.rect }
    }

    pub fn block(self, block: Block<'a>) -> Self {
        Self { widget: self.widget, block, rect: self.rect }
    }

    pub fn rect(self, rect: Rect) -> Self {
        Self { widget: self.widget, block: self.block, rect }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(&self.widget, self.rect);
    }
}

impl<'a> Default for Component<'a> {
    fn default() -> Self {
        let widget = Paragraph::default();
        let block = Block::default();
        let rect = Rect::default();
        Self { widget, block, rect }
    }
}

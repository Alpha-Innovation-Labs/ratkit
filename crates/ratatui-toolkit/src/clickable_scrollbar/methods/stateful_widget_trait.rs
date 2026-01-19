use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

pub trait ClickableScrollbarStatefulWidgetExt {
    type State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State);
}

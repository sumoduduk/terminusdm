use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, LineGauge, Widget};

pub struct Volume(f32);

impl Volume {
    pub fn new(volume: f32) -> Self {
        Self(volume)
    }
}

impl Widget for &Volume {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vol = (self.0 / 2.0) as f64;

        let block_volume = Block::bordered()
            .fg(Color::Red)
            .title("Volume")
            .border_type(BorderType::Rounded);

        LineGauge::default()
            .filled_style(Style::new().fg(Color::White).bg(Color::LightRed))
            .line_set(symbols::line::NORMAL)
            .ratio(vol)
            .block(block_volume)
            .render(area, buf);
    }
}

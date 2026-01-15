use crate::app::state::MessageState;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct MessageScreen<'a> {
    state: &'a MessageState,
}

impl<'a> MessageScreen<'a> {
    pub fn new(state: &'a MessageState) -> Self {
        MessageScreen { state }
    }
}

impl<'a> Widget for MessageScreen<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Créer un dialog centré
        let dialog_width = 50.min(area.width.saturating_sub(4));
        let dialog_height = 8.min(area.height.saturating_sub(4));

        let dialog_area = centered_rect(dialog_width, dialog_height, area);

        // Remplir l'arrière-plan avec un effet semi-transparent (grisé)
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if x < buf.area.width && y < buf.area.height {
                    buf.cell_mut((x, y)).map(|cell| cell.set_bg(Color::Black));
                }
            }
        }

        // Déterminer le style selon le type de message
        let (icon, title_color, border_color) = if self.state.is_error {
            ("⚠️  ", Color::Red, Color::Red)
        } else {
            ("✅ ", Color::Green, Color::Green)
        };

        let title = format!("{}{}", icon, self.state.title);

        // Créer le block du dialog
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title.as_str())
            .title_style(
                Style::default()
                    .fg(title_color)
                    .add_modifier(Modifier::BOLD),
            )
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        // Layout interne: message + aide
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(2)])
            .split(inner);

        // Message
        let message = Paragraph::new(self.state.message.as_str())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));

        message.render(chunks[0], buf);

        // Aide
        let help = Paragraph::new(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" ou "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Fermer"),
        ]))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

        help.render(chunks[1], buf);
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height.saturating_sub(height)) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width.saturating_sub(width)) / 2),
        ])
        .split(popup_layout[1])[1]
}

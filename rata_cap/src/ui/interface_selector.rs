use crate::network::NetworkInterface;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Widget},
};

pub struct InterfaceSelector {
    interfaces: Vec<NetworkInterface>,
    state: ListState,
}

impl InterfaceSelector {
    pub fn new(interfaces: Vec<NetworkInterface>) -> Self {
        let mut state = ListState::default();
        if !interfaces.is_empty() {
            state.select(Some(0));
        }
        Self { interfaces, state }
    }

    pub fn next(&mut self) {
        if self.interfaces.is_empty() {
            return;
        }
        
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.interfaces.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.interfaces.is_empty() {
            return;
        }
        
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.interfaces.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&NetworkInterface> {
        self.state.selected().and_then(|i| self.interfaces.get(i))
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new("Network Interface Selection")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        title.render(chunks[0], buf);

        let items: Vec<ListItem> = self
            .interfaces
            .iter()
            .map(|iface| {
                let mut lines = vec![Line::from(vec![
                    Span::styled(
                        &iface.name,
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ),
                ])];
                
                if let Some(desc) = &iface.description {
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(desc, Style::default().fg(Color::Gray)),
                    ]));
                }
                
                if !iface.addresses.is_empty() {
                    let addr_str = iface.addresses.join(", ");
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("IP: {}", addr_str),
                            Style::default().fg(Color::Green),
                        ),
                    ]));
                }
                
                if iface.is_loopback {
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled("[Loopback]", Style::default().fg(Color::Magenta)),
                    ]));
                }
                
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Available Interfaces"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        ratatui::widgets::StatefulWidget::render(list, chunks[1], buf, &mut self.state);

        let help = Paragraph::new("↑/↓: Navigate | Enter: Select | q: Quit")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        help.render(chunks[2], buf);
    }
}
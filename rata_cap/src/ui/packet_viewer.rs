use crate::network::Packet;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState, Widget},
};

pub struct PacketViewer {
    packets: Vec<Packet>,
    state: TableState,
    is_paused: bool,
    interface_name: String,
}

impl PacketViewer {
    pub fn new(interface_name: String) -> Self {
        Self {
            packets: Vec::new(),
            state: TableState::default(),
            is_paused: false,
            interface_name,
        }
    }

    pub fn add_packet(&mut self, packet: Packet) {
        if !self.is_paused {
            self.packets.push(packet);
            if self.packets.len() > 1000 {
                self.packets.remove(0);
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn scroll_up(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected > 0 {
            self.state.select(Some(selected - 1));
        }
    }

    pub fn scroll_down(&mut self) {
        let selected = self.state.selected().unwrap_or(0);
        if selected < self.packets.len().saturating_sub(1) {
            self.state.select(Some(selected + 1));
        }
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

        let status = if self.is_paused {
            format!("Capturing on {} [PAUSED]", self.interface_name)
        } else {
            format!("Capturing on {} [RUNNING]", self.interface_name)
        };
        
        let title = Paragraph::new(status)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        title.render(chunks[0], buf);

        let header = Row::new(vec![
            Cell::from("Time"),
            Cell::from("Source"),
            Cell::from("Destination"),
            Cell::from("Protocol"),
            Cell::from("Length"),
            Cell::from("Info"),
        ])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .height(1);

        let rows: Vec<Row> = self
            .packets
            .iter()
            .map(|packet| {
                Row::new(vec![
                    Cell::from(packet.time_str()),
                    Cell::from(packet.source.clone()),
                    Cell::from(packet.destination.clone()),
                    Cell::from(packet.protocol.to_string()),
                    Cell::from(packet.length.to_string()),
                    Cell::from(packet.info.clone()),
                ])
                .height(1)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Min(20),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Packets ({})", self.packets.len())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

        ratatui::widgets::StatefulWidget::render(table, chunks[1], buf, &mut self.state);

        let help = Paragraph::new("↑/↓: Scroll | Space: Pause/Resume | q: Back | Esc: Exit")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        help.render(chunks[2], buf);
    }
}
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::network::{capture::PacketCapture, NetworkInterface, Packet};
use crate::ui::{InterfaceSelector, PacketViewer};

#[derive(Debug)]
pub enum AppState {
    InterfaceSelection,
    Capturing(String),
    Exiting,
}

pub struct App {
    state: AppState,
    interface_selector: Option<InterfaceSelector>,
    packet_viewer: Option<PacketViewer>,
    packet_rx: Option<mpsc::Receiver<Packet>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let interfaces = NetworkInterface::list_interfaces()
            .map_err(|e| anyhow::anyhow!("Failed to list network interfaces: {}. Make sure you have proper permissions.", e))?;
        
        if interfaces.is_empty() {
            return Err(anyhow::anyhow!("No network interfaces found. Please check your network configuration."));
        }
        
        Ok(Self {
            state: AppState::InterfaceSelection,
            interface_selector: Some(InterfaceSelector::new(interfaces)),
            packet_viewer: None,
            packet_rx: None,
        })
    }

    pub fn is_running(&self) -> bool {
        !matches!(self.state, AppState::Exiting)
    }

    pub async fn update(&mut self) -> Result<()> {
        if let Some(rx) = &mut self.packet_rx {
            while let Ok(packet) = rx.try_recv() {
                if let Some(viewer) = &mut self.packet_viewer {
                    viewer.add_packet(packet);
                }
            }
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match &self.state {
                    AppState::InterfaceSelection => {
                        match key.code {
                            KeyCode::Up => {
                                if let Some(selector) = &mut self.interface_selector {
                                    selector.previous();
                                }
                            }
                            KeyCode::Down => {
                                if let Some(selector) = &mut self.interface_selector {
                                    selector.next();
                                }
                            }
                            KeyCode::Enter => {
                                if let Some(selector) = &self.interface_selector {
                                    if let Some(interface) = selector.selected() {
                                        self.start_capture(interface.name.clone()).await?;
                                    }
                                }
                            }
                            KeyCode::Char('q') | KeyCode::Esc => {
                                self.state = AppState::Exiting;
                            }
                            _ => {}
                        }
                    }
                    AppState::Capturing(_) => {
                        match key.code {
                            KeyCode::Up => {
                                if let Some(viewer) = &mut self.packet_viewer {
                                    viewer.scroll_up();
                                }
                            }
                            KeyCode::Down => {
                                if let Some(viewer) = &mut self.packet_viewer {
                                    viewer.scroll_down();
                                }
                            }
                            KeyCode::Char(' ') => {
                                if let Some(viewer) = &mut self.packet_viewer {
                                    viewer.toggle_pause();
                                }
                            }
                            KeyCode::Char('q') => {
                                self.state = AppState::InterfaceSelection;
                                self.packet_viewer = None;
                                self.packet_rx = None;
                            }
                            KeyCode::Esc => {
                                self.state = AppState::Exiting;
                            }
                            _ => {}
                        }
                    }
                    AppState::Exiting => {}
                }
            }
        }

        Ok(())
    }

    async fn start_capture(&mut self, interface_name: String) -> Result<()> {
        let (tx, rx) = mpsc::channel(1000);
        self.packet_rx = Some(rx);
        self.packet_viewer = Some(PacketViewer::new(interface_name.clone()));
        self.state = AppState::Capturing(interface_name.clone());

        let capture = PacketCapture::new(interface_name);
        tokio::spawn(async move {
            if let Err(e) = capture.start_capture(tx).await {
                eprintln!("Capture error: {}", e);
            }
        });

        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        match &self.state {
            AppState::InterfaceSelection => {
                if let Some(selector) = &mut self.interface_selector {
                    selector.render(frame.size(), frame.buffer_mut());
                }
            }
            AppState::Capturing(_) => {
                if let Some(viewer) = &mut self.packet_viewer {
                    viewer.render(frame.size(), frame.buffer_mut());
                }
            }
            AppState::Exiting => {}
        }
    }
}
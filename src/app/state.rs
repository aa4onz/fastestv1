// src/app/state.rs
use crate::models::{ActivePanel, DiscordMessage, Server};
use ratatui::widgets::ListState;
use std::time::Instant;

pub struct AppState {
    pub token: String,
    pub target_channel_id: String,
    pub messages: Vec<DiscordMessage>,
    pub input_text: String,
    pub list_state: ListState,
    pub servers_state: ListState,
    pub channels_state: ListState,
    pub servers: Vec<Server>,
    pub active_panel: ActivePanel,
    pub failed_nonces: Vec<String>,
    pub last_typing_sent: Option<Instant>,
}

impl AppState {
    pub fn new(token: String) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut servers_state = ListState::default();
        servers_state.select(Some(0));

        let mut channels_state = ListState::default();
        channels_state.select(Some(0));

        Self {
            token,
            target_channel_id: String::new(),
            messages: Vec::new(),
            input_text: String::new(),
            list_state,
            servers_state,
            channels_state,
            servers: Vec::new(),
            active_panel: ActivePanel::ChatInput,
            failed_nonces: Vec::new(),
            last_typing_sent: None,
        }
    }
}

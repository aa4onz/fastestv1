// src/app/handlers.rs
use crate::models::{AppEvent, DiscordMessage, MessageStatus};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use tokio::sync::mpsc::Sender;

impl crate::app::state::AppState {
    pub async fn handle_event(
        &mut self,
        event: AppEvent,
        tx: &Sender<AppEvent>,
    ) -> bool {
        match event {
            AppEvent::IncomingMessage(m) => {
                if m.nonce.starts_with("err-") {
                    self.messages.push(m);
                } else if let Some(existing) = self.messages.iter_mut().find(|x| x.nonce == m.nonce && !m.nonce.is_empty()) {
                    existing.status = MessageStatus::Delivered;
                    existing.timestamp = m.timestamp;
                    existing.content = m.content;
                } else {
                    self.messages.push(m);
                }
            }
            AppEvent::MessageSent { nonce, timestamp } => {
                if let Some(m) = self.messages.iter_mut().find(|x| x.nonce == nonce) {
                    m.status = MessageStatus::Delivered;
                    if !timestamp.is_empty() {
                        m.timestamp = timestamp;
                    }
                }
            }
            AppEvent::MessageFailed { nonce } => {
                if let Some(m) = self.messages.iter_mut().find(|x| x.nonce == nonce) {
                    m.status = MessageStatus::Failed;
                }
            }
            AppEvent::Terminal(Event::Key(key)) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    return true;
                }
                match key.code {
                    KeyCode::Enter => {
                        let text = self.input_text.trim().to_string();
                        if !text.is_empty() {
                            let nonce = format!("n-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
                            self.messages.push(DiscordMessage {
                                nonce: nonce.clone(),
                                author: "You".to_string(),
                                content: text.clone(),
                                timestamp: String::new(),
                                status: MessageStatus::Sending,
                            });
                            self.input_text.clear();
                            let _ = tx.send(AppEvent::HttpSendChat { nonce, text }).await;
                        }
                    }
                    KeyCode::Char(c) => {
                        self.input_text.push(c);
                        let now = std::time::Instant::now();
                        let should_send_typing = match self.last_typing_sent {
                            Some(last) => now.duration_since(last).as_secs() >= 8,
                            None => true,
                        };
                        if should_send_typing {
                            self.last_typing_sent = Some(now);
                            let _ = tx.send(AppEvent::HttpTriggerTyping).await;
                        }
                    }
                    KeyCode::Backspace => {
                        self.input_text.pop();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }
}

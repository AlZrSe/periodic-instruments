//! Plugin UI module — periodic-table view, preset browser, macro knobs
//!
//! This module contains the iced-based GUI implementation via nice-plug.
//! Currently provides a minimal editor registration.

use iced::{Element, Length, Sandbox, Settings, Theme};
use nice_plug::iced::IcedEditor;

/// Main editor state for the plugin UI
#[derive(Default)]
pub struct EditorState {
    // Will hold UI state: current element, parameter values, etc.
}

/// Messages the UI can send
#[derive(Debug, Clone)]
pub enum EditorMessage {
    // Parameter changes, element selection, etc.
}

/// Minimal editor view — empty for now, will be expanded in later tasks
impl Sandbox for EditorState {
    type Message = EditorMessage;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Periodic Instruments".to_string()
    }

    fn update(&mut self, _message: EditorMessage) {
        // Handle UI messages
    }

    fn view(&self) -> Element<EditorMessage> {
        use iced::widget::{container, text, Column};
        
        container(
            Column::new()
                .push(text("Periodic Instruments").size(32))
                .push(text("Version 0.1.0").size(16))
                .push(text("Foundation Build — UI coming soon").size(14))
                .spacing(16),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

/// Editor type alias for nice-plug integration
pub type PluginEditor = IcedEditor<EditorState, EditorMessage>;

/// Default editor dimensions
pub const DEFAULT_WIDTH: u32 = 1200;
pub const DEFAULT_HEIGHT: u32 = 800;
pub const MIN_WIDTH: u32 = 800;
pub const MIN_HEIGHT: u32 = 600;
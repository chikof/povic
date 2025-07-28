use color_eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;
use crate::config::AppConfig;

pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: AppConfig,
}

impl Home {
    pub fn new(config: AppConfig) -> Self {
        Self { command_tx: None, config }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: AppConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            },
            Action::Render => {
                // add any logic here that should run on every render
            },
            _ => {},
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        frame.render_widget(Paragraph::new("hello world"), area);
        Ok(())
    }
}

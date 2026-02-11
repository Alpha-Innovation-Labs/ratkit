use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{widgets::Block, Frame};
use ratkit::{run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig};
use ratkit::widgets::ai_chat::{AIChat, AIChatEvent, Message};

struct AiChatDemo {
    chat: AIChat,
}

impl AiChatDemo {
    fn new() -> Self {
        let mut chat = AIChat::new();
        chat.messages_mut()
            .add(Message::assistant("Hello! Ask me anything.".to_string()));
        Self { chat }
    }
}

impl CoordinatorApp for AiChatDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        match event {
            CoordinatorEvent::Keyboard(keyboard) => {
                if keyboard.key_code == KeyCode::Char('q')
                    && keyboard.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(CoordinatorAction::Quit);
                }

                match self.chat.handle_key(keyboard.key_code) {
                    AIChatEvent::MessageSubmitted(text) => {
                        self.chat.set_loading(false);
                        self.chat
                            .messages_mut()
                            .add(Message::assistant(format!("Echo: {}", text)));
                    }
                    AIChatEvent::Command(command) => {
                        self.chat
                            .messages_mut()
                            .add(Message::assistant(format!("Command: {}", command)));
                        self.chat.set_loading(false);
                    }
                    _ => {}
                }

                Ok(CoordinatorAction::Redraw)
            }
            _ => Ok(CoordinatorAction::Continue),
        }
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let block = Block::default().title(" Ctrl+Q to quit ");
        let inner = block.inner(area);
        frame.render_widget(block, area);
        self.chat.render(frame, inner);
    }
}

fn main() -> std::io::Result<()> {
    let app = AiChatDemo::new();
    run_with_diagnostics(app, RunnerConfig::default())
}

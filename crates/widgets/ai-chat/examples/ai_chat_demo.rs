use std::io;

use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{widgets::Block, Frame};
use ratkit_ai_chat::{AIChat, AIChatEvent, Message};
use ratkit_example_runner::{run, App, RunConfig, RunnerAction, RunnerEvent};

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

impl App for AiChatDemo {
    fn on_event(&mut self, event: RunnerEvent) -> io::Result<RunnerAction> {
        match event {
            RunnerEvent::Crossterm(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(RunnerAction::Quit);
                }

                match self.chat.handle_key(key.code) {
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

                Ok(RunnerAction::Redraw)
            }
            _ => Ok(RunnerAction::Redraw),
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

fn main() -> io::Result<()> {
    let mut app = AiChatDemo::new();
    run(&mut app, RunConfig::default())
}

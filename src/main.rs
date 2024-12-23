use iced::{Element, Length, Sandbox, Settings, Theme};
use iced::widget::{container, row, text, text_editor, column, horizontal_space};
fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("This is a text editor!")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input_content = text_editor(&self.content).on_action(Message::Edit).height(Length::Fill);
        let position = {
            let (row, column) = &self.content.cursor_position();
            text(format!("{} : {}", row + 1, column + 1))
        };

        let status_bar = row!(horizontal_space(), position);
        container(column!(input_content, status_bar)).padding(20).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
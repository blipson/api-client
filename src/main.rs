use iced::Element;
use iced::widget::{Column, button, column, row, text_input};

impl ApiClient {
    fn view(&self) -> Column<Message> {
        let dropdown: Option<Element<Message>> = self.show_dropdown.then(|| {
            column![
                button("Request").on_press(Message::CreateRequest),
                button("Folder").on_press(Message::CreateFolder),
            ]
            .spacing(5)
            .padding(5)
            .into()
        });

        let request_box: Option<Element<Message>> = self.show_request_box.then(|| {
            text_input("Enter request...", &self.text_value)
                .on_input(Message::Type)
                .on_submit(Message::SubmitRequest)
                .padding(5)
                .into()
        });

        column![row![button("Create").on_press(Message::ToggleDropdown)].push_maybe(request_box)]
            .push_maybe(dropdown)
    }
}

#[derive(Default)]
struct ApiClient {
    show_dropdown: bool,
    show_request_box: bool,
    text_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    Type(String),
    ToggleDropdown,
    CreateRequest,
    CreateFolder,
    SubmitRequest,
}

impl ApiClient {
    fn update(&mut self, message: Message) {
        match message {
            Message::Type(value) => {
                self.text_value = value
            }
            Message::ToggleDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            Message::CreateRequest => {
                println!("Create Request triggered");
                self.show_dropdown = false;
                self.show_request_box = true;
            }
            Message::CreateFolder => {
                println!("Create Folder triggered");
                self.show_dropdown = false;
            }
            Message::SubmitRequest => {
                println!("{}", "submitting...")
            }
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("API Client", ApiClient::update, ApiClient::view)
}

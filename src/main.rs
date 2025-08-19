use iced::widget::{button, column, row, scrollable, text_input, Column, Container, Space, Text};
use iced::{Element, Font, Length};
use serde_json::Value;

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
            text_input("Enter request...", &self.request_text)
                .on_input(Message::Type)
                .on_submit(Message::RunRequest)
                .padding(5)
                .width(Length::FillPortion(1))
                .into()
        });

        let run_request: Option<Element<Message>> = self
            .show_request_box
            .then(|| button("Run").on_press(Message::RunRequest).into());

        let request_space: Option<Space> = (self.show_request_box && !self.show_response)
            .then(|| Space::with_width(Length::FillPortion(1)));

        let response: Option<Element<Message>> = self.show_response.then(|| {
            Container::new(
                scrollable(Text::new(&self.response).font(Font::MONOSPACE).size(14))
                    .width(Length::Fill),
            )
            .padding(10)
            .width(Length::Fill)
            .into()
        });

        column![
            row![button("Create").on_press(Message::ToggleDropdown)]
                .push_maybe(request_box)
                .push_maybe(run_request)
                .push_maybe(request_space)
                .push_maybe(response)
        ]
        .push_maybe(dropdown)
    }
}

#[derive(Default)]
struct ApiClient {
    show_dropdown: bool,
    show_request_box: bool,
    request_text: String,
    show_response: bool,
    response: String,
}

#[derive(Debug, Clone)]
enum Message {
    Type(String),
    ToggleDropdown,
    CreateRequest,
    CreateFolder,
    RunRequest,
}

impl ApiClient {
    fn update(&mut self, message: Message) {
        match message {
            Message::Type(value) => self.request_text = value,
            Message::ToggleDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            Message::CreateRequest => {
                self.show_dropdown = false;
                self.show_request_box = true;
                self.show_response = false;
                self.request_text = "".to_string();
            }
            Message::CreateFolder => {
                self.show_dropdown = false;
            }
            Message::RunRequest => match reqwest::blocking::get(&self.request_text) {
                Ok(response) => {
                    self.show_response = true;
                    let json: Value = response.json().expect("");
                    self.response = json.to_string();
                }
                Err(error) => {
                    self.show_response = true;
                    self.response = "Error".to_string();
                }
            },
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("API Client", ApiClient::update, ApiClient::view)
}

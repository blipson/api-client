use iced::widget::{Column, button, column, row, text_input, Space, Container, Text, scrollable};
use iced::{Element, Font, Length, Theme};
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
            text_input("Enter request...", &self.text_value)
                .on_input(Message::Type)
                .on_submit(Message::SubmitRequest)
                .padding(5)
                .width(Length::FillPortion(1))
                .into()
        });

        let run_request: Option<Element<Message>> = self.show_request_box.then(|| {
            button("Run")
                .on_press(Message::RunRequest)
                .into()
        });

        let request_space: Option<Space> = (self.show_request_box && !self.show_response).then(|| {
            Space::with_width(Length::FillPortion(1))
        });

        let response: Option<Element<Message>> = self.show_response.then(|| {
            Container::new(
                scrollable(
                    Text::new(&self.response)
                        .font(Font::MONOSPACE)
                        .size(14)
                )
                    .height(Length::Fixed(240.0))
                    .width(Length::Fill)
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
    text_value: String,
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
    SubmitRequest,
}

impl ApiClient {
    fn update(&mut self, message: Message) {
        match message {
            Message::Type(value) => self.text_value = value,
            Message::ToggleDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            Message::CreateRequest => {
                self.show_dropdown = false;
                self.show_request_box = true;
            }
            Message::CreateFolder => {
                self.show_dropdown = false;
            }
            Message::RunRequest => match reqwest::blocking::get(&self.text_value) {
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
            Message::SubmitRequest => {
                println!("{}", "submitting...")
            }
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("API Client", ApiClient::update, ApiClient::view)
}

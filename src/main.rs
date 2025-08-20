mod components;
mod models;

use components::side_nav;

use crate::models::folder::Folder;
use crate::Event::CreateFolder;
use iced::widget::{container, row, Container, Row, Space};
use iced::Length;

impl ApiClient {
    fn view(&self) -> Row<Event> {
        row![
            side_nav::view(&self.folders),
            Container::new(Space::with_width(Length::Fixed(0.0)))
                .width(Length::Fixed(1.0))
                .height(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color(iced::Color::WHITE)),
                    ..Default::default()
                }),
            Container::new(Space::with_width(Length::Fixed(0.0)))
                .width(Length::FillPortion(3))
                .height(Length::Fill)
        ]
    }
}

#[derive(Default)]
struct ApiClient {
    folders: Vec<Folder>
}

#[derive(Debug, Clone)]
enum Event {
    CreateFolder,
}

impl ApiClient {
    fn update(&mut self, event: Event) {
        match event {
            CreateFolder => {
                self.folders.push(Folder {
                    name: "New Folder".to_string(),
                    requests: Vec::new()
                })
            }
        }
    }
}

pub fn main() -> iced::Result {
    iced::run("API Client", ApiClient::update, ApiClient::view)
}

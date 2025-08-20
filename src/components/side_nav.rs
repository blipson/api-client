use crate::Event;
use crate::Event::CreateFolder;
use crate::models::folder::Folder;
use iced::widget::{Container, Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub fn view(folders: &[Folder]) -> Element<Event> {
    Container::new(
        column![
            row![text("API Client").size(40)],
            row![
                text("Folders"),
                Space::with_width(Length::Fill),
                button(text("+")).on_press(CreateFolder),
            ],
            scrollable(
                column(folders.iter().map(|folder| {
                    row![text(folder.name.clone()).size(16)]
                        .spacing(8)
                        .align_y(Alignment::Center)
                        .into()
                }))
                .spacing(6),
            )
            .height(Length::Fill),
        ]
        .padding(20)
        .spacing(15)
        .align_x(Alignment::Start),
    )
    .height(Length::Fill)
    .width(Length::FillPortion(1))
    .style(|_| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.2, 0.2, 0.2,
        ))),
        ..Default::default()
    })
    .into()
}

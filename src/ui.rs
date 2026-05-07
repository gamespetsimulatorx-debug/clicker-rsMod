use iced::widget::{
    button, checkbox, container, pick_list, row, text, text_input, column, space
};
use iced::{Alignment, Color, Element, Length, Border, Shadow, Vector, Theme};
use crate::constants::*;
use crate::icons::{icon_text, Icon, ICON_SIZE_BUTTON, ICON_SIZE_SMALL};
use crate::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    Success,
    Error,
    Warning,
    Info,
}

impl StatusType {
    pub fn icon(self) -> Icon {
        match self {
            StatusType::Success => Icon::Success,
            StatusType::Error => Icon::Error,
            StatusType::Warning => Icon::Warning,
            StatusType::Info => Icon::Info,
        }
    }

    pub fn color(self) -> Color {
        match self {
            StatusType::Success => Color::from_rgb(COLOR_SUCCESS[0], COLOR_SUCCESS[1], COLOR_SUCCESS[2]),
            StatusType::Error => Color::from_rgb(COLOR_ERROR[0], COLOR_ERROR[1], COLOR_ERROR[2]),
            StatusType::Warning => Color::from_rgb(COLOR_WARNING[0], COLOR_WARNING[1], COLOR_WARNING[2]),
            StatusType::Info => Color::from_rgb(COLOR_INFO[0], COLOR_INFO[1], COLOR_INFO[2]),
        }
    }
}

/// Wraps content in a styled card with background and soft borders
pub fn card_container<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .padding(UI_SPACING_MEDIUM)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(Color::from_rgb(COLOR_SURFACE[0], COLOR_SURFACE[1], COLOR_SURFACE[2]).into()),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                    width: 1.0,
                    radius: 8.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 4.0,
                },
                ..Default::default()
            }
        })
        .into()
}

/// Creates a premium button with icon, label, and hover transitions
pub fn icon_button(
    icon: Icon,
    label: &str,
    message: Option<Message>,
) -> button::Button<'_, Message> {
    let content = row![
        icon_text(icon, ICON_SIZE_BUTTON),
        text(label).size(14).font(iced::Font::default())
    ]
    .spacing(UI_SPACING_SMALL)
    .align_y(Alignment::Center);

    button(content)
        .padding([8, 16])
        .style(move |theme: &Theme, status| {
            let palette = theme.extended_palette();
            let base_color = Color::from_rgb(COLOR_ACCENT[0], COLOR_ACCENT[1], COLOR_ACCENT[2]);
            
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(base_color.into()),
                    text_color: Color::WHITE,
                    border: Border { radius: 6.0.into(), ..Default::default() },
                    ..Default::default()
                },
                button::Status::Pressed => button::Style {
                    background: Some(base_color.into()),
                    text_color: Color::WHITE,
                    border: Border { radius: 6.0.into(), ..Default::default() },
                    shadow: Shadow::default(),
                    ..Default::default()
                },
                button::Status::Disabled => button::Style {
                    background: Some(palette.secondary.weak.color.into()),
                    text_color: palette.secondary.weak.text,
                    border: Border { radius: 6.0.into(), ..Default::default() },
                    ..Default::default()
                },
                _ => button::Style {
                    background: Some(Color::from_rgba(COLOR_ACCENT[0], COLOR_ACCENT[1], COLOR_ACCENT[2], 0.1).into()),
                    text_color: base_color,
                    border: Border { 
                        radius: 6.0.into(), 
                        color: base_color,
                        width: 1.0 
                    },
                    ..Default::default()
                },
            }
        })
        .on_press_maybe(message)
}

/// A standard input row with integrated validation and label hierarchy
pub fn input_row<'a>(
    label: &'a str,
    placeholder: &'a str,
    value: &'a str,
    on_change: fn(String) -> Message,
    validation_status: Option<(bool, &'a str, &'a str)>,
) -> Element<'a, Message> {
    let mut main_row = row![
        text(label)
            .width(Length::Fixed(UI_LABEL_WIDTH))
            .size(14)
            .color(Color::from_rgb(0.7, 0.7, 0.7)),
        text_input(placeholder, value)
            .on_input(on_change)
            .width(Length::Fixed(UI_INPUT_WIDTH))
            .padding(10)
    ]
    .spacing(UI_SPACING_SMALL)
    .align_y(Alignment::Center);

    if let Some((is_valid, valid_t, invalid_t)) = validation_status {
        main_row = main_row.push(validation_indicator(is_valid, valid_t, invalid_t));
    }

    main_row.into()
}

pub fn validation_indicator<'a>(
    is_valid: bool,
    valid_text: &'a str,
    invalid_text: &'a str,
) -> Element<'a, Message> {
    let color = if is_valid { 
        Color::from_rgb(COLOR_SUCCESS[0], COLOR_SUCCESS[1], COLOR_SUCCESS[2]) 
    } else { 
        Color::from_rgb(COLOR_ERROR[0], COLOR_ERROR[1], COLOR_ERROR[2]) 
    };

    row![
        icon_text(
            if is_valid { Icon::Valid } else { Icon::Invalid },
            ICON_SIZE_SMALL
        ).color(color),
        text(if is_valid { valid_text } else { invalid_text })
            .size(UI_VALIDATION_SIZE)
            .color(color)
    ]
    .spacing(UI_SPACING_TINY)
    .width(Length::Fixed(UI_VALIDATION_WIDTH))
    .align_y(Alignment::Center)
    .into()
}

pub fn status_message(status_type: StatusType, message: &str) -> Element<'_, Message> {
    let color = status_type.color();
    container(
        row![
            icon_text(status_type.icon(), ICON_SIZE_SMALL).color(color),
            text(message).size(UI_VALIDATION_SIZE).color(color).bold()
        ]
        .spacing(UI_SPACING_TINY)
        .align_y(Alignment::Center)
    )
    .padding([4, 8])
    .style(move |_| container::Style {
        background: Some(Color { a: 0.05, ..color }.into()),
        border: Border { radius: 4.0.into(), ..Default::default() },
        ..Default::default()
    })
    .into()
}

pub fn section_header(icon: Icon, title: &str) -> Element<'_, Message> {
    column![
        row![
            icon_text(icon, ICON_SIZE_BUTTON).color(Color::from_rgb(COLOR_ACCENT[0], COLOR_ACCENT[1], COLOR_ACCENT[2])),
            text(title).size(UI_SUBTITLE_SIZE).bold()
        ]
        .spacing(UI_SPACING_SMALL)
        .align_y(Alignment::Center),
        space::vertical(Length::Fixed(4.0)),
        container(space::horizontal(Length::Fill))
            .height(1)
            .style(|_| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.1).into()),
                ..Default::default()
            })
    ]
    .spacing(UI_SPACING_TINY)
    .into()
}

pub fn styled_checkbox(
    label: &str,
    is_checked: bool,
    on_toggle: fn(bool) -> Message,
) -> Element<'_, Message> {
    checkbox(label, is_checked)
        .on_toggle(on_toggle)
        .size(18)
        .spacing(UI_SPACING_SMALL)
        .into()
}

pub fn clickable_text(text_content: &str, size: u16, message: Message) -> Element<'_, Message> {
    button(text(text_content).size(size))
        .style(|theme: &Theme, status| {
            let palette = theme.extended_palette();
            button::Style {
                background: None,
                text_color: match status {
                    button::Status::Hovered => palette.primary.strong.color,
                    _ => palette.primary.base.color,
                },
                border: Border::default(),
                shadow: Shadow::default(),
            }
        })
        .padding(0)
        .on_press(message)
        .into()
}
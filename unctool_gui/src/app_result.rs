use iced::Alignment;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Padding;
use iced::Subscription;
use iced::Task;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::clipboard;
use iced::time;
use iced::widget;
use iced::widget::Button;
use iced::widget::{Column, Row};
use iced::widget::{column, row};
use std::time::{Duration, Instant};
use unctool;

#[derive(Debug)]
pub struct InitContext {
    pub is_success: bool,
    pub text_value: String,
    pub scale_factor: f32,
}

pub fn run(init_context: InitContext) -> iced::Result {
    iced::application(move || App::new(&init_context), App::update, App::view)
        .subscription(App::subscription)
        .scale_factor(App::scale_factor)
        .window_size((300, 120))
        .title(App::title)
        .centered()
        .run()
}

#[derive(Debug, PartialEq)]
enum CopyButtonState {
    Enabled,
    Disabled(Instant),
}

#[derive(Debug)]
struct FontSize {
    regular: u32,
    large: u32,
}

impl FontSize {
    fn new(regular: u32, large: u32) -> Self {
        FontSize {
            regular: regular,
            large: large,
        }
    }
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize {
            regular: 14,
            large: 18,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    PathChanged(String),
    OnSubmit,
    OnCopy,
    OnTick,
}

#[derive(Debug)]
struct App {
    is_success: bool,
    text_value: String,
    copy_button_state: CopyButtonState,
    font_size: FontSize,
    scale_factor: f32,
}

impl App {
    fn title(&self) -> String {
        String::from("UNC Tool")
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn new(init_context: &InitContext) -> (Self, Task<Message>) {
        (
            App {
                is_success: init_context.is_success,
                text_value: init_context.text_value.clone(),
                scale_factor: init_context.scale_factor,
                copy_button_state: CopyButtonState::Enabled,
                font_size: FontSize::default(),
            },
            Task::default(),
        )
    }

    fn text_success_or_failure(&self) -> Element<'_, Message> {
        let content = match self.is_success {
            true => "Success",
            false => "Failure",
        };

        widget::text(content)
            .size(self.font_size.large)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .into()
    }

    fn text_input_result_or_error(&self) -> Element<'_, Message> {
        let placeholder = match self.is_success {
            true => "Success",
            false => "Failure",
        };

        widget::text_input(placeholder, &self.text_value)
            .size(self.font_size.regular)
            .width(Length::Fill)
            .font(Font::MONOSPACE)
            .on_input(Message::PathChanged)
            .align_x(Horizontal::Left)
            .into()
    }

    fn button<'a>(&self, s: &'a str) -> Button<'a, Message> {
        widget::button(
            widget::text(s)
                .align_x(Alignment::Center)
                .size(self.font_size.regular),
        )
    }

    fn button_copy(&self) -> Element<'_, Message> {
        let btn = match self.copy_button_state {
            CopyButtonState::Enabled => self.button("Copy").on_press(Message::OnCopy),
            CopyButtonState::Disabled(_) => self.button("Done"),
        };

        btn.style(widget::button::primary)
            .width(Length::Shrink)
            .padding(5)
            .into()
    }

    fn button_ok(&self) -> Element<'_, Message> {
        self.button("Ok")
            .width(Length::Fill)
            .on_press(Message::OnSubmit)
            .style(widget::button::primary)
            .into()
    }

    fn row_header(&self) -> Element<'_, Message> {
        row![self.text_success_or_failure()]
            .align_y(Alignment::Center)
            .into()
    }

    fn row_body(&self) -> Row<'_, Message> {
        row![self.text_input_result_or_error(), self.button_copy()]
            .align_y(Vertical::Center)
            .spacing(2)
            .into()
    }

    fn row_footer(&self) -> Row<'_, Message> {
        row![self.button_ok()].align_y(Vertical::Center).into()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            self.row_header(),
            self.row_body(),
            widget::space().height(5),
            self.row_footer()
        ]
        .align_x(Alignment::Center)
        .padding(10)
        .spacing(0)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnCopy => {
                if let Some(until) = Instant::now().checked_add(Duration::from_secs(1)) {
                    self.copy_button_state = CopyButtonState::Disabled(until);
                }
                clipboard::write(self.text_value.clone()).into()
            }
            Message::OnTick => {
                match self.copy_button_state {
                    CopyButtonState::Enabled => {}
                    CopyButtonState::Disabled(until) => {
                        if until <= Instant::now() {
                            self.copy_button_state = CopyButtonState::Enabled
                        }
                    }
                };
                Task::none()
            }
            Message::PathChanged(val) => {
                self.text_value = val;
                Task::none()
            }

            Message::OnSubmit => std::process::exit(0),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.copy_button_state {
            CopyButtonState::Enabled => Subscription::none(),
            CopyButtonState::Disabled(_) => {
                time::every(Duration::from_millis(500)).map(|_| Message::OnTick)
            }
        }
    }
}
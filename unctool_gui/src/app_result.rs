use iced::Alignment::Center;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Length::Shrink;
use iced::Padding;
use iced::Subscription;
use iced::Task;
use iced::alignment::Horizontal::Left;
use iced::alignment::Vertical::Top;
use iced::clipboard;
use iced::time;
use iced::widget::Text;
use iced::widget::button;
use iced::widget::row;
use iced::widget::space::horizontal;
use iced::widget::value;
use iced::widget::{column, pick_list, scrollable, space, text, text_input};
use iced::window;
use iced::window::Settings;
use std::time::{Duration, Instant};
use unctool;

pub fn run(result: unctool::Result<String>) -> iced::Result {
    iced::application(move || App::new(&result), App::update, App::view)
        .subscription(App::subscription)
        .title(App::title)
        .window_size((300, 120))
        .scale_factor(|_| 1.0)
        .centered()
        .run()
}

#[derive(Debug, PartialEq)]
enum CopyButtonState {
    Enabled,
    TempDisabled(Instant),
}

struct App {
    is_success: bool,
    text_value: String,
    copy_btn: CopyButtonState,
}

#[derive(Debug, Clone)]
enum Message {
    PathChanged(String),
    OnSubmit,
    OnCopy,
    OnTick,
}

impl App {
    fn title(&self) -> String {
        String::from("UNC Tool")
    }

    fn new(result: &unctool::Result<String>) -> (Self, Task<Message>) {
        let (is_success, text_value) = match result {
            unctool::Result::Ok(value) => (true, value.clone()),
            unctool::Result::Err(err) => (false, err.to_string()),
        };

        (
            App {
                is_success: is_success,
                text_value: text_value,
                copy_btn: CopyButtonState::Enabled,
            },
            Task::default(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        let size = 14;
        let size_big = size + 4;

        let success = match self.is_success {
            true => text("Success")
                .size(size_big)
                .height(Length::Fill)
                .align_x(Center),
            false => text("Failed")
                .size(size_big)
                .height(Length::Fill)
                .align_x(Center),
        };

        let text2 = text_input("Result", &self.text_value)
            .size(size)
            .width(Length::Fill)
            .font(Font::MONOSPACE)
            .on_input(Message::PathChanged)
            .align_x(Left);

        let btn_copy = match self.copy_btn {
            CopyButtonState::Enabled => button(text("Copy").size(size))
                .on_press(Message::OnCopy)
                .width(Length::Shrink)
                .padding(5)
                .style(button::primary),
            CopyButtonState::TempDisabled(_) => button(text("Done").size(size))
                .width(Length::Shrink)
                .padding(5)
                .style(button::primary),
        };

        let row3 = row![success].align_y(Center);

        let row4 = row![text2, btn_copy].align_y(Center).spacing(2);

        let btn_ok = button(text("Ok").align_x(Center).size(size))
            .width(Length::Fill)
            .on_press(Message::OnSubmit)
            .style(button::primary);

        let row5 = row![btn_ok].align_y(Center);

        let col = column![row3, row4, space().height(5), row5]
            .align_x(Center)
            .padding(10)
            .spacing(0);

        col.into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnCopy => {
                let x = Instant::now().checked_add(Duration::from_secs(2)).unwrap();
                self.copy_btn = CopyButtonState::TempDisabled(x);
                clipboard::write(self.text_value.clone()).into()
            }

            Message::OnTick => {
                let x = match self.copy_btn {
                    CopyButtonState::TempDisabled(ins) => {
                        if ins <= Instant::now() {
                            false
                        } else {
                            true
                        }
                    }
                    CopyButtonState::Enabled => true,
                };
                if !x {
                    self.copy_btn = CopyButtonState::Enabled;
                }
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
        match self.copy_btn {
            CopyButtonState::Enabled => Subscription::none(),
            CopyButtonState::TempDisabled(_) => {
                time::every(Duration::from_millis(500)).map(|_| Message::OnTick)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::{Error, simulator};

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = Counter { value: 0 };
        let mut ui = simulator(counter.view());

        let _ = ui.click("Increment")?;
        let _ = ui.click("Increment")?;
        let _ = ui.click("Decrement")?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 1);

        let mut ui = simulator(counter.view());
        assert!(ui.find("1").is_ok(), "Counter should display 1!");

        Ok(())
    }
}

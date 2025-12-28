use iced::Alignment::Center;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Length::Shrink;
use iced::Padding;
use iced::Task;
use iced::alignment::Horizontal::Left;
use iced::alignment::Vertical::Top;
use iced::widget::Text;
use iced::widget::button;
use iced::widget::row;
use iced::widget::space::horizontal;
use iced::widget::{column, pick_list, scrollable, space, text, text_input};
use iced::window;
use iced::window::Settings;

pub fn show() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .window_size((300, 120))
		.scale_factor(|_| 1.0)
        .centered()
        .run()
}

#[derive(Default)]
struct App {
    path: String,
}

#[derive(Debug, Clone)]
enum Message {
    OnSubmit,
}

impl App {
    const ICON_FONT: &'static [u8] = include_bytes!("../icons.ttf");

    fn title(&self) -> String {
        String::from("UNCTool")
    }

    fn new() -> (Self, Task<Message>) {
        (
            App {
                path: String::new(),
            },
            Task::default(),
        )
    }

    fn view(&self) -> Element<'_, Message> {

		let size = 14;
		let size_big = size + 4;

        let success = text(
			"Success!"
		)
        .size(size_big)
		// .width(Length::Fill)
		.height(Length::Fill)
		.align_x(Center);

        let text2 = text_input(
            "some example path aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "some example path aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        // let text2 = text_input(
        //     "some example path ", "aaa"
        // )
        .size(size)
        .width(Length::Fill)
		.font(Font::MONOSPACE)
		.align_x(Left);

        let btn_copy = button(text("Copy").size(size))
            .width(Length::Shrink)
            .on_press(Message::OnSubmit)
            .padding(5)
            .style(button::primary);

        let row3 = row![success]
			.align_y(Center);

        let row4 = row![text2, btn_copy]
			.align_y(Center)
            .spacing(2);

        let btn_ok = button(text("Ok").align_x(Center).size(size))
            .width(Length::Fill)
            .on_press(Message::OnSubmit)
            .style(button::primary);

        let row5 = row![btn_ok]
			.align_y(Center);

        let col = column![row3, row4, space().height(5), row5]
			.align_x(Center)
			.padding(10)
            .spacing(0);

        col.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OnSubmit => {
                println!("On submit")
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

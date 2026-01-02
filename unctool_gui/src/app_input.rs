use iced::Alignment::Center;
use iced::Element;
use iced::Length::Fill;
use iced::Length::Shrink;
use iced::Task;
use iced::widget::button;
use iced::widget::row;
use iced::widget::{column, pick_list, scrollable, space, text_input};
use iced::window;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title(App::title)
        .window_size((400.0, 200.0))
        .run()
}

#[derive(Default)]
struct App {
    target_os: Option<TargetOS>,
    command: Option<CommandName>,
    path: String,
}

// #[derive(Debug, Default)]
// struct AppState {
//     input_value: String,
// }

#[derive(Debug, Clone)]
enum Message {
    TargetOsChanged(TargetOS),
    CommandChanged(CommandName),
    PathChanged(String),
    OnSubmit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetOS {
    #[default]
    Windows,
    Linux,
}

impl std::fmt::Display for TargetOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TargetOS::Windows => "Windows",
                TargetOS::Linux => "Linux",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandName {
    #[default]
    Convert,
    LocalPath,
    RemotePath,
}

impl std::fmt::Display for CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CommandName::Convert => "Linux UNC <-> Windows UNC",
                CommandName::LocalPath => "Windows/Linux UNC -> Local path",
                CommandName::RemotePath => "Local path -> Windows/Linux UNC",
            }
        )
    }
}

impl App {
    fn title(&self) -> String {
        String::from("UNCTool")
    }

    fn new() -> (Self, Task<Message>) {
        (
            App {
                target_os: Some(TargetOS::Windows),
                command: Some(CommandName::Convert),
                path: String::new(),
            },
            Task::default(),
        )
    }

    fn view(&self) -> Element<'_, Message> {
        let commands = [
            CommandName::Convert,
            CommandName::LocalPath,
            CommandName::LocalPath,
        ];

        let pick_list_commands = pick_list(commands, self.command, Message::CommandChanged)
            .placeholder("Choose command");

        let targets = [TargetOS::Windows, TargetOS::Linux];

        let pick_list_targets = pick_list(targets, self.target_os, Message::TargetOsChanged)
            .placeholder("Choose target OS");

        // let text = text_input("aaa", "bbb").on_input(Message::PathChanged);
        // .placeholder("Enter path");

        let input = text_input("Enter UNC or filesystem path", &self.path)
            .on_input(Message::PathChanged)
            .on_submit(Message::OnSubmit)
            .padding(15)
            .size(30)
            .align_x(Center);

        let btn_sumbit = button("Submit")
            .on_press(Message::OnSubmit)
            .padding(10)
            .style(button::primary);

        let btn_cancel = button("Cancel")
            .on_press(Message::OnSubmit)
            .padding(10)
            .style(button::secondary);

        let row1 = row!["Command", pick_list_commands]
            .width(Fill)
            .height(Fill)
            .align_y(Center)
            .spacing(10);

        let row2 = row!["Target OS", pick_list_targets]
            .width(Fill)
            .height(Fill)
            .align_y(Center)
            .spacing(10);

        let row3 = row![input]
            .width(Fill)
            .height(Fill)
            .align_y(Center)
            .spacing(10);

        let row4 = row![btn_sumbit, btn_cancel]
            .width(Fill)
            .height(Fill)
            .align_y(Center)
            .spacing(10);

        // let row2 = row!["Target OS", pick_list_targets]
        //     .width(Fill)
        //     .height(Fill)
        //     .align_y(Center)
        //     .spacing(10);

        let content = column![row1, row2, row3, row4].spacing(10);

        content.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TargetOsChanged(val) => {
                self.target_os = Some(val);
            }
            Message::CommandChanged(val) => {
                self.command = Some(val);
            }
            Message::PathChanged(val) => self.path = val,
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

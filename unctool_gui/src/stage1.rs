use iced::Alignment;
use iced::Alignment::Center;
use iced::Element;
use iced::Font;
use iced::Length::Fill;
use iced::Length::FillPortion;
use iced::Length::Shrink;
use iced::Task;
use iced::alignment::Horizontal;
use iced::widget::button;
use iced::widget::container;
use iced::widget::row;
use iced::widget::{column, pick_list, scrollable, space, text_input, text};
use iced::window;

#[derive(Debug, Clone)]
pub struct UISettings {
    pub scale_factor: f32,
}

#[derive(Debug, Clone)]
pub struct InitContext {
    pub ui_settings: UISettings,
}

pub fn run(init_context: InitContext) -> iced::Result {
    iced::application(move || App::new(init_context.clone()), App::update, App::view)
        .scale_factor(App::scale_factor)
        .window_size((400, 200))
        .title(App::title)
        .resizable(false)
        .centered()
        .run()
}

#[derive(Default)]
struct App {
    target_os: Option<TargetOS>,
    command: Option<CommandName>,
    path: String,
    scale_factor: f32,
}

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
                CommandName::Convert => r"Linux UNC ↔ Windows UNC",
                CommandName::LocalPath => r"Windows/Linux UNC → Local path",
                CommandName::RemotePath => r"Local path → Windows/Linux UNC",
            }
        )
    }
}

impl App {
    fn title(&self) -> String {
        format!("UNC Tool {}", env!("CARGO_PKG_VERSION"))
    }

    fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    fn new(init_context: InitContext) -> (Self, Task<Message>) {
        (
            App {
                scale_factor: init_context.ui_settings.scale_factor,
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
            .width(Fill)
            .align_x(Horizontal::Left);


        let btn_sumbit = button("Submit")
            .on_press(Message::OnSubmit)
            .padding(10)
            .style(button::primary);

        let btn_cancel = button("Cancel")
            .on_press(Message::OnSubmit)
            .padding(10)
            .style(button::secondary);

        let row1 = row![text("Command").width(Fill),  pick_list_commands.width(Shrink)]
        	.width(Fill)
            .align_y(Center)
            .spacing(10);

        let row2 = row![text("Target OS").width(Fill), pick_list_targets.width(Shrink)]
        	.width(Fill)
            .align_y(Center)
            .spacing(10);

        let row3 = row!["Path", input]
        	.width(Fill)
            .align_y(Center)
            .spacing(10);

        let row4 = row![space::horizontal(), btn_sumbit, btn_cancel]
            .align_y(Center)
            .padding(5)
            .spacing(10);

        let content = column![row1, row2, space::vertical(), row3, space::vertical(), row4].spacing(5).padding(5);

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
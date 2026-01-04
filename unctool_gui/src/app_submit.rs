use iced::Element;
use iced::Length;
use iced::Task;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget;
use iced::widget::Text;
use iced::widget::{column, row};
use std::env;
use std::process::Command;

#[derive(Debug)]
struct FontSize {
    regular: u32,
}

impl Default for FontSize {
    fn default() -> Self {
        FontSize { regular: 14 }
    }
}

#[derive(Debug, Clone)]
pub struct UISettings {
    pub scale_factor: f32,
}

#[derive(Debug, Clone)]
pub struct InitContext {
    pub ui_settings: UISettings,
}

pub fn run(ctx: InitContext) -> iced::Result {
    iced::application(move || App::new(ctx.clone()), App::update, App::view)
        .scale_factor(App::scale_factor)
        .window_size((400, 170))
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
    font_size: FontSize,
}

#[derive(Debug, Clone)]
enum Message {
    TargetOsChanged(TargetOS),
    CommandChanged(CommandName),
    PathChanged(String),
    OnSubmit,
    OnCancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum TargetOS {
    #[default]
    Windows,
    Linux,
}

impl std::fmt::Display for TargetOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TargetOS::Windows => "Windows",
            TargetOS::Linux => "Linux",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub enum CommandName {
    #[default]
    Convert,
    LocalPath,
    RemotePath,
}

impl std::fmt::Display for CommandName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CommandName::Convert => r"Linux UNC ↔ Windows UNC",
            CommandName::LocalPath => r"Windows/Linux UNC → Local path",
            CommandName::RemotePath => r"Local path → Windows/Linux UNC",
        };
        write!(f, "{}", s)
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
                font_size: FontSize::default(),
                path: String::new(),
            },
            Task::default(),
        )
    }

    fn text<'a>(&self, s: &'a str) -> Text<'a> {
        widget::text(s).size(self.font_size.regular)
    }

    fn button_submit(&self) -> Element<'_, Message> {
        let btn = widget::button(self.text("Submit"));
        let btn = match !self.path.is_empty() {
            true => btn.on_press(Message::OnSubmit),
            false => btn,
        };
        btn.width(Length::Shrink)
            .style(widget::button::primary)
            .padding(10)
            .into()
    }

    fn button_cancel(&self) -> Element<'_, Message> {
        widget::button(self.text("Cancel"))
            .style(widget::button::secondary)
            .on_press(Message::OnCancel)
            .width(Length::Shrink)
            .padding(10)
            .into()
    }

    fn pick_list_commands(&self) -> Element<'_, Message> {
        let commands = [
            CommandName::Convert,
            CommandName::LocalPath,
            CommandName::RemotePath,
        ];

        widget::pick_list(commands, self.command, Message::CommandChanged)
            .text_size(self.font_size.regular)
            .placeholder("Choose command")
            .width(Length::Shrink)
            .into()
    }

    fn pick_list_targets(&self) -> Element<'_, Message> {
        let mut targets = vec![TargetOS::Windows, TargetOS::Linux];
        if self.target_os == None {
            targets.clear();
        }

        widget::pick_list(targets, self.target_os, Message::TargetOsChanged)
            .text_size(self.font_size.regular)
            .placeholder("Choose target OS")
            .width(Length::Shrink)
            .into()
    }

    fn text_input_path(&self) -> Element<'_, Message> {
        widget::text_input("Enter UNC or filesystem path", &self.path)
            .size(self.font_size.regular)
            .on_input(Message::PathChanged)
            .on_submit(Message::OnSubmit)
            .align_x(Horizontal::Left)
            .width(Length::Fill)
            .into()
    }

    fn row_command(&self) -> Element<'_, Message> {
        row![
            self.text("Command").width(Length::Fill),
            self.pick_list_commands()
        ]
        .align_y(Vertical::Center)
        .width(Length::Fill)
        .spacing(10)
        .into()
    }

    fn row_target_os(&self) -> Element<'_, Message> {
        row![
            self.text("Target OS").width(Length::Fill),
            self.pick_list_targets()
        ]
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .spacing(10)
        .into()
    }

    fn row_input_path(&self) -> Element<'_, Message> {
        row![
            self.text("Path").width(Length::Shrink),
            self.text_input_path()
        ]
        .width(Length::Fill)
        .align_y(Vertical::Center)
        .spacing(10)
        .into()
    }

    fn row_button_bar(&self) -> Element<'_, Message> {
        row![
            widget::space::horizontal(),
            self.button_submit(),
            self.button_cancel()
        ]
        .align_y(Vertical::Center)
        .spacing(10)
        .into()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            self.row_command(),
            self.row_target_os(),
            widget::space::vertical(),
            self.row_input_path(),
            widget::space::vertical(),
            self.row_button_bar()
        ]
        .spacing(5)
        .padding(10)
        .into()
    }

    fn on_submit(&self) {
        let command = match self.command.unwrap() {
            CommandName::Convert => "convert",
            CommandName::LocalPath => "local-path",
            CommandName::RemotePath => "remote-path",
        };

        let fn_target_os = || match self.target_os.unwrap() {
            TargetOS::Windows => "windows",
            TargetOS::Linux => "linux",
        };

        let args: Vec<String> = env::args().collect();
        let mut binding = Command::new(args[0].clone());
        let cmd = binding.arg("--ui-scale").arg(self.scale_factor.to_string());
        let cmd = cmd.arg(command).arg(self.path.clone());

        let final_cmd = match self.command.unwrap() {
            CommandName::LocalPath => cmd,
            _ => cmd.arg("-t").arg(fn_target_os()),
        };

        final_cmd.spawn().ok();
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PathChanged(val) => {
                self.path = val;
                Task::none()
            }
            Message::TargetOsChanged(val) => {
                self.target_os = Some(val);
                Task::none()
            }
            Message::CommandChanged(val) => {
                self.command = Some(val);
                self.target_os = match val {
                    CommandName::LocalPath => None,
                    _ => Some(TargetOS::Windows),
                };
                Task::none()
            }
            Message::OnSubmit => {
                self.on_submit();
                Task::none()
            }
            Message::OnCancel => iced::exit(),
        }
    }
}

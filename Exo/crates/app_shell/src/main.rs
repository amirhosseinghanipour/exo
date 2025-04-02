use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use iced::advanced::graphics::core::event::Status;
use iced::advanced::subscription::Recipe;
use iced::futures::stream::BoxStream;
use iced::widget::{button, container, row, scrollable, text, text_input};
use iced::Event;
use iced::Subscription;
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

use futures::{stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use browser_core::{BrowserController, ContentState};
use shared_types::Result as ExoResult;

mod app_logic {
    use super::*;
    use futures::FutureExt;

    #[derive(Debug, Clone)]
    pub enum Message {
        UrlInputChanged(String),
        LoadButtonPressed,
        ContentUpdate(ContentState),
    }

    pub struct ExoApp {
        url_input: String,
        current_state: ContentState,
        core_controller: BrowserController,
        update_receiver_handle: Arc<Mutex<Option<mpsc::Receiver<ContentState>>>>,
    }

    struct CoreUpdatesListener {
        id: &'static str,
        receiver_handle: Arc<Mutex<Option<mpsc::Receiver<ContentState>>>>,
    }

    impl Recipe for CoreUpdatesListener {
        type Output = Message;

        fn hash(&self, state: &mut iced::advanced::Hasher) {
            std::any::TypeId::of::<Self>().hash(state);
            self.id.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: Pin<Box<dyn stream::Stream<Item = (Event, Status)> + Send>>,
        ) -> BoxStream<'static, Self::Output> {
            let receiver_handle = self.receiver_handle.clone();

            Box::pin(async move {
                let receiver_option = receiver_handle.lock().unwrap().take();

                if let Some(receiver) = receiver_option {
                    log::info!("Subscription stream started via Recipe.");
                    ReceiverStream::new(receiver)
                        .map(Message::ContentUpdate)
                        .boxed()
                } else {
                    log::warn!("Subscription Recipe attempted but receiver was already taken.");
                    stream::empty()
                        .map(|never: ContentState| Message::ContentUpdate(never))
                        .boxed()
                }
            })
            .flatten_stream()
            .boxed()
        }
    }

    impl Application for ExoApp {
        type Executor = executor::Default;
        type Message = Message;
        type Theme = Theme;
        type Flags = ();

        fn new(_flags: ()) -> (Self, Command<Message>) {
            let (sender, receiver) = mpsc::channel(100);
            let app = ExoApp {
                url_input: "https://example.com".to_string(),
                current_state: ContentState::Idle,
                core_controller: BrowserController::new(sender),
                update_receiver_handle: Arc::new(Mutex::new(Some(receiver))),
            };
            (app, Command::none())
        }

        fn title(&self) -> String {
            String::from("Exo Browser - Alpha")
        }

        fn update(&mut self, message: Message) -> Command<Message> {
            match message {
                Message::UrlInputChanged(value) => {
                    self.url_input = value;
                    Command::none()
                }
                Message::LoadButtonPressed => {
                    self.current_state = ContentState::Loading(
                        shared_types::Url::parse(&self.url_input)
                            .unwrap_or_else(|_| shared_types::Url::parse("exo://loading").unwrap()),
                    );
                    self.core_controller.load_url(self.url_input.clone());
                    Command::none()
                }
                Message::ContentUpdate(new_state) => {
                    log::info!("UI received state update: {:?}", new_state);
                    self.current_state = new_state;
                    Command::none()
                }
            }
        }

        fn view(&self) -> Element<Message> {
            let address_bar = text_input("Enter URL...", &self.url_input)
                .on_input(Message::UrlInputChanged)
                .on_submit(Message::LoadButtonPressed)
                .padding(10);

            let load_button = button("Go")
                .on_press(Message::LoadButtonPressed)
                .padding(10);

            let header = row![address_bar, load_button]
                .spacing(10)
                .align_items(iced::Alignment::Center);

            let content_view_text = match &self.current_state {
                ContentState::Idle => "Enter a URL and click 'Go'.".to_string(),
                ContentState::Loading(url) => {
                    format!("Loading {}...", url)
                }
                ContentState::Loaded(_url, output) => output.text_content.clone(),
                ContentState::Error(_url, error) => {
                    format!("Error: {:?}", error)
                }
            };

            let content_display = text(content_view_text);

            let scrollable_content =
                scrollable(container(content_display).width(Length::Fill).padding(20))
                    .height(Length::Fill);

            iced::widget::column![header, scrollable_content]
                .spacing(10)
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }

        fn subscription(&self) -> iced::Subscription<Message> {
            let recipe = CoreUpdatesListener {
                id: "core_updates_listener",
                receiver_handle: self.update_receiver_handle.clone(),
            };
            Subscription::from_recipe(recipe)
        }

        fn theme(&self) -> Theme {
            Theme::Dark
        }
    }
}

#[tokio::main]
async fn main() -> ExoResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    log::info!("Starting Exo...");

    app_logic::ExoApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1024.0, 768.0),
            ..Default::default()
        },
        ..Settings::default()
    })
    .map_err(|e| shared_types::ExoError::Core(format!("Iced application error: {}", e)))?;

    Ok(())
}

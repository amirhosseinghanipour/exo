use shared_types::{ExoError, Url};
use log::{info, error};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum ContentState {
    Idle,
    Loading(Url),
    Loaded(Url, rendering_engine::RenderOutput),
    Error(Url, ExoError),
}

pub struct BrowserController {
    update_sender: mpsc::Sender<ContentState>,
    current_state: ContentState,
}

impl BrowserController {
    pub fn new(update_sender: mpsc::Sender<ContentState>) -> Self {
        Self {
            update_sender,
            current_state: ContentState::Idle,
        }
    }

    pub fn load_url(&mut self, url_string: String) {
        info!("Core received request to load: {}", url_string);
        match Url::parse(&url_string) {
            Ok(url) => {
                 if url.scheme() != "http" && url.scheme() != "https" {
                    let err = ExoError::UrlParse("Only http/https URLs are supported".to_string());
                    self.update_state(ContentState::Error(url, err));
                    return;
                }

                let state_url = url.clone();
                self.update_state(ContentState::Loading(state_url.clone()));

                let sender = self.update_sender.clone();
                let task_url = url.clone();

                tokio::spawn(async move {
                    let result = networking::fetch_url(&task_url).await;
                    let final_state = match result {
                        Ok(html_content) => {
                            let render_output = rendering_engine::render_content(&html_content);
                            ContentState::Loaded(task_url, render_output)
                        }
                        Err(err) => {
                            error!("Failed to load URL {}: {:?}", task_url, err);
                            let _error_output = rendering_engine::render_error(&err);
                            ContentState::Error(task_url, err)
                        }
                    };
                    if sender.send(final_state).await.is_err() {
                        error!("Failed to send update back to UI thread (receiver dropped?)");
                    }
                });
            }
            Err(e) => {
                 let invalid_url_str = url_string.clone();
                 let dummy_url = Url::parse("exo://error").unwrap();
                error!("Invalid URL format [{}]: {}", invalid_url_str, e);
                let err = ExoError::UrlParse(format!("Invalid URL [{}]: {}", invalid_url_str, e));
                self.update_state(ContentState::Error(dummy_url, err));
            }
        }
    }

    fn update_state(&mut self, new_state: ContentState) {
        self.current_state = new_state.clone();
        let sender = self.update_sender.clone();
         tokio::spawn(async move {
             let _ = sender.send(new_state).await;
         });
    }

}

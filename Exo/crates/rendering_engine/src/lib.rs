use log::debug;
use shared_types;

#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub text_content: String,
}

pub fn render_content(html_content: &str) -> RenderOutput {
    debug!("'Rendering' content ({} bytes)", html_content.len());
    RenderOutput {
        text_content: html_content.to_string(),
    }
}

pub fn render_error(error: &shared_types::ExoError) -> RenderOutput {
    debug!("'Rendering' error: {:?}", error);
    RenderOutput {
        text_content: format!("Error loading page:\n\n{:?}", error),
    }
}

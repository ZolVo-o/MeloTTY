mod dashboard;
mod progress;
mod search;
mod theme;

use ratatui::Frame;
use std::time::Duration;

use crate::app::{Mode, Page};
use crate::browser::FileBrowser;
use crate::playlist::Playlist;

pub use theme::Theme;
pub use theme::CATTPUCCIN;

pub fn render(
    f: &mut Frame,
    mode: &Mode,
    page: Page,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    volume: f32,
    track_position: Duration,
    track_duration: Option<Duration>,
    spectrum: &[f32],
    search_query: &str,
    cover_ascii: &Option<Vec<(String, Vec<ratatui::style::Color>)>>,
) {
    dashboard::render(
        f,
        mode,
        page,
        browser,
        playlist,
        is_playing,
        has_sink,
        volume,
        track_position,
        track_duration,
        spectrum,
        search_query,
        cover_ascii,
    );
}

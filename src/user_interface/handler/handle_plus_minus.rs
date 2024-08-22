use crate::{app::App, playback::PlaybackEvent};

pub fn handle_plus(app: &mut App) {
    app.increase_volume();

    let sender = app.tx_playback.clone();
    let _ = sender.send(PlaybackEvent::SetVolume(app.volume));
}

pub fn handle_minus(app: &mut App) {
    app.decrease_volume();

    let sender = app.tx_playback.clone();
    let _ = sender.send(PlaybackEvent::SetVolume(app.volume));
}

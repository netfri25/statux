use mpris::{Metadata, PlaybackStatus, PlayerFinder};

use std::fmt::Write;

use super::Component;

pub struct Playing;

impl Component for Playing {
    async fn update(&mut self, buf: &mut String) {
        const ERROR: &str = "playing write error";

        buf.clear();
        let Some((metadata, status)) = get_metadata() else {
            write!(buf, "[]").expect(ERROR);
            return;
        };

        write!(buf, "[").expect(ERROR);
        if status != PlaybackStatus::Playing {
            write!(buf, "^c#666666^").expect(ERROR);
        }

        let artists = metadata.artists().map(|artists| artists.join(", ")).unwrap_or_default();
        let title = metadata.title().unwrap_or_default();
        write!(buf, "{} - {}", artists, title).expect(ERROR);
        write!(buf, "^d^]").expect(ERROR);
    }
}

fn get_metadata() -> Option<(Metadata, PlaybackStatus)> {
    let players = PlayerFinder::new().ok()?.find_all().ok()?;
    let player = players.into_iter().next()?;
    let playback_status = player.get_playback_status().ok()?;
    let metadata = player.get_metadata().ok()?;
    Some((metadata, playback_status))
}

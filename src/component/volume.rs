use alsa::mixer::{Mixer, SelemChannelId, SelemId};

use std::fmt::Write;


use super::{Component, EMPTY_OUTPUT};

pub struct Volume;

impl Component for Volume {
    async fn update(&mut self, buf: &mut String) {
        buf.clear();
        if let Some(volume) = get_volume() {
            write!(buf, "{:2}%", volume).expect("volume write error");
        } else {
            write!(buf, "{}", EMPTY_OUTPUT).expect("volume write error");
        }
    }
}

fn get_volume() -> Option<i64> {
    let mixer = Mixer::new("default", false).ok()?;
    let selem = mixer.find_selem(&SelemId::new("Master", 0))?;
    let (min, max) = selem.get_playback_volume_range();
    // front left is common. most systems only expose front-left + front-right
    let channel = SelemChannelId::FrontLeft;
    let vol = selem.get_playback_volume(channel).ok()?;
    let muted = selem.get_playback_switch(channel).ok()? == 0;
    if muted {
        return None
    }
    let range = max - min;
    let percent = ((vol - min) * 100 + range / 2) / range;
    Some(percent)
}

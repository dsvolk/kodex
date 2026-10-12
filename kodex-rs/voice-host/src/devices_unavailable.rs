//! Keep non-native helper targets buildable without linking a host audio backend.

use std::io;

pub(super) fn list_devices(
    _: kodex_realtime_webrtc::AudioDeviceKind,
) -> io::Result<Vec<kodex_realtime_webrtc::AudioDevice>> {
    Err(io::Error::other(
        "audio devices unavailable for this helper target",
    ))
}

pub(super) struct Devices;

impl Devices {
    pub(super) fn receive(&self, _: crate::incoming::ReceivedRtp) -> io::Result<()> {
        Self::open(Default::default()).map(|_| ())
    }

    pub(super) fn take_state(&self) -> io::Result<kodex_realtime_webrtc::AudioState> {
        Self::open(Default::default()).map(|_| kodex_realtime_webrtc::AudioState::default())
    }

    pub(super) fn open(_: kodex_realtime_webrtc::AudioDeviceSelection) -> io::Result<Self> {
        Err(io::Error::other(
            "audio devices unavailable for this helper target",
        ))
    }

    pub(super) fn set_controls(&self, _: kodex_realtime_webrtc::AudioControls) -> io::Result<()> {
        Self::open(Default::default()).map(|_| ())
    }

    pub(super) async fn service(
        &mut self,
        _: &mut crate::audio_track::AudioTrack,
    ) -> io::Result<usize> {
        Self::open(Default::default()).map(|_| 0)
    }
}

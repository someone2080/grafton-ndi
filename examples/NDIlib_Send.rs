use std::time::{Duration, Instant};
use grafton_ndi::{Error, VideoFrame, NDI, Send, Sender, FourCCVideoType, FrameFormatType};

fn main() -> Result<(), Error> {
    // Initialize the NDI library and ensure it's properly cleaned up
    let video_frame = VideoFrame::new(
        1920,
        1080,
        FourCCVideoType::RGBA,
        30,
        1,
        16.0 / 9.0,
        FrameFormatType::Progressive,
    );

    let start = Instant::now();
    let mut sender = get_sender()?;
    while start.elapsed() < Duration::from_secs(15) {
        sender.sender.send_video(&video_frame);
    }

    Ok(())
}

struct NdiSender {
    ndi: NDI,
    sender: Send,
}

fn get_sender() -> Result<NdiSender, Error> {
    if let Ok(ndi) = NDI::new() {
        let sender_options = Sender {
            name: "Sample NDI Send".to_string(),
            groups: None,
            clock_video: false,
            clock_audio: false,
        };

        let sender = Send::new(sender_options)?;

        return Ok(NdiSender {
            ndi,
            sender
        });
    } else {
        Err(Error::InitializationFailed(
            "Failed to create NDI send instance".into(),
        ))
    }
}

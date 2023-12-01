use anyhow::Context;
use awedio::Sound;

pub struct AudioState {
    files: Vec<RecordingOption>,
    manager: awedio::manager::Manager,
    backend: awedio::backends::CpalBackend,
}

impl AudioState {
    pub fn mvp_default() -> anyhow::Result<Self> {
        let (manager, backend) = awedio::start().context("starting awedio")?;
        Ok(AudioState {
            files: Vec::new(),
            manager,
            backend,
        })
    }
}

trait ToSound {
    fn to_sound(&self) -> Box<dyn Sound>;
}

struct RecordingOption {
    id: &'static str,
    weight: f64,
    source: Box<dyn ToSound>,
}

impl AudioState {
    pub fn play(&mut self, id: &str) -> Option<tokio::sync::oneshot::Receiver<()>> {
        if let Some(pick) = self.pick_recording_at_rand(id) {
            let (sound, notifier) = pick.source.to_sound().with_async_completion_notifier();
            self.manager.play(Box::new(sound));
            Some(notifier)
        } else {
            tracing::error!(?id, "No sound bite found");
            None
        }
    }
    pub fn play_bike_bell(&mut self) -> tokio::sync::oneshot::Receiver<()> {
        let bike_bell = awedio::sounds::decoders::Mp3Decoder::new(std::io::Cursor::new(
            include_bytes!("../../sounds/bike-bell.mp3"),
        ));
        let (sound, notifier) = bike_bell.with_async_completion_notifier();
        self.manager.play(Box::new(sound));
        notifier
    }
    fn pick_recording_at_rand(&self, id: &str) -> Option<&RecordingOption> {
        let files = self.files.iter().filter(|a| a.id == id).collect::<Vec<_>>();
        if files.is_empty() {
            return None;
        } else {
            let total = files.iter().fold(0f64, |acc, a| a.weight + acc);
            let rand = stupid_rand_f64();
            let mut pick = total * rand;
            for f in files {
                pick -= f.weight;
                if pick < 0f64 {
                    return Some(f);
                }
            }
        }

        None
    }
}

fn stupid_rand_f64() -> f64 {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    ((dur.as_micros() + dur.as_millis()).rem_euclid(1000) as f64) * 0.001f64
}

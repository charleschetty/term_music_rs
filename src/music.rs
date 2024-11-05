use std::{
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use lofty::file::AudioFile;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct MusicHandle {
    sink: Arc<Sink>,
    music_output: Arc<(OutputStream, OutputStreamHandle)>,
    time_played: Arc<Mutex<u32>>,
    volume: f32,
}

impl MusicHandle {
    pub fn new() -> Self {
        Self {
            sink: Arc::new(Sink::new_idle().0),
            music_output: Arc::new(OutputStream::try_default().unwrap()),
            time_played: Arc::new(Mutex::new(0)),
            volume: 1.0,
        }
    }
    pub fn play_new(&mut self, file_name: PathBuf) {
        self.sink.stop();
        *self.time_played.lock().unwrap() = 0;

        self.sink = Arc::new(Sink::try_new(&self.music_output.1).unwrap());

        let file = std::fs::File::open(&file_name).unwrap();
        self.sink
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        self.sink.set_volume(self.volume);

        let sclone = self.sink.clone();
        let tpclone = self.time_played.clone();

        let _t1 = thread::spawn(move || {
            // let file = BufReader::new(File::open(file_name).unwrap());
            // let source = rodio::Decoder::new(file).unwrap();

            let sink_clone_2 = sclone.clone();
            let tpclone2 = tpclone.clone();
            // sclone.append(source);

            let _ = thread::spawn(move || {
                while sink_clone_2.len() == 1 {
                    thread::sleep(Duration::from_secs(1));
                    if !sink_clone_2.is_paused() {
                        *tpclone2.lock().unwrap() += 1;
                    }
                }
            });
            sclone.sleep_until_end();
        });
    }

    pub fn play_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn stop(&mut self) {
        self.sink.stop();
    }

    pub fn set_time_played(&mut self, t: u32) {
        *self.time_played.lock().unwrap() = t;
    }
    pub fn time_played(&self) -> u32 {
        *self.time_played.lock().unwrap()
    }
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn change_volume(&mut self, volume: f32) {
        self.volume += volume;
        if self.volume < 0. {
            self.volume = 0.;
        } else if self.volume > 1. {
            self.volume = 1.;
        }
        self.sink.set_volume(self.volume)
    }
    pub fn get_volume(&self) -> f32 {
        self.sink.volume()
    }
}

pub fn get_song_length(path: &PathBuf) -> Option<u32> {
    let path = std::path::Path::new(&path);
    let tagged_file = match lofty::probe::Probe::open(path)
        .expect("ERROR: Bad path provided!")
        .read()
    {
        Ok(item) => item,
        Err(_) => return None,
    };

    let properties = &tagged_file.properties();
    let duration = properties.duration();
    Some(duration.as_secs() as u32)
}

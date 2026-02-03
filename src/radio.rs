use std::fs::File;
use std::os::fd::AsRawFd;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};
use std::thread::{self, JoinHandle};

use rodio::{Decoder, OutputStreamBuilder, Sink};
use stream_download::{
    Settings, StreamDownload, http::HttpStream, http::reqwest, storage::temp::TempStorageProvider,
};

pub struct Station {
    pub name: &'static str,
    pub url: &'static str,
}

pub const STATIONS: &[Station] = &[
    Station {
        name: "Chillout",
        url: "https://streams.ilovemusic.de/iloveradio17.mp3",
    },
    Station {
        name: "Lounge",
        url: "https://streams.ilovemusic.de/iloveradio2.mp3",
    },
    Station {
        name: "Relax FM",
        url: "https://stream.relaxfm.ru/relaxfm192.mp3",
    },
    Station {
        name: "Smooth Jazz",
        url: "https://streaming.radio.co/s774887f7b/listen",
    },
    Station {
        name: "Ambient",
        url: "https://streams.ilovemusic.de/iloveradio6.mp3",
    },
];

// Radio states
const STATE_STOPPED: u8 = 0;
const STATE_LOADING: u8 = 1;
const STATE_PLAYING: u8 = 2;
const STATE_ERROR: u8 = 3;

pub struct Radio {
    pub current_station: usize,
    state: Arc<AtomicU8>,
    stop_flag: Arc<AtomicU8>,
    volume: Arc<AtomicU8>, // 0-100
    handle: Option<JoinHandle<()>>,
}

impl Radio {
    pub fn new() -> Self {
        Self {
            current_station: 0,
            state: Arc::new(AtomicU8::new(STATE_STOPPED)),
            stop_flag: Arc::new(AtomicU8::new(0)),
            volume: Arc::new(AtomicU8::new(100)),
            handle: None,
        }
    }

    pub fn set_volume(&self, vol: u8) {
        self.volume.store(vol.min(100), Ordering::SeqCst);
    }

    pub fn volume_handle(&self) -> Arc<AtomicU8> {
        self.volume.clone()
    }

    pub fn station(&self) -> &Station {
        &STATIONS[self.current_station]
    }

    pub fn is_loading(&self) -> bool {
        self.state.load(Ordering::SeqCst) == STATE_LOADING
    }

    pub fn is_playing(&self) -> bool {
        self.state.load(Ordering::SeqCst) == STATE_PLAYING
    }

    pub fn is_error(&self) -> bool {
        self.state.load(Ordering::SeqCst) == STATE_ERROR
    }

    pub fn toggle(&mut self) {
        let state = self.state.load(Ordering::SeqCst);
        if state == STATE_STOPPED || state == STATE_ERROR {
            self.play();
        } else {
            self.stop();
        }
    }

    pub fn play(&mut self) {
        let current_state = self.state.load(Ordering::SeqCst);
        if current_state == STATE_LOADING || current_state == STATE_PLAYING {
            return;
        }

        self.stop_flag.store(0, Ordering::SeqCst);
        self.state.store(STATE_LOADING, Ordering::SeqCst);

        let url = STATIONS[self.current_station].url.to_string();
        let stop_flag = self.stop_flag.clone();
        let state = self.state.clone();
        let volume = self.volume.clone();

        let handle = thread::spawn(move || {
            // Create audio output
            let audio_stream = match OutputStreamBuilder::open_default_stream() {
                Ok(s) => s,
                Err(_) => {
                    state.store(STATE_ERROR, Ordering::SeqCst);
                    return;
                }
            };

            let sink = Sink::connect_new(audio_stream.mixer());

            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Check if stopped while setting up
                if stop_flag.load(Ordering::SeqCst) == 1 {
                    return;
                }

                let client = reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::limited(10))
                    .build()
                    .unwrap();

                let http_stream = match HttpStream::new(client, url.parse().unwrap()).await {
                    Ok(s) => s,
                    Err(_) => {
                        state.store(STATE_ERROR, Ordering::SeqCst);
                        return;
                    }
                };

                if stop_flag.load(Ordering::SeqCst) == 1 {
                    return;
                }

                let reader = match StreamDownload::from_stream(
                    http_stream,
                    TempStorageProvider::new(),
                    Settings::default(),
                )
                .await
                {
                    Ok(r) => r,
                    Err(_) => {
                        state.store(STATE_ERROR, Ordering::SeqCst);
                        return;
                    }
                };

                if stop_flag.load(Ordering::SeqCst) == 1 {
                    return;
                }

                let source = match Decoder::new(reader) {
                    Ok(s) => s,
                    Err(_) => {
                        state.store(STATE_ERROR, Ordering::SeqCst);
                        return;
                    }
                };

                sink.append(source);
                sink.set_volume(volume.load(Ordering::SeqCst) as f32 / 100.0);
                sink.play();

                // Now playing!
                state.store(STATE_PLAYING, Ordering::SeqCst);

                // Keep thread alive, poll volume changes
                while stop_flag.load(Ordering::SeqCst) == 0 {
                    let vol = volume.load(Ordering::SeqCst) as f32 / 100.0;
                    sink.set_volume(vol);
                    thread::sleep(std::time::Duration::from_millis(50));
                }

                sink.stop();
            });

            drop(sink);

            // Suppress rodio's "Dropping OutputStream" message
            unsafe {
                let null = File::open("/dev/null").ok();
                let old_stderr = libc::dup(2);
                if let Some(ref f) = null {
                    libc::dup2(f.as_raw_fd(), 2);
                }
                drop(audio_stream);
                if old_stderr >= 0 {
                    libc::dup2(old_stderr, 2);
                    libc::close(old_stderr);
                }
            }
        });

        self.handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.stop_flag.store(1, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        self.state.store(STATE_STOPPED, Ordering::SeqCst);
    }

    pub fn next_station(&mut self) {
        let was_active = self.is_playing() || self.is_loading();
        self.stop();
        self.current_station = (self.current_station + 1) % STATIONS.len();
        if was_active {
            self.play();
        }
    }

    pub fn prev_station(&mut self) {
        let was_active = self.is_playing() || self.is_loading();
        self.stop();
        self.current_station = if self.current_station == 0 {
            STATIONS.len() - 1
        } else {
            self.current_station - 1
        };
        if was_active {
            self.play();
        }
    }
}

impl Drop for Radio {
    fn drop(&mut self) {
        self.stop();
    }
}

// This is free and unencumbered software released into the public domain.

use hf_hub::api::Progress as HfProgress;
use indicatif::{ProgressBar as IBar, ProgressStyle};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Progress {
    inner: Arc<Mutex<IBar>>,
    started: Arc<Mutex<bool>>,
}

impl Progress {
    pub fn new() -> Self {
        let bar = IBar::new(0);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{wide_bar} {percent:>3}%")
                .unwrap()
                .progress_chars("█░"),
        );
        Self {
            inner: Arc::new(Mutex::new(bar)),
            started: Arc::new(Mutex::new(false)),
        }
    }

    pub fn global() -> &'static Self {
        use std::sync::OnceLock;
        static GLOBAL: OnceLock<Progress> = OnceLock::new();
        GLOBAL.get_or_init(Self::new)
    }
}

impl HfProgress for Progress {
    fn init(&mut self, size: usize, _filename: &str) {
        if size == 0 {
            return;
        }
        let mut started = self.started.lock().unwrap();
        let bar = self.inner.lock().unwrap();
        bar.set_length(size as u64);
        bar.set_position(0);
        *started = true;
    }

    fn update(&mut self, n: usize) {
        if !*self.started.lock().unwrap() {
            return;
        }
        let bar = self.inner.lock().unwrap();
        let pos = bar.position().saturating_add(n as u64);
        bar.set_position(pos);
    }

    fn finish(&mut self) {
        let mut started = self.started.lock().unwrap();
        if !*started {
            return;
        }
        let bar = self.inner.lock().unwrap();
        bar.finish_and_clear();
        *started = false;
    }
}

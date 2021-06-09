use crate::util::{StatefulList, TabsState};
use diff::LineDifference;
use event_handle::event_handle::EventHandle;
use filewatch::FileWatch;
use std::sync::mpsc;
use std::time::Duration;
use store::store::Store;
use tui::text::Spans;

pub struct LineDifference1<'a> {
    pub name: &'a str,
    pub location: &'a str,
}

pub struct AutoStash {
    pub watch_path: String,
    pub watch: FileWatch,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub versions: StatefulList<&'a str>,
    pub available_versions: Vec<String>,
    pub new_version: Vec<LineDifference>,
    pub processed_diffs: Vec<Spans<'static>>,
    pub servers: Vec<LineDifference1<'a>>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Result<App<'a>, Box<dyn std::error::Error>> {
        Ok(App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["1h", "24h", "7 Tage"]),
            show_chart: true,
            available_versions: Vec::new(),
            versions: StatefulList::with_items(Vec::new()),
            processed_diffs: Vec::new(),
            new_version: Vec::new(),
            servers: vec![LineDifference1 {
                name: "foo",
                location: "bar",
            }],
        })
    }
    pub fn on_up(&mut self) {
        self.versions.previous();
    }

    pub fn on_down(&mut self) {
        self.versions.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}

#[derive(Clone)]
pub struct Config {
    pub store_path: String,
    pub watch_path: String,
    pub debounce_time: Duration,
}

use std::env;

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip binary
        args.next();

        let store_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a store path"),
        };

        let watch_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a watch path"),
        };

        let debounce_time = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a debounce time"),
        }
        .parse::<u64>()
        .unwrap();

        Ok(Config {
            store_path,
            watch_path,
            debounce_time: Duration::from_millis(debounce_time),
        })
    }
}

impl AutoStash {
    pub fn new(
        config: &Config,
        stack_sender: mpsc::Sender<Vec<LineDifference>>,
        version_sender: mpsc::Sender<Vec<LineDifference>>,
        undo_redo_sender: mpsc::Receiver<(u8, u8)>
    ) -> Result<AutoStash, Box<dyn std::error::Error>> {
        let store = Store::new(config.store_path.as_str(), config.watch_path.as_str())?;

        let event_handle = EventHandle::new(store, stack_sender, version_sender, undo_redo_sender);
        let watch = FileWatch::new(config.debounce_time, event_handle)?;

        Ok(AutoStash {
            watch,
            watch_path: config.watch_path.clone(),
        })
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.watch.start_watching(self.watch_path.as_str())
    }
}

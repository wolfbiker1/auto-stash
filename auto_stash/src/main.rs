use auto_stash::{AutoStash, Config};
use event_handle::event_handle::EventHandleCommunication;
use flume::unbounded;
use std::{env, process, thread};
use ui::ui::{UICommunication, UI};

fn main() {
    let (file_versions_to_ui, on_file_versions) = unbounded();
    let (undo_to_handle, on_undo) = unbounded();
    let (redo_to_handle, on_redo) = unbounded();
    let (time_frame_change_to_handle, on_time_frame_change) = unbounded();
    let (key_to_ui, on_key) = unbounded();
    let (quit_to_ui, on_quit) = unbounded();
    let (quit_to_handle, on_handle_quit) = unbounded();

    let ui = UI::new(
        "".to_string(),
        UICommunication {
            on_key,
            on_file_versions,
            on_quit: on_quit.clone(),
            time_frame_change_to_handle,
            key_to_ui,
            redo_to_handle,
            undo_to_handle,
            quit_to_ui,
            quit_to_handle,
        },
    );

    thread::spawn(|| {
        ui::run(ui).unwrap_or_else(|err| {
            eprintln!("Could not run ui: {:?}", err);
            process::exit(1);
        });
    });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut auto_stash = AutoStash::new(
        &config,
        EventHandleCommunication {
            file_versions_to_ui,
            on_redo,
            on_undo,
            on_time_frame_change,
        },
        on_handle_quit,
    )
    .unwrap_or_else(|err| {
        eprintln!("Problem creating auto stash: {:?}", err);
        process::exit(1);
    });

    auto_stash.run().unwrap_or_else(|err| {
        eprintln!("Could not run auto stash: {:?}", err);
        process::exit(1);
    });
}

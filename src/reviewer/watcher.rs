use std::io;
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use notify::{Event, RecursiveMode, Watcher};

use super::parser::parse_results;
use super::render::Renderer;
use super::results::Results;
use super::runner::{CargoOutcome, run_cargo_test};
use super::theme::{DEBOUNCE_MS, POLL_INTERVAL_MS};

const WATCH_PATH: &str = "src/exercises";

pub fn run() -> io::Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).map_err(map_notify_err)?;
    watcher
        .watch(Path::new(WATCH_PATH), RecursiveMode::Recursive)
        .map_err(map_notify_err)?;

    let mut renderer = Renderer::new();
    let mut last_results = Results::new();

    run_and_render(&mut renderer, &mut last_results)?;

    let poll = Duration::from_millis(POLL_INTERVAL_MS);
    let debounce = Duration::from_millis(DEBOUNCE_MS);
    let mut pending = false;

    loop {
        match rx.recv_timeout(poll) {
            Ok(Ok(_)) => pending = true,
            Ok(Err(e)) => eprintln!("watcher error: {e:?}"),
            Err(RecvTimeoutError::Timeout) if pending => {
                thread::sleep(debounce);
                while rx.try_recv().is_ok() {}
                run_and_render(&mut renderer, &mut last_results)?;
                pending = false;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

fn run_and_render(renderer: &mut Renderer, last_results: &mut Results) -> io::Result<()> {
    renderer.show_running();
    match run_cargo_test()? {
        CargoOutcome::Tests(raw) => {
            *last_results = parse_results(&raw);
            renderer.render(last_results, None);
        }
        CargoOutcome::CompileError(err) => {
            renderer.render(last_results, Some(&err));
        }
    }
    Ok(())
}

fn map_notify_err(err: notify::Error) -> io::Error {
    io::Error::other(format!("{err}"))
}

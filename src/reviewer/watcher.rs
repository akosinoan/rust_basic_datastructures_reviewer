use std::io::{self, BufRead, IsTerminal};
use std::path::Path;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;

use notify::{RecursiveMode, Watcher};

use super::parser::parse_results;
use super::render::Renderer;
use super::results::Results;
use super::runner::{CargoOutcome, run_cargo_test};
use super::theme::DEBOUNCE_MS;

const WATCH_PATH: &str = "src/exercises";

enum Event {
    Watch,
    Stdin(String),
}

pub fn run() -> io::Result<()> {
    let (tx, rx) = mpsc::channel::<Event>();

    let watch_tx = tx.clone();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        match res {
            Ok(_) => {
                let _ = watch_tx.send(Event::Watch);
            }
            Err(e) => eprintln!("watcher error: {e:?}"),
        }
    })
    .map_err(map_notify_err)?;
    watcher
        .watch(Path::new(WATCH_PATH), RecursiveMode::Recursive)
        .map_err(map_notify_err)?;

    if io::stdin().is_terminal() {
        spawn_stdin_reader(tx.clone());
    }

    let mut renderer = Renderer::new();
    let mut last_results = Results::new();

    run_and_render(&mut renderer, &mut last_results)?;

    let debounce = Duration::from_millis(DEBOUNCE_MS);
    let mut pending = false;

    loop {
        let timeout = if pending { debounce } else { Duration::from_secs(3600) };
        match rx.recv_timeout(timeout) {
            Ok(Event::Watch) => pending = true,
            Ok(Event::Stdin(line)) => {
                if line == "hint" {
                    renderer.show_hint(&last_results);
                }
            }
            Err(RecvTimeoutError::Timeout) if pending => {
                drain_pending_watch_events(&rx, &mut renderer, &last_results);
                run_and_render(&mut renderer, &mut last_results)?;
                pending = false;
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

fn drain_pending_watch_events(
    rx: &mpsc::Receiver<Event>,
    renderer: &mut Renderer,
    last_results: &Results,
) {
    while let Ok(ev) = rx.try_recv() {
        if let Event::Stdin(line) = ev {
            if line == "hint" {
                renderer.show_hint(last_results);
            }
        }
    }
}

fn spawn_stdin_reader(tx: Sender<Event>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };
            let normalized = line.trim().to_lowercase();
            if normalized.is_empty() {
                continue;
            }
            if tx.send(Event::Stdin(normalized)).is_err() {
                break;
            }
        }
    });
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

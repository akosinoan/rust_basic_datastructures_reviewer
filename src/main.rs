mod exercises;

#[cfg(feature = "solutions")]
mod solutions;

mod reviewer;

fn main() -> std::io::Result<()> {
    reviewer::watcher::run()
}

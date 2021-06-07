use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch() -> notify::Result<()> {
    let home = dirs::home_dir().unwrap_or(PathBuf::new());
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(0))?;
    watcher.watch(&home.join(".zshrc"), RecursiveMode::Recursive)?;
    watcher.watch(&home.join("Desktop"), RecursiveMode::Recursive)?;
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::NoticeWrite(path) => println!("NoticeWrite(({:?}))", &path),
                DebouncedEvent::NoticeRemove(path) => println!("NoticeRemove({:?})", &path),
                DebouncedEvent::Create(path) => println!("Create({:?})", &path),
                DebouncedEvent::Write(path) => println!("Write({:?})", &path),
                DebouncedEvent::Chmod(path) => println!("Chmod({:?})", &path),
                DebouncedEvent::Remove(path) => println!("Remove({:?})", &path),
                DebouncedEvent::Rename(old, new) => println!("Rename({:?}) -> ({:?})", &old, &new),
                DebouncedEvent::Rescan => todo!(),
                DebouncedEvent::Error(e, path) => {
                    println!("error: {} > path: ({:?})", e, &path.unwrap_or_default())
                }
            },
            Err(e) => println!("watch error: ({:?})", e),
        }
    }
}

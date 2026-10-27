mod kitty;

use crate::kitty::KittyRenderer;
use clap::Parser;
use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};
use notify::event::ModifyKind;
use notify::{Event, RecursiveMode, Watcher};
use rand::random;
use std::error::Error;
use std::io::{Write, stdin, stdout};
use std::{fs, path::Path, sync::mpsc};
use tiny_skia::{Color, Pixmap};

// TODO: add a "dir" flag - makes the file watcher watch entire directory including the script,
// recursively.

#[derive(Parser, Debug)]
#[command(name = "sketch")]
#[command(name = "Lua x Kitty = CLI sketchcoding")]
struct Args {
    #[arg()]
    file: String,
}

enum SketchEvent {
    ContentChanged,
    TerminalResized(u16, u16),
    TerminalError(std::io::Error),
    Tick,
    Quit,
}

struct TerminalSetup;

impl TerminalSetup {
    fn new() -> std::io::Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        Ok(Self)
    }
}

impl Drop for TerminalSetup {
    fn drop(&mut self) {
        let _ = execute!(stdout(), LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

// Thread to read terminal events and send them to a channel
// Current handled: resize
// TODO: mouse, keyboard
fn term_event_pump(tx: mpsc::Sender<SketchEvent>) {
    loop {
        match event::read() {
            Ok(event) => match event {
                event::Event::Resize(width, height) => {
                    let _ = tx.send(SketchEvent::TerminalResized(width, height));
                }
                event::Event::Key(key) => {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        let _ = tx.send(SketchEvent::Quit);
                    }
                }
                _ => {}
            },
            Err(err) => {
                let _ = tx.send(SketchEvent::TerminalError(err));
                return;
            }
        }
    }
}

fn make_image(width: u16, height: u16) -> Result<Pixmap, Box<dyn Error>> {
    Ok(Pixmap::new(width.try_into()?, height.try_into()?).unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _term = TerminalSetup::new()?;

    //
    //

    let mut kr = KittyRenderer::new(stdin(), stdout(), 0, 0);

    let size = terminal::window_size()?;
    let mut img = make_image(size.width, size.height)?;
    img.fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0).unwrap());

    kr.draw(&img)?;

    //
    //

    let source = fs::read_to_string(&args.file)?;
    // eprintln!("{source}");

    let (tx, rx) = mpsc::channel::<SketchEvent>();

    let term_tx = tx.clone();
    let _term_thread = std::thread::spawn(move || {
        let _ = term_event_pump(term_tx);
    });

    let mut watcher = notify::recommended_watcher(move |res| {
        let evt: Event = match res {
            Ok(evt) => evt,
            Err(e) => {
                // eprintln!("Watcher error: {e:?}");
                return;
            }
        };

        match evt.kind {
            notify::EventKind::Modify(ModifyKind::Any | ModifyKind::Data(_)) => {
                let _ = tx.send(SketchEvent::ContentChanged);
            }
            _ => {}
        };
    })?;

    watcher.watch(Path::new(&args.file), RecursiveMode::NonRecursive)?;
    'pump: for res in rx {
        match res {
            SketchEvent::ContentChanged => {
                let source = fs::read_to_string(&args.file)?;
                // eprintln!("{source}");
            }
            SketchEvent::TerminalResized(width, height) => {
                let size = terminal::window_size()?;
                if size.width == 0 || size.height == 0 {
                    println!("no window size!");
                }
                img = make_image(size.width, size.height)?;
                img.fill(Color::from_rgba(random(), random(), random(), 1.0).unwrap());
                kr.draw(&img)?;
            }
            SketchEvent::TerminalError(_err) => {
                println!("TERMINAL ERROR");
                break 'pump;
            }
            SketchEvent::Tick => {}
            SketchEvent::Quit => {
                break 'pump;
            }
        }
    }

    Ok(())
}

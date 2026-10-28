mod drawing_context;
mod kitty;

use crate::drawing_context::DrawingContext;
use crate::kitty::KittyRenderer;
use clap::Parser;
use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{cursor, execute, terminal};
use notify::event::ModifyKind;
use notify::{Event, RecursiveMode, Watcher};
use rand::random;
use std::error::Error;
use std::io::{stdin, stdout};
use std::thread::sleep;
use std::time::{Duration, Instant};
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
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let _term = TerminalSetup::new()?;

    //
    //

    let mut kr = KittyRenderer::new(stdout());

    let size = terminal::window_size()?;

    let mut dc = DrawingContext::new(u32::from(size.width), u32::from(size.height))?;
    dc.clear();

    kr.draw(&dc.image)?;

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

    let mut last_frame_rendered_at = Instant::now();
    let mut run = true;
    while run {
        loop {
            let evt = rx.try_recv();
            if evt.is_err() {
                break;
            }
            match evt? {
                SketchEvent::ContentChanged => {
                    let source = fs::read_to_string(&args.file)?;
                    // eprintln!("{source}");
                }
                SketchEvent::TerminalResized(width, height) => {
                    let size = terminal::window_size()?;
                    if size.width == 0 || size.height == 0 {
                        println!("no window size!");
                    }
                    dc.resize(u32::from(size.width), u32::from(size.height))?;
                }
                SketchEvent::TerminalError(_err) => {
                    println!("TERMINAL ERROR");
                    run = false;
                }
                SketchEvent::Tick => {}
                SketchEvent::Quit => {
                    run = false;
                }
            }
        }

        //
        // This is where the Lua update() call will be...

        dc.set_fill(Color::from_rgba(random(), random(), random(), 1.0).unwrap());
        let x: f32 = random::<f32>() * (dc.width() as f32);
        let y: f32 = random::<f32>() * (dc.height() as f32);
        let w: f32 = random::<f32>() * 100.0;
        let h: f32 = random::<f32>() * 100.0;
        dc.fill_rect(x, y, w, h);

        //
        //

        let render_due_at = last_frame_rendered_at + Duration::from_micros(1_000_000 / 30);
        let sleep_time = render_due_at - Instant::now();
        if sleep_time.is_zero() {
            dc.set_fill(Color::from_rgba(1.0, 0.0, 0.0, 1.0).unwrap());
        } else {
            dc.set_fill(Color::from_rgba(0.0, 1.0, 0.0, 1.0).unwrap());
            sleep(sleep_time);
        }

        last_frame_rendered_at = Instant::now();

        dc.fill_rect(0.0, 0.0, 10.0, 10.0);
        kr.draw(&dc.image)?;
    }

    Ok(())
}

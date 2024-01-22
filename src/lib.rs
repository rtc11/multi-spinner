pub mod spinners;

use std::{
    sync::{Arc, Mutex},
    thread::{self},
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::MoveToRow,
    style::Print,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use crate::spinners::Animation;

/// # Example:
/// ```rust
///use multi_spinner::{Spinner, spinners::Animation};
///use std::{ thread::{self}, time::Duration, sync::{Arc, Mutex}, };
///
///fn main() {
///    let stdout = Arc::new(Mutex::new(std::io::stdout()));
///    let files = ["file1", "file2", "file3", "file4", "file5"];
///    let handles = files.iter().enumerate().map(|(i, file)|{
///        let file = file.to_owned();
///        let stdout = stdout.clone(); 
///        thread::spawn(move ||{
///
///            let mut spinner = Spinner::builder()
///                .spinner(Animation::Bars10(0))
///                .msg(format!("downloading {file}\n"))
///                .row(i)
///                .stdout(stdout)
///                .build();
///
///            spinner.start();
///            thread::sleep(Duration::from_secs(3));
///            spinner.stop().expect("stopped");
///        })
///    }).collect::<Vec<_>>();
///
///    for handle in handles {
///        let () = handle.join().expect("join thread");
///    }
///}
///```
#[must_use]
#[derive(Clone)]
pub struct Spinner {
    spinner: Animation,
    row: usize,
    msg: String,
    active: Arc<Mutex<bool>>,
    stdout: Arc<Mutex<std::io::Stdout>>,
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl Spinner {
    pub fn builder() -> SpinnerBuilder {
        SpinnerBuilder::default()
    }

    /// Starts the spinner in a separate thread.
    ///
    /// # Panics
    /// Will panic if active, stdout or handle is poisoned.
    pub fn start(&mut self) {
        *self.active.lock().expect("lock active's mutex") = true;

        let active = self.active.clone();
        let mut spinner = self.clone();

        let handle = thread::spawn(move || {
            while *active.lock().expect("lock active's mutex") {
                spinner.spin().expect("spin");
                thread::sleep(Duration::from_millis(65_u64));
            }
        });

        *self.handle.lock().expect("lock handle's mutex") = Some(handle);
    }

    /// # Panics
    /// Will panic if stdout is poisoned.
    ///
    /// # Errors
    /// Fails if crossterm fails to execute or if conversion of usize to u16 fails.
    fn spin(&mut self) -> Result<()> {
        let mut stdout = self.stdout.lock().expect("lock stdout's mutex");
        let row = u16::try_from(self.row)?;
        stdout.execute(MoveToRow(row))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;

        let frame = self.spinner.next_frame();
        let msg = &self.msg;

        stdout.execute(MoveToRow(row))?;
        stdout.execute(Print(format!("{frame} {msg}")))?;
        drop(stdout);

        Ok(())
    }

    /// Stops the spinner
    ///
    /// # Panics
    /// Panics if either active, handle or stdout is poisoned or if joining thread panics.
    ///
    /// # Errors
    /// Fails if crossterm fails to execute or if conversion of usize to u16 fails.
    pub fn stop(&mut self) -> Result<()> {
        *self.active.lock().expect("lock active's mutex") = false;

        let handle = self
            .handle
            .clone()
            .lock()
            .expect("lock handle's mutex")
            .take();

        if let Some(handle) = handle {
            let () = handle.join().expect("join spinner thread");
        }

        let mut stdout = self.stdout.lock().expect("lock stdout's mutex");
        let row = u16::try_from(self.row)?;
        stdout.execute(MoveToRow(row))?;
        stdout.execute(Clear(ClearType::CurrentLine))?;
        drop(stdout);

        Ok(())
    }
}

#[must_use]
#[derive(Clone)]
pub struct SpinnerBuilder {
    spinner: Animation,
    row: usize,
    msg: String,
    active: Arc<Mutex<bool>>,
    stdout: Arc<Mutex<std::io::Stdout>>,
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl Default for SpinnerBuilder {
    fn default() -> Self {
        Self {
            spinner: Animation::Dots2(0),
            row: 0,
            msg: "Loading".to_owned(),
            active: Arc::new(Mutex::new(false)),
            stdout: Arc::new(Mutex::new(std::io::stdout())),
            handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl SpinnerBuilder {
    /// If stdout is not provided, it will default to `std::io::stdout()`.
    pub fn stdout(mut self, stdout: Arc<Mutex<std::io::Stdout>>) -> Self {
        self.stdout = stdout;
        self
    }

    /// Message to display after the spinner.
    pub fn msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }

    /// Terminal row to display the spinner on.
    /// Defaults to 0.
    pub const fn row(mut self, row: usize) -> Self {
        self.row = row;
        self
    }

    /// Animation to use for the spinner.
    /// Defaults to `Animation::Dots2(0)`.
    pub const fn spinner(mut self, spinner: Animation) -> Self {
        self.spinner = spinner;
        self
    }

    /// Builds and starts the spinner.
    pub fn start(self) -> Spinner {
        let mut spinner = self.build();
        spinner.start();
        spinner
    }

    /// Builds the spinner.
    pub fn build(self) -> Spinner {
        Spinner {
            spinner: self.spinner,
            row: self.row,
            msg: self.msg,
            active: self.active,
            stdout: self.stdout,
            handle: self.handle,
        }
    }
}

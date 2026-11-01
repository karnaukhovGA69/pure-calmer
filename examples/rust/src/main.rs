//! Pure Calmer / Rust — types, traits, lifetimes and macros.
//! Красный — только для диагностики, не для Result, Err или panic!.
#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::sync::mpsc;
use std::thread;

// 01 / ATTRIBUTES, ENUMS, STRUCTS, TYPES, GENERICS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Running,
    Finished,
}

#[derive(Debug, Clone)]
struct Job<'a> {
    id: u64,
    name: &'a str,
    load: f64,
    status: Status,
}

#[derive(Debug)]
enum JobError {
    InvalidLoad(f64),
    MissingName,
}

type JobResult<T> = Result<T, JobError>;
const DEFAULT_LIMIT: usize = 3;
static THEME_NAME: &str = "Pure Calmer";

// 02 / TRAITS, ASSOCIATED TYPES, METHODS, REFERENCES
trait Runner {
    type Output;
    fn run(&mut self) -> JobResult<Self::Output>;
}

impl Runner for Job<'_> {
    type Output = String;

    fn run(&mut self) -> JobResult<Self::Output> {
        if !(0.0..=100.0).contains(&self.load) {
            return Err(JobError::InvalidLoad(self.load));
        }
        self.status = Status::Running;
        Ok(format!("{}: {:.1}%", self.name, self.load))
    }
}

impl Display for Job<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "#{} {}", self.id, self.name)
    }
}

// 03 / EXPLICIT LIFETIMES, GENERIC BOUNDS, WHERE
fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

fn describe<T>(value: &T) -> String
where
    T: Display + ?Sized,
{
    format!("value = {value}")
}

async fn predicted_load() -> JobResult<f64> {
    Ok(42.5)
}

macro_rules! section {
    ($title:expr) => {
        println!("\n── {} ──", $title);
    };
}

// 04 / LITERALS, ESCAPES, RAW STRINGS, PATTERNS
fn main() -> Result<(), String> {
    section!("Pastel syntax preview");
    let greeting = "Hello, пастельный мир!\n\tquoted: \"hello\"";
    let raw = r#"{"theme":"Pure Calmer","accent":"pink"}"#;
    let character = 'Ж';
    let bytes = b"hello";
    let (integer, hexadecimal, binary) = (42_u32, 0xCAFE, 0b1010);
    let (fraction, exponent) = (std::f64::consts::PI, 6.02e23_f64);
    let enabled = true;
    let missing: Option<&str> = None;

    let mut jobs = [
        Job {
            id: 1,
            name: "night batch",
            load: 18.5,
            status: Status::Idle,
        },
        Job {
            id: 2,
            name: "forecast",
            load: 42.0,
            status: Status::Idle,
        },
        Job {
            id: 3,
            name: "packing",
            load: 73.5,
            status: Status::Idle,
        },
    ];

    let names: Vec<_> = jobs.iter().map(|job| job.name).collect();
    let mut lookup: HashMap<&str, usize> = HashMap::new();
    lookup.insert("jobs", jobs.len());
    lookup.entry("limit").or_insert(DEFAULT_LIMIT);

    // 05 / MATCH, GUARDS, IF LET, ERROR PROPAGATION
    for job in &mut jobs {
        let output = job.run().map_err(|error| format!("{error:?}"))?;
        match job.load {
            load if load < 30.0 => println!("low: {output}"),
            30.0..=70.0 => println!("medium: {output}"),
            _ => println!("high: {output}"),
        }
    }
    if let Some(count) = lookup.get("jobs") {
        println!("{count} jobs; {}", longer("blue", "pastel pink"));
    }

    // 06 / CHANNELS, MOVE CLOSURES, THREADS, LABELS
    let (sender, receiver) = mpsc::channel();
    let worker = thread::spawn(move || sender.send("finished"));
    let update = receiver.recv().map_err(|error| error.to_string())?;
    worker
        .join()
        .map_err(|_| "worker panicked".to_owned())?
        .map_err(|error| error.to_string())?;
    'preview: for (index, name) in names.iter().enumerate() {
        println!("{index}: {name} / {update}");
        if index >= DEFAULT_LIMIT {
            break 'preview;
        }
    }

    println!("{greeting} {raw} {character} {bytes:?}");
    println!("{integer} {hexadecimal} {binary} {fraction} {exponent}");
    println!(
        "{enabled} {missing:?} {} {}",
        THEME_NAME,
        describe(&jobs[0])
    );

    // 07 / ДИАГНОСТИКА: убери // у следующей строки для красной ошибки.
    // let broken: u32 = "this is not a number";
    // TODO: обычный комментарий; Err — обычный вариант enum, не ошибка редактора.
    Ok(())
}

use std::io::{self, Write};
use std::ops::AddAssign;

#[derive(Clone, PartialEq, Eq)]
pub enum Direction {
    Clockwise,
    AntiClockwise,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tool {
    Off,
    On,
}

#[derive(Clone, PartialEq)]
pub enum Distance {
    Absolute,
    Incremental,
}

#[derive(Default, PartialEq, Clone)]
pub struct Program(Vec<GCode>);

impl std::ops::Deref for Program {
    type Target = [GCode];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AddAssign for Program {
    fn add_assign(&mut self, mut other: Program) {
        self.0.extend(other.0.drain(..));
    }
}

impl From<Vec<GCode>> for Program {
    fn from(v: Vec<GCode>) -> Self {
        Self(v)
    }
}

impl Program {
    pub fn push(&mut self, g: GCode) {
        self.0.push(g)
    }
}

macro_rules! write_if_some {
    ($w:expr, $s:expr, $v:ident) => {
        if let Some(v) = $v {
            write!($w, $s, v)
        } else {
            Ok(())
        }
    };
}

/// Rudimentary regular expression GCode validator.
pub fn validate_gcode(gcode: &&str) -> bool {
    use regex::Regex;
    let re = Regex::new(r##"^(?:(?:%|\(.*\)|(?:[A-Z^E^U][+-]?\d+(?:\.\d*)?))\h*)*$"##).unwrap();
    gcode.lines().all(|line| re.is_match(line))
}

#[derive(Clone, PartialEq)]
pub enum GCode {
    RapidPositioning {
        x: Option<f64>,
        y: Option<f64>,
    },
    LinearInterpolation {
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
        f: Option<f64>,
    },
    Dwell {
        p: f64,
    },
    UnitsInches,
    UnitsMillimeters,
    ProgramEnd,
    StartSpindle {
        d: Direction,
        s: f64,
    },
    StopSpindle,
    DistanceMode(Distance),
    Comment(Box<String>),
    Raw(Box<String>),
}

pub fn program2gcode<W: Write>(p: &Program, mut w: W) -> io::Result<()> {
    use GCode::*;
    let mut last_feedrate: Option<f64> = None;
    for code in p.iter() {
        match code {
            RapidPositioning { x, y } => {
                if let (None, None) = (x, y) {
                    continue;
                }
                write!(w, "G0")?;
                write_if_some!(w, " X{}", x)?;
                write_if_some!(w, " Y{}", y)?;
                writeln!(w)?;
            }
            LinearInterpolation { x, y, z, f } => {
                if let (None, None, None, None) = (x, y, z, f) {
                    continue;
                }

                let f = match (last_feedrate, f) {
                    (None, None) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Linear interpolation without previously set feedrate",
                        ))
                    }
                    (Some(last), Some(new)) => {
                        if (last - *new).abs() < std::f64::EPSILON {
                            last_feedrate = Some(*new);
                            Some(new)
                        } else {
                            None
                        }
                    }
                    (None, Some(new)) => {
                        last_feedrate = Some(*new);
                        Some(new)
                    }
                    (Some(_), None) => None,
                };
                write!(w, "G1")?;
                write_if_some!(w, " X{}", x)?;
                write_if_some!(w, " Y{}", y)?;
                write_if_some!(w, " Z{}", z)?;
                write_if_some!(w, " F{}", f)?;
                writeln!(w)?;
            }
            Dwell { p } => {
                writeln!(w, "G4 P{}", p)?;
            }
            UnitsInches => {
                writeln!(w, "G20")?;
            }
            UnitsMillimeters => {
                writeln!(w, "G21")?;
            }
            ProgramEnd => {
                writeln!(w, "M20")?;
            }
            StartSpindle { d, s } => {
                let d = match d {
                    Direction::Clockwise => 3,
                    Direction::AntiClockwise => 4,
                };
                writeln!(w, "M{} S{}", d, s)?;
            }
            StopSpindle => {
                writeln!(w, "M5")?;
            }
            DistanceMode(mode) => {
                writeln!(
                    w,
                    "G{}",
                    match mode {
                        Distance::Absolute => 90,
                        Distance::Incremental => 91,
                    }
                )?;
            }
            Comment(name) => {
                writeln!(w, "({})", name)?;
            }
            Raw(raw) => {
                writeln!(w, "{}", raw)?;
            }
        }
    }
    Ok(())
}

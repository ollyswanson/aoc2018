use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// has format [1518-11-01 00:00]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

pub type ID = u32;

#[derive(Debug)]
pub enum EventKind {
    Sleep,
    Wake,
    Start { id: ID },
}

#[derive(Debug)]
pub struct Event {
    datetime: DateTime,
    kind: EventKind,
}

impl FromStr for Event {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        // input of form
        // [1518-03-21 23:52] Guard #2887 begins shift
        // [1518-07-31 00:58] wakes up
        // [1518-05-16 00:22] falls asleep
        // seems to be some problems with rust analyzer and using lazy_static macro in function
        // scope
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                \[
                (?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})
                \s+
                (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})
                \]\s+
                # non capturing guard + id | wakes up / falls asleep
                (?:Guard\ \#(?P<id>[0-9]+)\ begins\ shift|(?P<sleep>.+))
                "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            Some(caps) => caps,
            None => return Err(From::from("unrecognized format for event")),
        };

        let datetime = DateTime {
            year: caps["year"].parse()?,
            month: caps["month"].parse()?,
            day: caps["day"].parse()?,
            hour: caps["hour"].parse()?,
            minute: caps["minute"].parse()?,
        };

        let kind = if let Some(id) = caps.name("id") {
            EventKind::Start {
                id: id.as_str().parse()?,
            }
        } else if &caps["sleep"] == "wakes up" {
            EventKind::Wake
        } else if &caps["sleep"] == "falls asleep" {
            EventKind::Sleep
        } else {
            return Err(From::from("unrecoginized event kind"));
        };

        Ok(Event { datetime, kind })
    }
}

pub struct RawLogs {
    events: Vec<Event>,
}

impl RawLogs {
    pub fn new(s: &str) -> Result<Self> {
        let mut events: Vec<Event> = Vec::new();

        for line in s.lines() {
            let event: Event = line.parse()?;
            events.push(event);
        }

        events.sort_by(|c1, c2| c1.datetime.cmp(&c2.datetime));

        Ok(RawLogs { events })
    }

    pub fn process_logs(self) -> Result<LogsByGuard> {
        let mut events = self.events.iter();
        let mut logs_by_guard: HashMap<ID, Vec<u32>> = HashMap::new();

        let id = if let Some(event) = events.next() {
            match event.kind {
                EventKind::Start { id } => id,
                _ => return Err(From::from("invalid events log, no guard on duty")),
            }
        } else {
            return Err(From::from("Empty logs"));
        };

        let mut log = logs_by_guard.entry(id).or_insert_with(|| vec![0; 60]);
        let mut fell_asleep: Option<u32> = None;

        for event in events {
            match event.kind {
                EventKind::Start { id } => {
                    if fell_asleep.is_some() {
                        return Err(From::from("Guard finished shift while asleep"));
                    }
                    log = logs_by_guard.entry(id).or_insert_with(|| vec![0; 60]);
                }
                EventKind::Wake => {
                    if let Some(fell_asleep) = fell_asleep.take() {
                        for minute in fell_asleep..event.datetime.minute {
                            log[minute as usize] += 1;
                        }
                    } else {
                        return Err(From::from("Can't wake twice in a row!"));
                    }
                }
                EventKind::Sleep => {
                    if fell_asleep.is_some() {
                        return Err(From::from("Can't sleep twice in a row"));
                    }

                    fell_asleep = Some(event.datetime.minute);
                }
            }
        }

        Ok(LogsByGuard {
            inner: logs_by_guard,
        })
    }
}

pub struct SleepiestMinute {
    pub minute: u32,
    pub frequency: u32,
}

pub struct LogsByGuard {
    inner: HashMap<ID, Vec<u32>>,
}

impl LogsByGuard {
    pub fn sleeps_most(&self) -> ID {
        let (&id, _) = self
            .inner
            .iter()
            .max_by_key(|(_, log)| log.iter().sum::<u32>())
            .unwrap();

        id
    }

    pub fn sleepiest_minute_by_guard(&self, id: ID) -> Option<SleepiestMinute> {
        let log = self.inner.get(&id)?;

        log.iter()
            .enumerate()
            .max_by(|x, y| x.1.cmp(y.1))
            .map(|(i, &f)| SleepiestMinute {
                minute: i as u32,
                frequency: f as u32,
            })
    }

    pub fn sleepiest_minute(&self) -> Option<(ID, SleepiestMinute)> {
        self.inner.keys().fold(None, |acc, &cur_id| {
            let cur_sm = self.sleepiest_minute_by_guard(cur_id)?;

            if let Some((_, ref sm)) = acc {
                if cur_sm.frequency > sm.frequency {
                    return Some((cur_id, cur_sm));
                }

                acc
            } else {
                Some((cur_id, cur_sm))
            }
        })
    }
}

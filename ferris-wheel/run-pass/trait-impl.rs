// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let a = match self {
            Duration::Minutes(val) => *val as u64 * 60u64 * 1000u64,
            Duration::Seconds(val) => *val as u64 * 1000u64,
            Duration::MilliSeconds(val) => *val,
        };
        let b = match other {
            Duration::Minutes(val) => *val as u64 * 60u64 * 1000u64,
            Duration::Seconds(val) => *val as u64 * 1000u64,
            Duration::MilliSeconds(val) => *val,
        };
        a == b
    }
}

use Duration::*;
fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}

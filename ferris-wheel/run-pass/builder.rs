// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<T: std::string::ToString>(&self, s: T) -> Self {
        Builder {
            string: Some(s.to_string()),
            number: self.number,
        }
    }

    fn number(&self, n: usize) -> Self {
        Builder {
            string: self.string.to_owned(),
            number: Some(n),
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            string: None,
            number: None,
        }
    }
}

impl std::string::ToString for Builder {
    fn to_string(&self) -> String {
        match (self.string.to_owned(), self.number) {
            (Some(s), None) => s,
            (None, Some(n)) => n.to_string(),
            (Some(s), Some(n)) => {
                let mut new_str = String::new();
                new_str.push_str(&s);
                new_str.push_str(" ");
                new_str.push_str(&n.to_string());
                new_str
            },
            (None, None) => String::from(""),
        }
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}

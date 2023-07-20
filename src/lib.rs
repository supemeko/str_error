use std::slice::Iter;

pub trait StrError {
    fn report(&mut self, msg: &str);
    fn throws(&mut self, msg: &str);
    fn catch(&self) -> bool;

    fn throws_as_err<T>(&mut self, msg: &str) -> Result<T, ()> {
        self.throws(msg);
        Err(())
    }

    fn throws_as_option<T>(&mut self, msg: &str) -> Option<T> {
        self.throws(msg);
        None
    }

    fn upthrows_if(&mut self) -> Option<()> {
        if self.catch() {
            Some(())
        } else {
            None
        }
    }
}

pub trait Reporter {
    fn report(&mut self, msg: &str);
}

impl Reporter for () {
    fn report(&mut self, _msg: &str) {}
}

pub struct DefaultStrError<T: Reporter> {
    pub err: Option<[char; 20]>,
    pub reporter: T,
}

pub struct DefaultReporter {
    pub data: Vec<String>,
}

impl Reporter for DefaultReporter {
    fn report(&mut self, msg: &str) {
        self.data.push(msg.to_string())
    }
}

impl<T: Reporter> StrError for DefaultStrError<T> {
    fn report(&mut self, msg: &str) {
        self.reporter.report(msg);
    }

    fn throws(&mut self, msg: &str) {
        let mut char = ['\0'; 20];
        for (i, c) in msg.chars().enumerate() {
            if i < 20 {
                char[i] = c;
            } else {
                break;
            }
        }
        self.err = Some(char);
    }

    fn catch(&self) -> bool {
        self.err.is_some()
    }
}

pub fn first() -> DefaultStrError<()> {
    DefaultStrError {
        err: None,
        reporter: (),
    }
}

pub fn reporter() -> DefaultStrError<DefaultReporter> {
    DefaultStrError {
        err: None,
        reporter: DefaultReporter { data: vec![] },
    }
}

impl DefaultStrError<DefaultReporter> {
    pub fn msg(&self) -> Iter<'_, String> {
        self.reporter.data.iter()
    }
}

impl DefaultStrError<()> {
    pub fn err_msg(&self) -> Option<String> {
        match self.err {
            Some(msg) => {
                let mut f = String::new();
                for c in msg {
                    if c == '\0' {
                        break;
                    }
                    f.push(c);
                }
                Some(f)
            }
            None => None,
        }
    }
}

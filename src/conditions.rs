#[derive(Clone, Debug)]
pub struct Conditions {
    pub timeout: u32,
    pub renew_timeout: Option<u32>,
    pub first_opens: Option<bool>,
    pub last_closes: Option<bool>,
    pub max_size: Option<usize>,
    pub patterns: Vec<String>
}

impl Conditions {
    fn new(timeout: u32) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            first_opens: None,
            last_closes: None,
            max_size: None,
            patterns: Vec::new()
        }
    }
}

pub struct Builder {
    conditions: Conditions
}

impl Builder {
    pub fn new(timeout: u32) -> Builder {
        Builder{
            conditions: Conditions::new(timeout)
        }
    }

    pub fn renew_timeout(&mut self, timeout: u32) -> &mut Builder {
        self.conditions.renew_timeout = Some(timeout);
        self
    }

    pub fn first_opens(&mut self, first_opens: bool) -> &mut Builder {
        self.conditions.first_opens = Some(first_opens);
        self
    }

    pub fn last_closes(&mut self, last_closes: bool) -> &mut Builder {
        self.conditions.last_closes = Some(last_closes);
        self
    }
    pub fn max_size(&mut self, max_size: usize) -> &mut Builder {
        self.conditions.max_size = Some(max_size);
        self
    }

    pub fn patterns(&mut self, patterns: Vec<String>) -> &mut Builder {
        self.conditions.patterns = patterns;
        self
    }

    pub fn build(&mut self) -> Conditions {
        self.conditions.clone()
    }
}

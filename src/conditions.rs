#[derive(Clone)]
pub struct Conditions {
    pub timeout: u32,
    pub renew_timeout: Option<u32>,
    pub last_closes: Option<bool>,
    pub max_size: Option<usize>
}

impl Conditions {
    fn new(timeout: u32) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            last_closes: None,
            max_size: None
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

    pub fn build(&mut self) -> Conditions {
        self.conditions.clone()
    }
}

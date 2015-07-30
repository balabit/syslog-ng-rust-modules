#[derive(Clone)]
pub struct Conditions {
    pub timeout: u32,
    pub renew_timeout: Option<u32>,
    pub uuid: Option<String>,
    pub max_size: Option<usize>
}

impl Conditions {
    fn new(timeout: u32) -> Conditions {
        Conditions {
            timeout: timeout,
            renew_timeout: None,
            uuid: None,
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

    pub fn uuid(&mut self, uuid: String) -> &mut Builder {
        self.conditions.uuid = Some(uuid);
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

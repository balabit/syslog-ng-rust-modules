use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Condition(Rc<Cell<bool>>);

impl Condition {
    pub fn new(state: bool) -> Condition {
        Condition(Rc::new(Cell::new(state)))
    }

    pub fn is_active(&self) -> bool {
        self.0.get()
    }

    pub fn activate(&mut self) {
        self.0.set(true);
    }

    pub fn deactivate(&mut self) {
        self.0.set(false);
    }
}

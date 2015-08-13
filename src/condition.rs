use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Condition(Rc<RefCell<bool>>);

impl Condition {
    pub fn new(state: bool) -> Condition {
        Condition(Rc::new(RefCell::new(state)))
    }

    pub fn is_active(&self) -> bool {
        *self.0.borrow()
    }

    pub fn activate(&mut self) {
        *self.0.borrow_mut() = true;
    }

    pub fn deactivate(&mut self) {
        *self.0.borrow_mut() = false;
    }
}

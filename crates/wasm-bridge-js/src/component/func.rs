use std::rc::Rc;

use js_sys::Function;

use crate::DropHandler;

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
    _closures: Rc<[DropHandler]>,
}

impl Func {
    pub(crate) fn new(function: Function, closures: Rc<[DropHandler]>) -> Self {
        Self {
            function,
            _closures: closures,
        }
    }
}

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Resource<T> {
    id: u32,
    _phantom: PhantomData<Box<T>>,
}

impl<T> Resource<T> {
    pub fn new_own(id: u32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn rep(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ResourceAny {
    pub(crate) id: u32,
}

impl ResourceAny {
    pub fn rep(&self) -> u32 {
        self.id
    }
}

pub struct ResourceType;

impl ResourceType {
    pub fn host<T>() {}
}

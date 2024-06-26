use std::any::Any;

use slab::Slab;
use thiserror::Error;

use super::Resource;

// TODO: unify with wasi's resource table?
#[derive(Default, Debug)]
pub struct ResourceTable(Slab<Box<dyn Any + Send>>);

impl ResourceTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<R: Any + Send>(&mut self, value: R) -> Result<Resource<R>, ResourceTableError> {
        let index = self.0.insert(Box::new(value));
        Ok(Resource::new_own(index as u32))
    }

    pub fn get<R: Any + Send>(&self, resource: &Resource<R>) -> Result<&R, ResourceTableError> {
        self.0
            .get(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast_ref()
            .ok_or(ResourceTableError::WrongType)
    }

    pub fn get_mut<R: Any + Send>(
        &mut self,
        resource: &Resource<R>,
    ) -> Result<&mut R, ResourceTableError> {
        self.0
            .get_mut(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast_mut()
            .ok_or(ResourceTableError::WrongType)
    }

    pub fn delete<R: Any + Send>(
        &mut self,
        resource: Resource<R>,
    ) -> Result<R, ResourceTableError> {
        Ok(*(self
            .0
            .try_remove(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast::<R>()
            .map_err(|_| ResourceTableError::WrongType)?))
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
/// Errors returned by operations on `ResourceTable`
pub enum ResourceTableError {
    /// ResourceTable has no free keys
    #[error("ResourceTable has no free keys")]
    Full,
    /// Resource not present in table
    #[error("Resource not present in table")]
    NotPresent,
    /// Resource present in table, but with a different type
    #[error("Resource present in table, but with a different type")]
    WrongType,
    /// Resource cannot be deleted because child resources exist in the table. Consult wit docs for
    /// the particular resource to see which methods may return child resources.
    #[error("Resource cannot be deleted because child resources exist in the table")]
    HasChildren,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    pub fn get_resource() {
        let mut table = ResourceTable::new();

        let resource = table.push("hello").unwrap();

        assert_eq!(*table.get(&resource).unwrap(), "hello");
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    pub fn delete_resource() {
        let mut table = ResourceTable::new();

        let resource = table.push("hello").unwrap();

        table.delete(resource.clone()).unwrap();

        assert_eq!(
            table.get(&resource).unwrap_err(),
            ResourceTableError::NotPresent
        );
    }

    #[wasm_bindgen_test::wasm_bindgen_test]
    pub fn wrong_type() {
        let mut table = ResourceTable::new();

        let resource = table.push("hello").unwrap();

        table.delete(resource.clone()).unwrap();

        table.push(42).unwrap();

        assert_eq!(
            table.get(&resource).unwrap_err(),
            ResourceTableError::WrongType
        );
    }
}

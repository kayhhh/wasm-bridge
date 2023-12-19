// From this PR https://github.com/kajacx/wasm-bridge/pull/3 by zimond

use js_sys::{Reflect, Uint8Array, WebAssembly};

use crate::{
    helpers::{map_js_error, static_str_to_js},
    AsContextMut, Result,
};

#[derive(Clone)]
pub struct Memory {
    memory: WebAssembly::Memory,
}

impl Memory {
    pub(crate) fn new(memory: WebAssembly::Memory) -> Self {
        Self { memory }
    }

    // We need this for compatible signature with wasmtime
    pub fn write(&self, _: impl AsContextMut, offset: usize, buffer: &[u8]) -> Result<()> {
        self.write_impl(offset, buffer)
    }

    pub(crate) fn write_impl(&self, offset: usize, buffer: &[u8]) -> Result<()> {
        let memory = Reflect::get(&self.memory, static_str_to_js("buffer"))
            .map_err(map_js_error("Memory has no buffer field"))?;
        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_from(buffer);
        Ok(())
    }

    // We need this for compatible signature with wasmtime
    pub fn read(&self, _: impl AsContextMut, offset: usize, buffer: &mut [u8]) -> Result<()> {
        self.read_impl(offset, buffer)
    }

    pub(crate) fn read_impl(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        let memory = Reflect::get(&self.memory, static_str_to_js("buffer"))
            .map_err(map_js_error("Memory has no buffer field"))?;
        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_to(buffer);
        Ok(())
    }
}

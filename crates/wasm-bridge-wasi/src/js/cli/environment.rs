use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use crate::js::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:cli/environment@0.2.0")?.func_wrap(
        "get-environment",
        |mut caller: StoreContextMut<T>, (): ()| {
            let env_vars = caller.data_mut().ctx().env_variables();
            Ok(env_vars.to_owned())
        },
    )
}

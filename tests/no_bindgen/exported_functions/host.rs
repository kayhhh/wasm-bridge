use wasm_bridge::*;

pub async fn run_test(bytes: &[u8]) -> Result<()> {
    let mut store = Store::<()>::default();
    #[allow(deprecated)]
    let module = Module::new(store.engine(), bytes).unwrap();
    #[allow(deprecated)]
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    single_value(&mut store, &instance).unwrap();
    few_values(&mut store, &instance).unwrap();
    many_values(&mut store).unwrap();
    errors(bytes).unwrap();

    Ok(())
}

fn single_value(mut store: &mut Store<()>, instance: &Instance) -> Result<()> {
    // Signed integers
    let add_five_i32 = instance.get_typed_func::<i32, i32>(&mut store, "add_five_i32").unwrap();

    for number in [-10, -1, 0, 10, i32::MIN + 1, i32::MAX - 2] {
        let returned = add_five_i32.call(&mut store, number).unwrap();
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_i64 = instance.get_typed_func::<i64, i64>(&mut store, "add_five_i64").unwrap();

    for number in [-10, -1, 0, 10, i64::MIN + 1, i64::MAX - 2] {
        let returned = add_five_i64.call(&mut store, number).unwrap();
        assert_eq!(returned, number.wrapping_add(5));
    }

    // Unsigned integers
    let add_five_u32 = instance.get_typed_func::<u32, u32>(&mut store, "add_five_i32").unwrap();

    for number in [0, 10, u32::MAX / 2 - 1, u32::MAX - 2] {
        let returned = add_five_u32.call(&mut store, number).unwrap();
        assert_eq!(returned, number.wrapping_add(5));
    }

    let add_five_u64 = instance.get_typed_func::<u64, u64>(&mut store, "add_five_i64").unwrap();

    for number in [0, 10, u64::MAX / 2 - 1, u64::MAX - 2] {
        let returned = add_five_u64.call(&mut store, number).unwrap();
        assert_eq!(returned, number.wrapping_add(5));
    }

    // Floats
    let add_five_f32 = instance.get_typed_func::<f32, f32>(&mut store, "add_five_f32").unwrap();

    for number in [0.0, 10.25, -2.5, 1_000_000.5, -1_000_000.5] {
        let returned = add_five_f32.call(&mut store, number).unwrap();
        assert_eq!(returned, number + 5.0);
    }

    let add_five_f64 = instance.get_typed_func::<f64, f64>(&mut store, "add_five_f64").unwrap();

    for number in [0.0, 10.25, -2.5, 10_000_000_000.5, -10_000_000_000.5] {
        let returned = add_five_f64.call(&mut store, number).unwrap();
        assert_eq!(returned, number + 5.0);
    }

    Ok(())
}

fn few_values(mut store: &mut Store<()>, instance: &Instance) -> Result<()> {
    // Multiple arguments
    let add_i32 = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add_i32").unwrap();

    let returned = add_i32.call(&mut store, (5, 10)).unwrap();
    assert_eq!(returned, 5 + 10);

    // Single-element tuple
    let add_five_f64 = instance.get_typed_func::<(f64,), (f64,)>(&mut store, "add_five_f64").unwrap();
    let returned = add_five_f64.call(&mut store, (5.5,)).unwrap();
    assert_eq!(returned, (5.5 + 5.0,));

    Ok(())
}

fn many_values(mut store: &mut Store<()>) -> Result<()> {
    let wat = r#"(module
        (func $add_ten_all (export "add_ten_all")
          (param $p0 i32) (param $p1 i64) (param $p2 i32) (param $p3 i64) (param $p4 f32) (param $p5 f64) (result i32 i64 i32 i64 f32 f64)
          (i32.add (local.get $p0) (i32.const 10))
          (i64.add (local.get $p1) (i64.const 10))
          (i32.add (local.get $p2) (i32.const 10))
          (i64.add (local.get $p3) (i64.const 10))
          (f32.add (local.get $p4) (f32.const 10))
          (f64.add (local.get $p5) (f64.const 10))
        )
      )
    "#;

    #[allow(deprecated)]
    let module = Module::new(store.engine(), wat.as_bytes()).unwrap();

    #[allow(deprecated)]
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    let add_ten_all = instance
        .get_typed_func::<(i32, i64, u32, u64, f32, f64), (i32, i64, u32, u64, f32, f64)>(
            &mut store,
            "add_ten_all",
        ).unwrap();
    let returned = add_ten_all.call(&mut store, (5, 15, 25, 35, 45.5, 55.5)).unwrap();
    assert_eq!(returned, (15, 25, 35, 45, 55.5, 65.5));

    Ok(())
}

fn errors(bytes: &[u8]) -> Result<()> {
    let mut store = Store::<()>::default();

    #[allow(deprecated)]
    Module::new(store.engine(), &[1, 5])
        .map(|_| ())
        .expect_err("parsing module from invalid binary bytes");

    #[allow(deprecated)]
    Module::new(store.engine(), "not a valid wat module".as_bytes())
        .map(|_| ())
        .expect_err("parsing module from invalid wat text");

    #[allow(deprecated)]
    let module = Module::new(store.engine(), bytes).unwrap();
    #[allow(deprecated)]
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    instance
        .get_typed_func::<i32, i32>(&mut store, "non_existing")
        .map(|_| ())
        .expect_err("trying to get a non existing function");

    // TODO: Number of arguments in currently the only type info available
    // Maybe look into how wasmer does it?
    // Bad number of arguments
    instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "add_five_i32")
        .map(|_| ())
        .expect_err("incorrect number if input arguments");

    let panics = instance.get_typed_func::<(), ()>(&mut store, "panics").unwrap();

    panics
        .call(&mut store, ())
        .expect_err("guest code should panic");

    Ok(())
}

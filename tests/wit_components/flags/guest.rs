wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "flags-test",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn export_add_green_and_blue(colors: Colors) -> Colors {
        import_add_green(colors).union(Colors::BLUE)
    }

    fn export_push_green_and_blue(colors: Vec<Colors>) -> Vec<Colors> {
        let mut colors = import_push_green(&colors);
        colors.push(Colors::BLUE);
        colors
    }

    fn export_add_first_and_last(values: ManyFlags) -> ManyFlags {
        import_add_first(values).union(ManyFlags::FLAG39)
    }

    fn export_push_first_and_last(values: Vec<ManyFlags>) -> Vec<ManyFlags> {
        let mut values = import_push_first(&values);
        values.push(ManyFlags::FLAG39);
        values
    }
}

export!(GuestImpl);

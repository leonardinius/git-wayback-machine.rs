use rustbox::RustBox;

struct Engine {
    rustbox : RustBox;
}

impl Engine {
    pub fn init() -> Self {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Self { rustbox : rustbox }
    }
}

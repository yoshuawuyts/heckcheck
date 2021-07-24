use heckcheck::{Arbitrary, HeckCheck};

#[test]
fn smoke() {
    #[derive(Clone, Debug, Arbitrary)]
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    let mut checker = HeckCheck::new();
    checker.check(|rgb: Rgb| {
        if rgb.r < rgb.g && rgb.g < rgb.b {
            panic!("error: r < g < b!");
        }
        Ok(())
    });
}

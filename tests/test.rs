use heckcheck::prelude::*;

#[test]
fn smoke() {
    #[derive(Clone, Debug, Arbitrary, PartialEq)]
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Rgb {
        pub fn to_hex(&self) -> String {
            format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
        }
        pub fn from_hex(s: String) -> Self {
            let s = s.strip_prefix('#').unwrap();
            Rgb {
                r: u8::from_str_radix(&s[0..2], 16).unwrap(),
                g: u8::from_str_radix(&s[2..4], 16).unwrap(),
                b: u8::from_str_radix(&s[4..6], 16).unwrap(),
            }
        }
    }

    heckcheck::check(|rgb: Rgb| {
        let hex = rgb.to_hex();
        let res = Rgb::from_hex(hex);
        assert_eq!(rgb, res);
        Ok(())
    });
}

#[test]
#[should_panic]
fn panics_on_bug() {
    #[derive(Arbitrary, Debug, PartialEq)]
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Rgb {
        pub fn to_hex(&self) -> String {
            format!("#{:02X}{:2X}{:02X}", self.r, self.g, self.b)
            // NOTE:         ^ bug is here; should be :02X
        }
        pub fn from_hex(s: String) -> Self {
            let s = s.strip_prefix('#').unwrap();
            Rgb {
                r: u8::from_str_radix(&s[0..2], 16).unwrap(),
                g: u8::from_str_radix(&s[2..4], 16).unwrap(),
                b: u8::from_str_radix(&s[4..6], 16).unwrap(),
            }
        }
    }

    heckcheck::check(|rgb: Rgb| {
        let hex = rgb.to_hex();
        let res = Rgb::from_hex(hex);
        assert_eq!(rgb, res);
        Ok(())
    });
}

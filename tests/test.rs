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
fn shrinker_terminates_when_no_prefix_reproduces() {
    use heckcheck::{Shrink, ShrinkReport, Shrinker};

    let source = vec![1, 2, 3, 4];
    let mut shrinker = Shrinker::shrink(source.clone());
    for _ in 0..64 {
        let _ = shrinker.next();
        if let Some(case) = shrinker.report(ShrinkReport::Pass) {
            assert_eq!(case, &source[..]);
            return;
        }
    }
    panic!("the shrinker did not terminate");
}

#[test]
fn reported_sequence_replays_late_byte_failure() {
    use heckcheck::arbitrary::{Arbitrary, Result, Unstructured};

    /// A value read from the *end* of the buffer, so no prefix of the data
    /// which produced it can reproduce the failure.
    #[derive(Debug)]
    pub struct Tail(usize);

    impl<'a> Arbitrary<'a> for Tail {
        fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
            Ok(Self(u.arbitrary_len::<u8>()?))
        }
    }

    let payload = std::panic::catch_unwind(|| {
        let mut checker = heckcheck::HeckCheck::from_seed(42);
        checker.check(|tail: Tail| {
            assert!(tail.0 < 500);
            Ok(())
        });
    })
    .unwrap_err();

    let msg = payload.downcast_ref::<String>().unwrap();
    let sequence = msg.split('`').nth(1).unwrap().to_owned();

    let replayed = std::panic::catch_unwind(|| {
        heckcheck::replay::<Tail, _>(&sequence, |tail| {
            assert!(tail.0 < 500);
            Ok(())
        });
    });
    assert!(
        replayed.is_err(),
        "the reported sequence did not replay the failure"
    );
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

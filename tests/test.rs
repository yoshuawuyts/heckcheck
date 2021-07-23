use heckcheck::{Arbitrary, HeckCheck};

#[test]
fn smoke() {
    #[derive(Arbitrary, PartialEq, Debug, Clone, Copy)]
    enum Bool {
        True,
        False,
    }

    let mut checker = HeckCheck::new();
    checker.check::<Bool, _>(|b| {
        assert_eq!(*b, Bool::True);
        Ok(())
    });
}

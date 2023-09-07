#![allow(non_snake_case)]

use windows::{core::*, Foundation::*};

static mut COUNTER: isize = 0;

#[implement(IStringable, IClosable)]
struct Test(String, i128);

impl Test {
    fn new(value: &str) -> Self {
        unsafe {
            COUNTER += 1;
        }
        Self(value.to_string(), 0)
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        unsafe {
            COUNTER -= 1;
        }
    }
}

impl IStringable_Impl for Test {
    fn ToString(this: &Self::This) -> Result<HSTRING> {
        Ok(this.0.as_str().into())
    }
}

impl IClosable_Impl for Test {
    fn Close(_this: &Self::This) -> Result<()> {
        Ok(())
    }
}

#[test]
fn identity() -> Result<()> {
    unsafe {
        assert_eq!(COUNTER, 0);
        {
            let a: IStringable = Test::new("test").into_interface();
            assert!(a.ToString()? == "test");

            let b: IClosable = a.cast()?;
            b.Close()?;

            let c: IUnknown = b.cast()?;

            let d: IInspectable = c.cast()?;

            assert!(a == d.cast()?);
        }
        {
            let a: IUnknown = Test::new("test").into_interface();
            let b: IClosable = a.cast()?;
            let c: IStringable = b.cast()?;
            assert!(c.ToString()? == "test");
        }
        {
            let a: IInspectable = Test::new("test").into_interface();
            let b: IStringable = a.cast()?;
            assert!(b.ToString()? == "test");
        }
        {
            let a: IInspectable = Test::new("test").into_interface();
            assert_eq!(a.GetRuntimeClassName()?, "identity.Test");

            let b: IStringable = a.cast()?;
            let c: &IInspectable = b.can_into();
            assert_eq!(c.GetRuntimeClassName()?, "identity.Test");

            let d: IClosable = a.cast()?;
            let e: &IInspectable = d.can_into();
            assert_eq!(e.GetRuntimeClassName()?, "identity.Test");

            let f: IInspectable = e.cast()?;
            assert_eq!(f.GetRuntimeClassName()?, "identity.Test");
        }
        assert_eq!(COUNTER, 0);
        Ok(())
    }
}

use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[implement(IStringable)]
struct Test;

impl IStringable_Impl for Test {
    fn ToString(_this: &Self::This) -> Result<HSTRING> {
        Err(Error::new(E_INVALIDARG, "Test message".into()))
    }
}

#[test]
fn test() {
    let test: IStringable = Test.into_interface();
    let result = test.ToString();
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(error.message(), "Test message");
}

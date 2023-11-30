#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::*;

#[implement(IStringable)]
struct BaseTrust;

impl IStringable_Impl for BaseTrust {
    fn ToString(_this: &Self::This) -> Result<HSTRING> {
        Ok("BaseTrust".into())
    }
}

#[implement(IClosable, TrustLevel = Partial, IStringable)]
struct PartialTrust;

impl IStringable_Impl for PartialTrust {
    fn ToString(_this: &Self::This) -> Result<HSTRING> {
        Ok("PartialTrust".into())
    }
}

impl IClosable_Impl for PartialTrust {
    fn Close(_this: &Self::This) -> Result<()> {
        Ok(())
    }
}

#[implement(IStringable, TrustLevel = Full)]
struct FullTrust;

impl IStringable_Impl for FullTrust {
    fn ToString(_this: &Self::This) -> Result<HSTRING> {
        Ok("FullTrust".into())
    }
}

#[test]
fn test() -> Result<()> {
    let base: IStringable = BaseTrust.into_interface();
    assert_eq!(base.ToString()?, "BaseTrust");
    assert_eq!(base.cast::<IInspectable>()?.GetTrustLevel()?, 0);

    let partial: IStringable = PartialTrust.into_interface();
    assert_eq!(partial.ToString()?, "PartialTrust");
    assert_eq!(partial.cast::<IInspectable>()?.GetTrustLevel()?, 1);

    let full: IStringable = FullTrust.into_interface();
    assert_eq!(full.ToString()?, "FullTrust");
    assert_eq!(full.cast::<IInspectable>()?.GetTrustLevel()?, 2);

    Ok(())
}

#![allow(non_snake_case)]

use windows::core::*;
use windows::Win32::Foundation::{BOOL, S_OK};
use windows::Win32::System::Com::*;

#[implement(IPersistStream)]
struct Test();

impl IPersist_Impl for Test {
    fn GetClassID(_this: &Self::This) -> Result<GUID> {
        Ok(GUID::zeroed())
    }
}

impl IPersistStream_Impl for Test {
    fn IsDirty(_this: &Self::This) -> HRESULT {
        S_OK
    }

    fn Load(_this: &Self::This, _: Option<&IStream>) -> Result<()> {
        Ok(())
    }

    fn Save(_this: &Self::This, _: Option<&IStream>, _: BOOL) -> Result<()> {
        Ok(())
    }

    fn GetSizeMax(_this: &Self::This) -> Result<u64> {
        Ok(0)
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream: IPersistStream = Test().into_interface();
        stream.GetClassID()?; // IPersist
        stream.IsDirty().ok()?; // IPersistStream
        stream.cast::<IPersistStream>()?;
        stream.cast::<IPersist>()?;

        let persist: &IPersist = stream.can_into();
        persist.GetClassID()?;
        persist.cast::<IPersistStream>()?;
        persist.cast::<IPersist>()?;

        Ok(())
    }
}

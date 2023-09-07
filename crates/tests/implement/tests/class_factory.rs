#![allow(non_snake_case)]

use windows::core::*;
use windows::Foundation::*;
use windows::Win32::Foundation::*;
use windows::Win32::System::Com::*;

#[implement(IClosable, IStringable)]
struct Object();

impl IStringable_Impl for Object {
    fn ToString(_this: &Self::This) -> Result<HSTRING> {
        Ok("Object".into())
    }
}

impl IClosable_Impl for Object {
    fn Close(_this: &Self::This) -> Result<()> {
        Ok(())
    }
}

#[implement(IClassFactory)]
struct Factory();

impl IClassFactory_Impl for Factory {
    fn CreateInstance(
        _this: &Self::This,
        outer: Option<&IUnknown>,
        iid: *const GUID,
        object: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        assert!(outer.is_none());
        let unknown: IInspectable = Object().into_interface();
        unsafe { unknown.query(iid, object).ok() }
    }

    fn LockServer(_this: &Self::This, lock: BOOL) -> Result<()> {
        assert!(lock.as_bool());
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let factory: IClassFactory = Factory().into_interface();
        factory.LockServer(true)?;

        let stringable: IStringable = factory.CreateInstance(None)?;
        assert_eq!(stringable.ToString()?, "Object");

        let closable: IClosable = factory.CreateInstance(None)?;
        closable.Close()?;

        Ok(())
    }
}

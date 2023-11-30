#![allow(non_snake_case)]

use windows::{
    core::*, Win32::System::Com::StructuredStorage::*, Win32::System::Com::*,
    Win32::UI::Shell::PropertiesSystem::*,
};

#[implement(IInitializeWithStream, IPropertyStore, IPropertyStoreCapabilities)]
struct Object();

impl IInitializeWithStream_Impl for Object {
    fn Initialize(_this: &Self::This, _: Option<&IStream>, _: u32) -> Result<()> {
        Ok(())
    }
}

impl IPropertyStore_Impl for Object {
    fn GetCount(_this: &Self::This) -> Result<u32> {
        Ok(123)
    }
    fn GetAt(_this: &Self::This, _: u32, _: *mut PROPERTYKEY) -> Result<()> {
        unimplemented!()
    }
    fn GetValue(_this: &Self::This, _: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        unimplemented!()
    }
    fn SetValue(_this: &Self::This, _: *const PROPERTYKEY, _: *const PROPVARIANT) -> Result<()> {
        unimplemented!()
    }
    fn Commit(_this: &Self::This) -> Result<()> {
        unimplemented!()
    }
}

impl IPropertyStoreCapabilities_Impl for Object {
    fn IsPropertyWritable(_this: &Self::This, _: *const PROPERTYKEY) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let a: IInitializeWithStream = Object().into_interface();
        a.Initialize(None, 0)?;

        let b: IPropertyStore = a.cast()?;
        assert!(b.GetCount()? == 123);

        let c: IPropertyStoreCapabilities = b.cast()?;
        c.IsPropertyWritable(&PROPERTYKEY::default())?;

        Ok(())
    }
}

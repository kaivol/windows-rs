mod bindings;
use std::sync::*;
use windows::{core::*, Foundation::*, Win32::Foundation::*, Win32::System::WinRT::*};

#[implement(bindings::IClass)]
struct Class(RwLock<i32>);

impl bindings::IClass_Impl for Class {
    fn Property(this: &Self::This) -> Result<i32> {
        let reader = this.0.read().unwrap();
        Ok(*reader)
    }
    fn SetProperty(this: &Self::This, value: i32) -> Result<()> {
        let mut writer = this.0.write().unwrap();
        *writer = value;
        Ok(())
    }
    fn Flags(_this: &Self::This) -> Result<bindings::Flags> {
        Ok(bindings::Flags::Ok)
    }
    fn Int32Array(
        _this: &Self::This,
        a: &[i32],
        b: &mut [i32],
        c: &mut Array<i32>,
    ) -> Result<Array<i32>> {
        assert_eq!(a.len(), b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn StringArray(
        _this: &Self::This,
        a: &[HSTRING],
        b: &mut [HSTRING],
        c: &mut Array<HSTRING>,
    ) -> Result<Array<HSTRING>> {
        assert_eq!(a.len(), b.len());
        assert!(c.is_empty());
        b.clone_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn Input(
        _this: &Self::This,
        a: Option<&IInspectable>,
        b: Option<&bindings::Class>,
        c: Option<&IStringable>,
        d: Option<&bindings::Callback>,
    ) -> Result<()> {
        let a = a.ok_or_else(|| Error::from(E_INVALIDARG))?;
        let b = b.ok_or_else(|| Error::from(E_INVALIDARG))?;
        let c = c.ok_or_else(|| Error::from(E_INVALIDARG))?;
        let d = d.ok_or_else(|| Error::from(E_INVALIDARG))?;

        let a: IUnknown = a.can_clone_into();
        let b: IUnknown = b.can_clone_into();
        assert_eq!(a, b);
        assert_eq!(c.ToString()?, "client");
        assert_eq!(d.Invoke(123)?, 123);

        Ok(())
    }
}

#[implement(IActivationFactory)]
struct ClassFactory;

impl IActivationFactory_Impl for ClassFactory {
    fn ActivateInstance(_this: &Self::This) -> Result<IInspectable> {
        Ok(Class(RwLock::new(0)).into_interface())
    }
}

#[no_mangle]
unsafe extern "system" fn DllGetActivationFactory(
    name: std::mem::ManuallyDrop<HSTRING>,
    result: *mut *mut std::ffi::c_void,
) -> HRESULT {
    let factory: Option<IActivationFactory> = match (*name).to_string().as_str() {
        "test_component.Class" => Some(ClassFactory.into_interface()),
        _ => None,
    };

    if let Some(factory) = factory {
        *result = std::mem::transmute(factory);
        S_OK
    } else {
        *result = std::ptr::null_mut();
        CLASS_E_CLASSNOTAVAILABLE
    }
}

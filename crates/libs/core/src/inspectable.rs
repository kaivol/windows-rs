use super::*;

/// A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value.
/// [`IInspectable`] represents the
/// [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)
/// interface.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct IInspectable(pub IUnknown);

impl IInspectable {
    pub fn GetIids(&self) -> Result<Array<GUID>> {
        unsafe {
            let mut result__ = std::mem::MaybeUninit::zeroed();
            (Interface::vtable(self).GetIids)(
                Interface::as_raw(self),
                Array::<GUID>::set_abi_len(std::mem::transmute(&mut result__)),
                result__.as_mut_ptr() as *mut _ as _,
            )
            .and_then(|| result__.assume_init())
        }
    }

    /// Returns the canonical type name for the underlying object.
    pub fn GetRuntimeClassName(&self) -> Result<HSTRING> {
        unsafe {
            let mut result__ = std::mem::zeroed();
            (Interface::vtable(self).GetRuntimeClassName)(Interface::as_raw(self), &mut result__)
                .from_abi(result__)
        }
    }

    pub fn GetTrustLevel(&self) -> Result<i32> {
        unsafe {
            let mut result__ = std::mem::zeroed();
            (Interface::vtable(self).GetTrustLevel)(Interface::as_raw(self), &mut result__)
                .from_abi(result__)
        }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct IInspectable_Vtbl {
    pub base: IUnknown_Vtbl,
    pub GetIids: unsafe extern "system" fn(
        this: *mut std::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut GUID,
    ) -> HRESULT,
    pub GetRuntimeClassName: unsafe extern "system" fn(
        this: *mut std::ffi::c_void,
        result__: *mut std::mem::MaybeUninit<HSTRING>,
    ) -> HRESULT,
    pub GetTrustLevel:
        unsafe extern "system" fn(this: *mut std::ffi::c_void, result__: *mut i32) -> HRESULT,
}

unsafe impl Interface for IInspectable {
    type Vtable = IInspectable_Vtbl;
}

unsafe impl ComInterface for IInspectable {
    const IID: GUID = GUID::from_u128(0xaf86e2e0_b12d_4c6a_9c5a_d7aa65101e90);
}

impl CanInto<IUnknown> for IInspectable {}

impl RuntimeType for IInspectable {
    const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice(b"cinterface(IInspectable)");
}

impl RuntimeName for IInspectable {}

impl std::fmt::Debug for IInspectable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Attempts to retrieve the string representation of the object via the
        // IStringable interface. If that fails, it will use the canonical type
        // name to give some idea of what the object represents.
        let name = <Self as ComInterface>::cast::<imp::IStringable>(self)
            .and_then(|s| s.ToString())
            .or_else(|_| self.GetRuntimeClassName())
            .unwrap_or_default();
        write!(f, "\"{}\"", name)
    }
}

macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl std::convert::TryFrom<$t> for IInspectable {
            type Error = Error;
            fn try_from(value: $t) -> Result<Self> {
                imp::PropertyValue::$m(value)
            }
        }
        impl std::convert::TryFrom<IInspectable> for $t {
            type Error = Error;
            fn try_from(value: IInspectable) -> Result<Self> {
                <IInspectable as ComInterface>::cast::<imp::IReference<$t>>(&value)?.Value()
            }
        }
        impl std::convert::TryFrom<&IInspectable> for $t {
            type Error = Error;
            fn try_from(value: &IInspectable) -> Result<Self> {
                <IInspectable as ComInterface>::cast::<imp::IReference<$t>>(value)?.Value()
            }
        })*
    };
}
primitive_boxed_type! {
    (bool, CreateBoolean),
    (u8, CreateUInt8),
    (i16, CreateInt16),
    (u16, CreateUInt16),
    (i32, CreateInt32),
    (u32, CreateUInt32),
    (i64, CreateInt64),
    (u64, CreateUInt64),
    (f32, CreateSingle),
    (f64, CreateDouble)
}
impl std::convert::TryFrom<&str> for IInspectable {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        let value: HSTRING = value.into();
        imp::PropertyValue::CreateString(&value)
    }
}
impl std::convert::TryFrom<HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: HSTRING) -> Result<Self> {
        imp::PropertyValue::CreateString(&value)
    }
}
impl std::convert::TryFrom<&HSTRING> for IInspectable {
    type Error = Error;
    fn try_from(value: &HSTRING) -> Result<Self> {
        imp::PropertyValue::CreateString(value)
    }
}
impl std::convert::TryFrom<IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: IInspectable) -> Result<Self> {
        <IInspectable as ComInterface>::cast::<imp::IReference<HSTRING>>(&value)?.Value()
    }
}
impl std::convert::TryFrom<&IInspectable> for HSTRING {
    type Error = Error;
    fn try_from(value: &IInspectable) -> Result<Self> {
        <IInspectable as ComInterface>::cast::<imp::IReference<HSTRING>>(value)?.Value()
    }
}

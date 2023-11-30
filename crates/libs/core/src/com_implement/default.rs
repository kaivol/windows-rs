use crate::com_implement::WeakRefCount;
use crate::*;
use std::borrow::Cow;
use std::boxed::Box;
use std::collections::HashSet;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::{any, ffi, marker, mem};

mod private {
    use super::*;

    pub struct VtablesValid;

    pub trait InterfaceTuple<Impl: ComObjectImpl> {
        const INTERFACES: &'static [&'static [GUID]];
        type Vtables: Copy;
        const VTABLES: Self::Vtables;
        const VTABLES_VALID: VtablesValid;
    }

    macro_rules! tuple_impl {
        ($($name:ident:$index:tt)+) => {
            impl<
                Impl: ComObjectImpl,
                $($name: Vtable<ComObject<Impl>, $index> + Iids + 'static,)+
            > InterfaceTuple<Impl> for ($($name,)+) {
                const INTERFACES: &'static [&'static [GUID]] = &[$($name::IIDS,)+];

                type Vtables = ($(&'static $name::Vtable,)+);

                const VTABLES_VALID: VtablesValid = {
                    $(let _ = ValidateInterfaces::<Impl, $name>::VALID;)+
                    VtablesValid
                };

                const VTABLES: Self::Vtables = {
                    let _ = Self::VTABLES_VALID;
                    ($($name::VTABLE_REF,)+)
                };
            }
        };
    }

    tuple_impl! { I0:0 }
    tuple_impl! { I0:0 I1:1 }
    tuple_impl! { I0:0 I1:1 I2:2 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 I11:11 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 I11:11 I12:12 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 I11:11 I12:12 I13:13 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 I11:11 I12:12 I13:13 I14:14 }
    tuple_impl! { I0:0 I1:1 I2:2 I3:3 I4:4 I5:5 I6:6 I7:7 I8:8 I9:9 I10:10 I11:11 I12:12 I13:13 I14:14 I15:15}
}
use private::*;

pub trait ComObjectImpl: BaseImpl<This = ComObject<Self>> {
    type Interfaces: InterfaceTuple<Self>;

    fn get_runtime_name() -> Cow<'static, str> {
        any::type_name::<Self>().replace("::", ".").into()
    }

    fn get_trust_level() -> i32 {
        0
    }
}

impl<Impl: ComObjectImpl> BaseImpl for Impl {
    type This = ComObject<Self>;
}

pub trait ComObjectImplExt: ComObjectImpl {
    fn into_object(self) -> ComObject<Self> {
        let boxed = Box::new(ComObjectBox {
            vtables: Self::Interfaces::VTABLES,
            this: self,
            count: WeakRefCount::new(),
        });
        ComObject {
            ptr: NonNull::from(Box::leak(boxed)),
        }
    }

    fn into_interface<Int: ComInterface>(self) -> Int {
        // Validate cast asap
        let _ = InterfaceCast::<Self, Int>::CAST_VALID;
        // Validate required interfaces asap
        let _ = Self::Interfaces::VTABLES_VALID;

        self.into_object().into_interface()
    }
}
impl<Impl: ComObjectImpl> ComObjectImplExt for Impl {}

#[doc(hidden)]
#[repr(C)]
pub struct ComObjectBox<Impl: ComObjectImpl> {
    vtables: <Impl::Interfaces as InterfaceTuple<Impl>>::Vtables,
    this: Impl,
    count: WeakRefCount,
}

pub struct ComObject<Impl: ComObjectImpl> {
    ptr: NonNull<ComObjectBox<Impl>>,
}

impl<Impl: ComObjectImpl> ImplProvider for ComObject<Impl> {
    type Impl = Impl;

    #[inline(always)]
    unsafe fn call_impl<Result, const OFFSET: usize>(
        this: *mut ffi::c_void,
        f: impl FnOnce(&Impl::This) -> Result,
    ) -> Result {
        let com_object = ComObject::from_com_interface_pointer::<OFFSET>(this);
        f(&*com_object)
    }

    unsafe fn query_interface<const OFFSET: usize>(
        this: *mut ffi::c_void,
        iid: &GUID,
    ) -> Option<NonNull<ffi::c_void>> {
        let this = ComObject::<Impl>::from_com_interface_pointer::<OFFSET>(this);

        fn interface_offset<Impl: ComObjectImpl>(iid: &GUID) -> Option<usize> {
            if *iid == imp::IAgileObject::IID {
                return Some(0);
            }
            Impl::Interfaces::INTERFACES
                .iter()
                .enumerate()
                .find_map(|(i, interface)| interface.contains(iid).then(|| i))
        }

        if let Some(offset) = interface_offset::<Impl>(iid) {
            this.add_ref();
            let interface_vtable = this.ptr.cast::<*mut ()>().as_ptr().add(offset).cast();
            NonNull::new(interface_vtable)
        } else if *iid == imp::IWeakReferenceSource::IID {
            let back_reference = ManuallyDrop::new(IUnknown::from_raw(this.ptr.cast().as_ptr()));
            NonNull::new(this.inner().count.query(back_reference).into_raw())
        } else {
            None
        }
    }

    unsafe fn add_ref<const OFFSET: usize>(this: *mut ffi::c_void) -> u32 {
        let this = ComObject::<Impl>::from_com_interface_pointer::<OFFSET>(this);
        this.add_ref()
    }

    unsafe fn release<const OFFSET: usize>(this: *mut ffi::c_void) -> u32 {
        let this = ComObject::<Impl>::from_com_interface_pointer::<OFFSET>(this);
        this.release()
    }

    fn get_iids() -> Array<GUID> {
        let mut iids = HashSet::new();
        iids.insert(imp::IWeakReferenceSource::IID);
        iids.insert(imp::IAgileObject::IID);
        iids.extend(
            Impl::Interfaces::INTERFACES
                .iter()
                .flat_map(|i| i.iter())
                .copied()
                .filter(|i| i != &IUnknown::IID && i != &IInspectable::IID),
        );
        let mut array = Array::with_len(iids.len());
        for (i, iid) in iids.into_iter().enumerate() {
            array[i] = iid;
        }
        array
    }

    fn get_runtime_name() -> HSTRING {
        HSTRING::from(&*Impl::get_runtime_name())
    }

    fn get_trust_level() -> i32 {
        Impl::get_trust_level()
    }
}

impl<Impl: ComObjectImpl> ComObject<Impl> {
    pub fn into_interface<Int: ComInterface>(self) -> Int {
        let _ = InterfaceCast::<Impl, Int>::CAST_VALID;

        let result = unsafe {
            let ptr = self
                .ptr
                .cast::<*mut ()>()
                .as_ptr()
                .add(InterfaceCast::<Impl, Int>::CAST_OFFSET)
                .cast();
            Int::from_raw(ptr)
        };

        mem::forget(self);
        result
    }

    pub fn to_interface<Int: ComInterface>(&self) -> Int {
        let _ = InterfaceCast::<Impl, Int>::CAST_VALID;

        self.clone().into_interface()
    }

    fn inner(&self) -> &ComObjectBox<Impl> {
        unsafe { self.ptr.as_ref() }
    }

    fn add_ref(&self) -> u32 {
        self.inner().count.add_ref()
    }

    unsafe fn release(&self) -> u32 {
        let remaining = self.inner().count.release();
        if remaining == 0 {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
        remaining
    }

    unsafe fn from_com_interface_pointer<const OFFSET: usize>(
        this: *mut ffi::c_void,
    ) -> ManuallyDrop<ComObject<Impl>> {
        let this = this
            .cast::<*mut ()>()
            .sub(OFFSET)
            .cast::<ComObjectBox<Impl>>();
        ManuallyDrop::new(ComObject {
            ptr: NonNull::new_unchecked(this),
        })
    }
}

impl<Impl: ComObjectImpl> Drop for ComObject<Impl> {
    fn drop(&mut self) {
        unsafe {
            Self::release(self);
        }
    }
}

impl<Impl: ComObjectImpl> Clone for ComObject<Impl> {
    fn clone(&self) -> Self {
        self.add_ref();
        Self { ptr: self.ptr }
    }
}

impl<Impl: ComObjectImpl> Deref for ComObject<Impl> {
    type Target = Impl;

    fn deref(&self) -> &Self::Target {
        &self.inner().this
    }
}

struct ValidateInterfaces<Impl, Int> {
    _marker: marker::PhantomData<(Impl, Int)>,
}

impl<Impl: ComObjectImpl, Int: Iids> ValidateInterfaces<Impl, Int> {
    const VALID: VtablesValid = {
        if Int::IID.to_u128() == IUnknown::IID.to_u128() {
            panic!("can not manually implement `IUnknown`");
        }
        if Int::IID.to_u128() == IInspectable::IID.to_u128() {
            panic!("can not manually implement `IInspectable`");
        }
        if Int::IID.to_u128() == imp::IAgileObject::IID.to_u128() {
            panic!("can not manually implement `IAgileObject`");
        }
        if Int::IID.to_u128() == imp::IWeakReferenceSource::IID.to_u128() {
            panic!("can not manually implement `IWeakReferenceSource`");
        }
        if !required_interfaces(Impl::Interfaces::INTERFACES, Int::REQUIRED_IIDS) {
            panic!("not all required interfaces are implemented");
        }
        VtablesValid
    };
}

const fn required_interfaces(interfaces: &[&[GUID]], required_iids: &[GUID]) -> bool {
    let mut required_iids_index = 0;
    'required: while required_iids_index < required_iids.len() {
        let required_iid = required_iids[required_iids_index];

        let mut interface_index = 0;
        while interface_index < interfaces.len() {
            let interface = interfaces[interface_index];
            if contains(interface, required_iid) {
                required_iids_index += 1;
                continue 'required;
            }
            interface_index += 1;
        }
        return false;
    }
    true
}

struct CastValid;

struct InterfaceCast<Impl, Int> {
    _marker: marker::PhantomData<(Impl, Int)>,
}

impl<Impl: ComObjectImpl, Int: ComInterface> InterfaceCast<Impl, Int> {
    const CAST_OFFSET: usize = {
        if let Some(offset) = interface_offset(Impl::Interfaces::INTERFACES, Int::IID) {
            offset
        } else {
            panic!("invalid cast. Interface not implemented");
        }
    };
    const CAST_VALID: CastValid = {
        let _ = Self::CAST_OFFSET;
        CastValid
    };
}

const fn interface_offset(interfaces: &[&[GUID]], query: GUID) -> Option<usize> {
    let mut i = 0;
    while i < interfaces.len() {
        let interface = interfaces[i];
        if contains(interface, query) {
            return Some(i);
        }
        i += 1;
    }
    None
}

const fn contains(iids: &[GUID], iid: GUID) -> bool {
    let mut i = 0;
    while i < iids.len() {
        if iids[i].to_u128() == iid.to_u128() {
            return true;
        }
        i += 1;
    }
    false
}

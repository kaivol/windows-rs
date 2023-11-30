pub trait GdiplusAbort_Impl: Sized {
    fn Abort(&self) -> ::windows_core::Result<()>;
}
impl GdiplusAbort_Vtbl {
    pub const fn new<Impl: GdiplusAbort_Impl>() -> GdiplusAbort_Vtbl {
        unsafe extern "system" fn Abort<Impl: GdiplusAbort_Impl>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::Abort(this).into()
        }
        Self { Abort: Abort::<Impl> }
    }
}
#[doc(hidden)]
struct GdiplusAbort_ImplVtbl<T: GdiplusAbort_Impl>(::std::marker::PhantomData<T>);
impl<T: GdiplusAbort_Impl> GdiplusAbort_ImplVtbl<T> {
    const VTABLE: GdiplusAbort_Vtbl = GdiplusAbort_Vtbl::new::<T>();
}
impl GdiplusAbort {
    pub fn new<'a, T: GdiplusAbort_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &GdiplusAbort_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait IImageBytes_Impl: ::windows_core::BaseImpl {
    fn CountBytes(this: &Self::This, pcb: *mut u32) -> ::windows_core::Result<()>;
    fn LockBytes(this: &Self::This, cb: u32, uloffset: u32, ppvbytes: *const *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnlockBytes(this: &Self::This, pvbytes: *const ::core::ffi::c_void, cb: u32, uloffset: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IImageBytes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageBytes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CountBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CountBytes(this, ::core::mem::transmute_copy(&pcb)).into())
        }
        unsafe extern "system" fn LockBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: u32, uloffset: u32, ppvbytes: *const *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockBytes(this, ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&ppvbytes)).into())
        }
        unsafe extern "system" fn UnlockBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbytes: *const ::core::ffi::c_void, cb: u32, uloffset: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockBytes(this, ::core::mem::transmute_copy(&pvbytes), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&uloffset)).into())
        }
        IImageBytes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CountBytes: CountBytes::<Identity, Impl, OFFSET>,
            LockBytes: LockBytes::<Identity, Impl, OFFSET>,
            UnlockBytes: UnlockBytes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}

pub trait ID3DBlob_Impl: ::windows_core::BaseImpl {
    fn GetBufferPointer(this: &Self::This) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(this: &Self::This) -> usize;
}
impl ::windows_core::Iids for ID3DBlob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DBlob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferPointer(this))
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DBlob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferSize(this))
        }
        ID3DBlob_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3DDestructionNotifier_Impl: ::windows_core::BaseImpl {
    fn RegisterDestructionCallback(this: &Self::This, callbackfn: PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<u32>;
    fn UnregisterDestructionCallback(this: &Self::This, callbackid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID3DDestructionNotifier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID3DDestructionNotifier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterDestructionCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackfn: PFN_DESTRUCTION_CALLBACK, pdata: *const ::core::ffi::c_void, pcallbackid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterDestructionCallback(this, ::core::mem::transmute_copy(&callbackfn), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallbackid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterDestructionCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID3DDestructionNotifier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callbackid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterDestructionCallback(this, ::core::mem::transmute_copy(&callbackid)).into())
        }
        ID3DDestructionNotifier_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterDestructionCallback: RegisterDestructionCallback::<Identity, Impl, OFFSET>,
            UnregisterDestructionCallback: UnregisterDestructionCallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ID3DInclude_Impl: Sized {
    fn Open(&self, includetype: D3D_INCLUDE_TYPE, pfilename: &::windows_core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows_core::Result<()>;
    fn Close(&self, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ID3DInclude_Vtbl {
    pub const fn new<Impl: ID3DInclude_Impl>() -> ID3DInclude_Vtbl {
        unsafe extern "system" fn Open<Impl: ID3DInclude_Impl>(this: *mut ::core::ffi::c_void, includetype: D3D_INCLUDE_TYPE, pfilename: ::windows_core::PCSTR, pparentdata: *const ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pbytes: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::Open(this, ::core::mem::transmute_copy(&includetype), ::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&pparentdata), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pbytes)).into()
        }
        unsafe extern "system" fn Close<Impl: ID3DInclude_Impl>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::Close(this, ::core::mem::transmute_copy(&pdata)).into()
        }
        Self { Open: Open::<Impl>, Close: Close::<Impl> }
    }
}
#[doc(hidden)]
struct ID3DInclude_ImplVtbl<T: ID3DInclude_Impl>(::std::marker::PhantomData<T>);
impl<T: ID3DInclude_Impl> ID3DInclude_ImplVtbl<T> {
    const VTABLE: ID3DInclude_Vtbl = ID3DInclude_Vtbl::new::<T>();
}
impl ID3DInclude {
    pub fn new<'a, T: ID3DInclude_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &ID3DInclude_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}

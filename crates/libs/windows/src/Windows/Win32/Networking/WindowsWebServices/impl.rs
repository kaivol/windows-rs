pub trait IContentPrefetcherTaskTrigger_Impl: ::windows_core::BaseImpl {
    fn TriggerContentPrefetcherTask(this: &Self::This, packagefullname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsRegisteredForContentPrefetch(this: &Self::This, packagefullname: &::windows_core::PCWSTR) -> ::windows_core::Result<u8>;
}
impl ::windows_core::Iids for IContentPrefetcherTaskTrigger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContentPrefetcherTaskTrigger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TriggerContentPrefetcherTask(this, ::core::mem::transmute(&packagefullname)).into())
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContentPrefetcherTaskTrigger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: ::windows_core::PCWSTR, isregistered: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRegisteredForContentPrefetch(this, ::core::mem::transmute(&packagefullname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isregistered, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContentPrefetcherTaskTrigger_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TriggerContentPrefetcherTask: TriggerContentPrefetcherTask::<Identity, Impl, OFFSET>,
            IsRegisteredForContentPrefetch: IsRegisteredForContentPrefetch::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}

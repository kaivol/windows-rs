pub trait IAsyncAction_Impl: ::windows_core::BaseImpl + IAsyncInfo_Impl {
    fn SetCompleted(this: &Self::This, handler: ::core::option::Option<&AsyncActionCompletedHandler>) -> ::windows_core::Result<()>;
    fn Completed(this: &Self::This) -> ::windows_core::Result<AsyncActionCompletedHandler>;
    fn GetResults(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAsyncAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAsyncInfo as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompleted(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Completed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Completed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResults(this).into())
        }
        IAsyncAction_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCompleted: SetCompleted::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            GetResults: GetResults::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAsyncActionWithProgress_Impl<TProgress>: ::windows_core::BaseImpl + IAsyncInfo_Impl
where
    TProgress: ::windows_core::RuntimeType + 'static,
{
    fn SetProgress(this: &Self::This, handler: ::core::option::Option<&AsyncActionProgressHandler<TProgress>>) -> ::windows_core::Result<()>;
    fn Progress(this: &Self::This) -> ::windows_core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(this: &Self::This, handler: ::core::option::Option<&AsyncActionWithProgressCompletedHandler<TProgress>>) -> ::windows_core::Result<()>;
    fn Completed(this: &Self::This) -> ::windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(this: &Self::This) -> ::windows_core::Result<()>;
}
impl<TProgress: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IAsyncActionWithProgress<TProgress> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAsyncInfo as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize, TProgress: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IAsyncActionWithProgress<TProgress> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProgress<TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProgress(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Progress<TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompleted<TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompleted(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Completed<TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Completed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResults<TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResults(this).into())
        }
        IAsyncActionWithProgress_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProgress: SetProgress::<TProgress, Identity, Impl, OFFSET>,
            Progress: Progress::<TProgress, Identity, Impl, OFFSET>,
            SetCompleted: SetCompleted::<TProgress, Identity, Impl, OFFSET>,
            Completed: Completed::<TProgress, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TProgress, Identity, Impl, OFFSET>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    };
}
pub trait IAsyncInfo_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<AsyncStatus>;
    fn ErrorCode(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAsyncInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAsyncInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AsyncStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ErrorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ErrorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IAsyncInfo_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ErrorCode: ErrorCode::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAsyncOperation_Impl<TResult>: ::windows_core::BaseImpl + IAsyncInfo_Impl
where
    TResult: ::windows_core::RuntimeType + 'static,
{
    fn SetCompleted(this: &Self::This, handler: ::core::option::Option<&AsyncOperationCompletedHandler<TResult>>) -> ::windows_core::Result<()>;
    fn Completed(this: &Self::This) -> ::windows_core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(this: &Self::This) -> ::windows_core::Result<TResult>;
}
impl<TResult: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IAsyncOperation<TResult> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAsyncInfo as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: usize, TResult: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IAsyncOperation<TResult> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCompleted<TResult: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompleted(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Completed<TResult: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Completed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResults<TResult: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<TResult>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAsyncOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCompleted: SetCompleted::<TResult, Identity, Impl, OFFSET>,
            Completed: Completed::<TResult, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TResult, Identity, Impl, OFFSET>,
            TResult: ::core::marker::PhantomData::<TResult>,
        }
    };
}
pub trait IAsyncOperationWithProgress_Impl<TResult, TProgress>: ::windows_core::BaseImpl + IAsyncInfo_Impl
where
    TResult: ::windows_core::RuntimeType + 'static,
    TProgress: ::windows_core::RuntimeType + 'static,
{
    fn SetProgress(this: &Self::This, handler: ::core::option::Option<&AsyncOperationProgressHandler<TResult, TProgress>>) -> ::windows_core::Result<()>;
    fn Progress(this: &Self::This) -> ::windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(this: &Self::This, handler: ::core::option::Option<&AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> ::windows_core::Result<()>;
    fn Completed(this: &Self::This) -> ::windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(this: &Self::This) -> ::windows_core::Result<TResult>;
}
impl<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IAsyncOperationWithProgress<TResult, TProgress> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IAsyncInfo as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize, TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IAsyncOperationWithProgress<TResult, TProgress> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProgress<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProgress(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Progress<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Progress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompleted<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompleted(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn Completed<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Completed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResults<TResult: ::windows_core::RuntimeType + 'static, TProgress: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<TResult>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAsyncOperationWithProgress_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProgress: SetProgress::<TResult, TProgress, Identity, Impl, OFFSET>,
            Progress: Progress::<TResult, TProgress, Identity, Impl, OFFSET>,
            SetCompleted: SetCompleted::<TResult, TProgress, Identity, Impl, OFFSET>,
            Completed: Completed::<TResult, TProgress, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TResult, TProgress, Identity, Impl, OFFSET>,
            TResult: ::core::marker::PhantomData::<TResult>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    };
}
pub trait IClosable_Impl: ::windows_core::BaseImpl {
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClosable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClosable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClosable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClosable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IClosable_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Close: Close::<Identity, Impl, OFFSET> }
    };
}
pub trait IGetActivationFactory_Impl: ::windows_core::BaseImpl {
    fn GetActivationFactory(this: &Self::This, activatableclassid: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::Iids for IGetActivationFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetActivationFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetActivationFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActivationFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetActivationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activatableclassid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivationFactory(this, ::core::mem::transmute(&activatableclassid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetActivationFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActivationFactory: GetActivationFactory::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMemoryBuffer_Impl: ::windows_core::BaseImpl + IClosable_Impl {
    fn CreateReference(this: &Self::This) -> ::windows_core::Result<IMemoryBufferReference>;
}
impl ::windows_core::Iids for IMemoryBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IClosable as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMemoryBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMemoryBuffer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateReference: CreateReference::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMemoryBufferReference_Impl: ::windows_core::BaseImpl + IClosable_Impl {
    fn Capacity(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Closed(this: &Self::This, handler: ::core::option::Option<&TypedEventHandler<IMemoryBufferReference, ::windows_core::IInspectable>>) -> ::windows_core::Result<EventRegistrationToken>;
    fn RemoveClosed(this: &Self::This, cookie: &EventRegistrationToken) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMemoryBufferReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IClosable as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMemoryBufferReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Capacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capacity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Closed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Closed(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveClosed(this, ::core::mem::transmute(&cookie)).into())
        }
        IMemoryBufferReference_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Capacity: Capacity::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertyValue_Impl: ::windows_core::BaseImpl {
    fn Type(this: &Self::This) -> ::windows_core::Result<PropertyType>;
    fn IsNumericScalar(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetUInt8(this: &Self::This) -> ::windows_core::Result<u8>;
    fn GetInt16(this: &Self::This) -> ::windows_core::Result<i16>;
    fn GetUInt16(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetInt32(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetUInt32(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInt64(this: &Self::This) -> ::windows_core::Result<i64>;
    fn GetUInt64(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetSingle(this: &Self::This) -> ::windows_core::Result<f32>;
    fn GetDouble(this: &Self::This) -> ::windows_core::Result<f64>;
    fn GetChar16(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetBoolean(this: &Self::This) -> ::windows_core::Result<bool>;
    fn GetString(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn GetGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDateTime(this: &Self::This) -> ::windows_core::Result<DateTime>;
    fn GetTimeSpan(this: &Self::This) -> ::windows_core::Result<TimeSpan>;
    fn GetPoint(this: &Self::This) -> ::windows_core::Result<Point>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<Size>;
    fn GetRect(this: &Self::This) -> ::windows_core::Result<Rect>;
    fn GetUInt8Array(this: &Self::This, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()>;
    fn GetInt16Array(this: &Self::This, value: &mut ::windows_core::Array<i16>) -> ::windows_core::Result<()>;
    fn GetUInt16Array(this: &Self::This, value: &mut ::windows_core::Array<u16>) -> ::windows_core::Result<()>;
    fn GetInt32Array(this: &Self::This, value: &mut ::windows_core::Array<i32>) -> ::windows_core::Result<()>;
    fn GetUInt32Array(this: &Self::This, value: &mut ::windows_core::Array<u32>) -> ::windows_core::Result<()>;
    fn GetInt64Array(this: &Self::This, value: &mut ::windows_core::Array<i64>) -> ::windows_core::Result<()>;
    fn GetUInt64Array(this: &Self::This, value: &mut ::windows_core::Array<u64>) -> ::windows_core::Result<()>;
    fn GetSingleArray(this: &Self::This, value: &mut ::windows_core::Array<f32>) -> ::windows_core::Result<()>;
    fn GetDoubleArray(this: &Self::This, value: &mut ::windows_core::Array<f64>) -> ::windows_core::Result<()>;
    fn GetChar16Array(this: &Self::This, value: &mut ::windows_core::Array<u16>) -> ::windows_core::Result<()>;
    fn GetBooleanArray(this: &Self::This, value: &mut ::windows_core::Array<bool>) -> ::windows_core::Result<()>;
    fn GetStringArray(this: &Self::This, value: &mut ::windows_core::Array<::windows_core::HSTRING>) -> ::windows_core::Result<()>;
    fn GetInspectableArray(this: &Self::This, value: &mut ::windows_core::Array<::windows_core::IInspectable>) -> ::windows_core::Result<()>;
    fn GetGuidArray(this: &Self::This, value: &mut ::windows_core::Array<::windows_core::GUID>) -> ::windows_core::Result<()>;
    fn GetDateTimeArray(this: &Self::This, value: &mut ::windows_core::Array<DateTime>) -> ::windows_core::Result<()>;
    fn GetTimeSpanArray(this: &Self::This, value: &mut ::windows_core::Array<TimeSpan>) -> ::windows_core::Result<()>;
    fn GetPointArray(this: &Self::This, value: &mut ::windows_core::Array<Point>) -> ::windows_core::Result<()>;
    fn GetSizeArray(this: &Self::This, value: &mut ::windows_core::Array<Size>) -> ::windows_core::Result<()>;
    fn GetRectArray(this: &Self::This, value: &mut ::windows_core::Array<Rect>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsNumericScalar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsNumericScalar(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUInt8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUInt8(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInt16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUInt16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUInt16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInt32(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUInt32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUInt32(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInt64(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUInt64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUInt64(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSingle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSingle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDouble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDouble(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChar16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChar16(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBoolean<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoolean(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTimeSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimeSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUInt8Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUInt8Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetInt16Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInt16Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetUInt16Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUInt16Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetInt32Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInt32Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetUInt32Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUInt32Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetInt64Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInt64Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetUInt64Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUInt64Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetSingleArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSingleArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetDoubleArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDoubleArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetChar16Array<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChar16Array(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetBooleanArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBooleanArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetStringArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetInspectableArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInspectableArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetGuidArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGuidArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetDateTimeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDateTimeArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetTimeSpanArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTimeSpanArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetPointArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPointArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetSizeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSizeArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        unsafe extern "system" fn GetRectArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRectArray(this, ::windows_core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into())
        }
        IPropertyValue_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            IsNumericScalar: IsNumericScalar::<Identity, Impl, OFFSET>,
            GetUInt8: GetUInt8::<Identity, Impl, OFFSET>,
            GetInt16: GetInt16::<Identity, Impl, OFFSET>,
            GetUInt16: GetUInt16::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            GetUInt32: GetUInt32::<Identity, Impl, OFFSET>,
            GetInt64: GetInt64::<Identity, Impl, OFFSET>,
            GetUInt64: GetUInt64::<Identity, Impl, OFFSET>,
            GetSingle: GetSingle::<Identity, Impl, OFFSET>,
            GetDouble: GetDouble::<Identity, Impl, OFFSET>,
            GetChar16: GetChar16::<Identity, Impl, OFFSET>,
            GetBoolean: GetBoolean::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetDateTime: GetDateTime::<Identity, Impl, OFFSET>,
            GetTimeSpan: GetTimeSpan::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetUInt8Array: GetUInt8Array::<Identity, Impl, OFFSET>,
            GetInt16Array: GetInt16Array::<Identity, Impl, OFFSET>,
            GetUInt16Array: GetUInt16Array::<Identity, Impl, OFFSET>,
            GetInt32Array: GetInt32Array::<Identity, Impl, OFFSET>,
            GetUInt32Array: GetUInt32Array::<Identity, Impl, OFFSET>,
            GetInt64Array: GetInt64Array::<Identity, Impl, OFFSET>,
            GetUInt64Array: GetUInt64Array::<Identity, Impl, OFFSET>,
            GetSingleArray: GetSingleArray::<Identity, Impl, OFFSET>,
            GetDoubleArray: GetDoubleArray::<Identity, Impl, OFFSET>,
            GetChar16Array: GetChar16Array::<Identity, Impl, OFFSET>,
            GetBooleanArray: GetBooleanArray::<Identity, Impl, OFFSET>,
            GetStringArray: GetStringArray::<Identity, Impl, OFFSET>,
            GetInspectableArray: GetInspectableArray::<Identity, Impl, OFFSET>,
            GetGuidArray: GetGuidArray::<Identity, Impl, OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Identity, Impl, OFFSET>,
            GetTimeSpanArray: GetTimeSpanArray::<Identity, Impl, OFFSET>,
            GetPointArray: GetPointArray::<Identity, Impl, OFFSET>,
            GetSizeArray: GetSizeArray::<Identity, Impl, OFFSET>,
            GetRectArray: GetRectArray::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IReference_Impl<T>: ::windows_core::BaseImpl + IPropertyValue_Impl
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn Value(this: &Self::This) -> ::windows_core::Result<T>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IReference<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPropertyValue as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReference_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IReference<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReference_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IReference_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
}
pub trait IReferenceArray_Impl<T>: ::windows_core::BaseImpl + IPropertyValue_Impl
where
    T: ::windows_core::RuntimeType + 'static,
{
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::Array<T>>;
}
impl<T: ::windows_core::RuntimeType + 'static> ::windows_core::Iids for IReferenceArray<T> {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IPropertyValue as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceArray_Impl<T>, const OFFSET: usize, T: ::windows_core::RuntimeType + 'static> ::windows_core::Vtable<Identity, OFFSET> for IReferenceArray<T> {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<T: ::windows_core::RuntimeType + 'static, Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReferenceArray_Impl<T>, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::windows_core::AbiType<T>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IReferenceArray_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    };
}
pub trait IStringable_Impl: ::windows_core::BaseImpl {
    fn ToString(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IStringable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStringable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStringable_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ToString: ToString::<Identity, Impl, OFFSET> }
    };
}
pub trait IWwwFormUrlDecoderEntry_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IWwwFormUrlDecoderEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWwwFormUrlDecoderEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWwwFormUrlDecoderEntry_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    };
}

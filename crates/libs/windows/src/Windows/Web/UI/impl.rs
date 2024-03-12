#[doc = "Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`, `\"UI\"`, `\"Web_Http\"`"]
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
pub trait IWebViewControl_Impl: ::windows_core::BaseImpl {
    fn Source(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn SetSource(this: &Self::This, source: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn DocumentTitle(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn CanGoBack(this: &Self::This) -> ::windows_core::Result<bool>;
    fn CanGoForward(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetDefaultBackgroundColor(this: &Self::This, value: &super::super::UI::Color) -> ::windows_core::Result<()>;
    fn DefaultBackgroundColor(this: &Self::This) -> ::windows_core::Result<super::super::UI::Color>;
    fn ContainsFullScreenElement(this: &Self::This) -> ::windows_core::Result<bool>;
    fn Settings(this: &Self::This) -> ::windows_core::Result<WebViewControlSettings>;
    fn DeferredPermissionRequests(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<WebViewControlDeferredPermissionRequest>>;
    fn GoForward(this: &Self::This) -> ::windows_core::Result<()>;
    fn GoBack(this: &Self::This) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Navigate(this: &Self::This, source: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn NavigateToString(this: &Self::This, text: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn NavigateToLocalStreamUri(this: &Self::This, source: ::core::option::Option<&super::super::Foundation::Uri>, streamresolver: ::core::option::Option<&super::IUriToStreamResolver>) -> ::windows_core::Result<()>;
    fn NavigateWithHttpRequestMessage(this: &Self::This, requestmessage: ::core::option::Option<&super::Http::HttpRequestMessage>) -> ::windows_core::Result<()>;
    fn InvokeScriptAsync(this: &Self::This, scriptname: &::windows_core::HSTRING, arguments: ::core::option::Option<&super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>;
    fn CapturePreviewToStreamAsync(this: &Self::This, stream: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>;
    fn CaptureSelectedContentToDataPackageAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>;
    fn BuildLocalStreamUri(this: &Self::This, contentidentifier: &::windows_core::HSTRING, relativepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn GetDeferredPermissionRequestById(this: &Self::This, id: u32, result: &mut ::core::option::Option<WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()>;
    fn NavigationStarting(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationStarting(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ContentLoading(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentLoading(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn DOMContentLoaded(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDOMContentLoaded(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn NavigationCompleted(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNavigationCompleted(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameNavigationStarting(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationStartingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationStarting(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameContentLoading(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlContentLoadingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameContentLoading(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameDOMContentLoaded(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlDOMContentLoadedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameDOMContentLoaded(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn FrameNavigationCompleted(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNavigationCompletedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameNavigationCompleted(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ScriptNotify(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlScriptNotifyEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveScriptNotify(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn LongRunningScriptDetected(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlLongRunningScriptDetectedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLongRunningScriptDetected(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnsafeContentWarningDisplaying(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsafeContentWarningDisplaying(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnviewableContentIdentified(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnviewableContentIdentifiedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnviewableContentIdentified(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PermissionRequested(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlPermissionRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePermissionRequested(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn UnsupportedUriSchemeIdentified(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnsupportedUriSchemeIdentified(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn NewWindowRequested(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlNewWindowRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNewWindowRequested(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn ContainsFullScreenElementChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContainsFullScreenElementChanged(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn WebResourceRequested(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<IWebViewControl, WebViewControlWebResourceRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveWebResourceRequested(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl ::windows_core::Iids for IWebViewControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "UI", feature = "Web_Http"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebViewControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Source<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Source(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSource(this, ::windows_core::from_raw_borrowed(&source)).into())
        }
        unsafe extern "system" fn DocumentTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentTitle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanGoBack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanGoBack(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanGoForward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanGoForward(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultBackgroundColor(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn DefaultBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultBackgroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContainsFullScreenElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContainsFullScreenElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeferredPermissionRequests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeferredPermissionRequests(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GoForward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GoForward(this).into())
        }
        unsafe extern "system" fn GoBack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GoBack(this).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Navigate(this, ::windows_core::from_raw_borrowed(&source)).into())
        }
        unsafe extern "system" fn NavigateToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NavigateToString(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn NavigateToLocalStreamUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, streamresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NavigateToLocalStreamUri(this, ::windows_core::from_raw_borrowed(&source), ::windows_core::from_raw_borrowed(&streamresolver)).into())
        }
        unsafe extern "system" fn NavigateWithHttpRequestMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestmessage: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NavigateWithHttpRequestMessage(this, ::windows_core::from_raw_borrowed(&requestmessage)).into())
        }
        unsafe extern "system" fn InvokeScriptAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, arguments: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InvokeScriptAsync(this, ::core::mem::transmute(&scriptname), ::windows_core::from_raw_borrowed(&arguments)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CapturePreviewToStreamAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CapturePreviewToStreamAsync(this, ::windows_core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CaptureSelectedContentToDataPackageAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CaptureSelectedContentToDataPackageAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BuildLocalStreamUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentidentifier: ::std::mem::MaybeUninit<::windows_core::HSTRING>, relativepath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BuildLocalStreamUri(this, ::core::mem::transmute(&contentidentifier), ::core::mem::transmute(&relativepath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeferredPermissionRequestById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: u32, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeferredPermissionRequestById(this, id, ::core::mem::transmute_copy(&result)).into())
        }
        unsafe extern "system" fn NavigationStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NavigationStarting(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveNavigationStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNavigationStarting(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn ContentLoading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentLoading(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveContentLoading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveContentLoading(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn DOMContentLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DOMContentLoaded(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveDOMContentLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDOMContentLoaded(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn NavigationCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NavigationCompleted(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveNavigationCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNavigationCompleted(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn FrameNavigationStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FrameNavigationStarting(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFrameNavigationStarting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFrameNavigationStarting(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn FrameContentLoading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FrameContentLoading(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFrameContentLoading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFrameContentLoading(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn FrameDOMContentLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FrameDOMContentLoaded(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFrameDOMContentLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFrameDOMContentLoaded(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn FrameNavigationCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FrameNavigationCompleted(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFrameNavigationCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFrameNavigationCompleted(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn ScriptNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScriptNotify(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveScriptNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveScriptNotify(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn LongRunningScriptDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LongRunningScriptDetected(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveLongRunningScriptDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLongRunningScriptDetected(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn UnsafeContentWarningDisplaying<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnsafeContentWarningDisplaying(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveUnsafeContentWarningDisplaying<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveUnsafeContentWarningDisplaying(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn UnviewableContentIdentified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnviewableContentIdentified(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveUnviewableContentIdentified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveUnviewableContentIdentified(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn PermissionRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermissionRequested(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePermissionRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePermissionRequested(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn UnsupportedUriSchemeIdentified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnsupportedUriSchemeIdentified(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveUnsupportedUriSchemeIdentified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveUnsupportedUriSchemeIdentified(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn NewWindowRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NewWindowRequested(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveNewWindowRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNewWindowRequested(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn ContainsFullScreenElementChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContainsFullScreenElementChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveContainsFullScreenElementChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveContainsFullScreenElementChanged(this, ::core::mem::transmute(&token)).into())
        }
        unsafe extern "system" fn WebResourceRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WebResourceRequested(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveWebResourceRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveWebResourceRequested(this, ::core::mem::transmute(&token)).into())
        }
        IWebViewControl_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Source: Source::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
            DocumentTitle: DocumentTitle::<Identity, Impl, OFFSET>,
            CanGoBack: CanGoBack::<Identity, Impl, OFFSET>,
            CanGoForward: CanGoForward::<Identity, Impl, OFFSET>,
            SetDefaultBackgroundColor: SetDefaultBackgroundColor::<Identity, Impl, OFFSET>,
            DefaultBackgroundColor: DefaultBackgroundColor::<Identity, Impl, OFFSET>,
            ContainsFullScreenElement: ContainsFullScreenElement::<Identity, Impl, OFFSET>,
            Settings: Settings::<Identity, Impl, OFFSET>,
            DeferredPermissionRequests: DeferredPermissionRequests::<Identity, Impl, OFFSET>,
            GoForward: GoForward::<Identity, Impl, OFFSET>,
            GoBack: GoBack::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            NavigateToString: NavigateToString::<Identity, Impl, OFFSET>,
            NavigateToLocalStreamUri: NavigateToLocalStreamUri::<Identity, Impl, OFFSET>,
            NavigateWithHttpRequestMessage: NavigateWithHttpRequestMessage::<Identity, Impl, OFFSET>,
            InvokeScriptAsync: InvokeScriptAsync::<Identity, Impl, OFFSET>,
            CapturePreviewToStreamAsync: CapturePreviewToStreamAsync::<Identity, Impl, OFFSET>,
            CaptureSelectedContentToDataPackageAsync: CaptureSelectedContentToDataPackageAsync::<Identity, Impl, OFFSET>,
            BuildLocalStreamUri: BuildLocalStreamUri::<Identity, Impl, OFFSET>,
            GetDeferredPermissionRequestById: GetDeferredPermissionRequestById::<Identity, Impl, OFFSET>,
            NavigationStarting: NavigationStarting::<Identity, Impl, OFFSET>,
            RemoveNavigationStarting: RemoveNavigationStarting::<Identity, Impl, OFFSET>,
            ContentLoading: ContentLoading::<Identity, Impl, OFFSET>,
            RemoveContentLoading: RemoveContentLoading::<Identity, Impl, OFFSET>,
            DOMContentLoaded: DOMContentLoaded::<Identity, Impl, OFFSET>,
            RemoveDOMContentLoaded: RemoveDOMContentLoaded::<Identity, Impl, OFFSET>,
            NavigationCompleted: NavigationCompleted::<Identity, Impl, OFFSET>,
            RemoveNavigationCompleted: RemoveNavigationCompleted::<Identity, Impl, OFFSET>,
            FrameNavigationStarting: FrameNavigationStarting::<Identity, Impl, OFFSET>,
            RemoveFrameNavigationStarting: RemoveFrameNavigationStarting::<Identity, Impl, OFFSET>,
            FrameContentLoading: FrameContentLoading::<Identity, Impl, OFFSET>,
            RemoveFrameContentLoading: RemoveFrameContentLoading::<Identity, Impl, OFFSET>,
            FrameDOMContentLoaded: FrameDOMContentLoaded::<Identity, Impl, OFFSET>,
            RemoveFrameDOMContentLoaded: RemoveFrameDOMContentLoaded::<Identity, Impl, OFFSET>,
            FrameNavigationCompleted: FrameNavigationCompleted::<Identity, Impl, OFFSET>,
            RemoveFrameNavigationCompleted: RemoveFrameNavigationCompleted::<Identity, Impl, OFFSET>,
            ScriptNotify: ScriptNotify::<Identity, Impl, OFFSET>,
            RemoveScriptNotify: RemoveScriptNotify::<Identity, Impl, OFFSET>,
            LongRunningScriptDetected: LongRunningScriptDetected::<Identity, Impl, OFFSET>,
            RemoveLongRunningScriptDetected: RemoveLongRunningScriptDetected::<Identity, Impl, OFFSET>,
            UnsafeContentWarningDisplaying: UnsafeContentWarningDisplaying::<Identity, Impl, OFFSET>,
            RemoveUnsafeContentWarningDisplaying: RemoveUnsafeContentWarningDisplaying::<Identity, Impl, OFFSET>,
            UnviewableContentIdentified: UnviewableContentIdentified::<Identity, Impl, OFFSET>,
            RemoveUnviewableContentIdentified: RemoveUnviewableContentIdentified::<Identity, Impl, OFFSET>,
            PermissionRequested: PermissionRequested::<Identity, Impl, OFFSET>,
            RemovePermissionRequested: RemovePermissionRequested::<Identity, Impl, OFFSET>,
            UnsupportedUriSchemeIdentified: UnsupportedUriSchemeIdentified::<Identity, Impl, OFFSET>,
            RemoveUnsupportedUriSchemeIdentified: RemoveUnsupportedUriSchemeIdentified::<Identity, Impl, OFFSET>,
            NewWindowRequested: NewWindowRequested::<Identity, Impl, OFFSET>,
            RemoveNewWindowRequested: RemoveNewWindowRequested::<Identity, Impl, OFFSET>,
            ContainsFullScreenElementChanged: ContainsFullScreenElementChanged::<Identity, Impl, OFFSET>,
            RemoveContainsFullScreenElementChanged: RemoveContainsFullScreenElementChanged::<Identity, Impl, OFFSET>,
            WebResourceRequested: WebResourceRequested::<Identity, Impl, OFFSET>,
            RemoveWebResourceRequested: RemoveWebResourceRequested::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebViewControl2_Impl: ::windows_core::BaseImpl {
    fn AddInitializeScript(this: &Self::This, script: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWebViewControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebViewControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddInitializeScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebViewControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, script: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddInitializeScript(this, ::core::mem::transmute(&script)).into())
        }
        IWebViewControl2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddInitializeScript: AddInitializeScript::<Identity, Impl, OFFSET>,
        }
    };
}

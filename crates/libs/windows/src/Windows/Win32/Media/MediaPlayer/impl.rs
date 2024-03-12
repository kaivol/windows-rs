#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeed_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Xml(this: &Self::This, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Rename(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Url(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetUrl(this: &Self::This, feedurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(this: &Self::This, newparentpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn LastWriteTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Download(this: &Self::This) -> ::windows_core::Result<()>;
    fn AsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelAsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn SyncSetting(this: &Self::This) -> ::windows_core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(this: &Self::This, syncsetting: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()>;
    fn Interval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetInterval(this: &Self::This, minutes: i32) -> ::windows_core::Result<()>;
    fn LastDownloadTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LocalEnclosurePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Items(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetItem(this: &Self::This, itemid: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Link(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Image(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastBuildDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn PubDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Ttl(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Copyright(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MaxItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxItemCount(this: &Self::This, count: i32) -> ::windows_core::Result<()>;
    fn DownloadEnclosuresAutomatically(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDownloadEnclosuresAutomatically(this: &Self::This, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DownloadStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(this: &Self::This, feedxml: &::windows_core::BSTR, feedurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsList(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MarkAllItemsRead(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetWatcher(this: &Self::This, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn UnreadItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeed {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeed {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: i32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUrl(this, ::core::mem::transmute(&feedurl)).into())
        }
        unsafe extern "system" fn LocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute(&newparentpath)).into())
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastwrite: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWriteTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwrite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this).into())
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncDownload(this).into())
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncDownload(this).into())
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncsetting: *mut FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncSetting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(syncsetting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, syncsetting: FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncSetting(this, ::core::mem::transmute_copy(&syncsetting)).into())
        }
        unsafe extern "system" fn Interval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Interval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterval(this, ::core::mem::transmute_copy(&minutes)).into())
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalEnclosurePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Items<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Items(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this, ::core::mem::transmute_copy(&itemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, homepage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(homepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Image(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastbuilddate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastBuildDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastbuilddate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PubDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastpopulatedate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PubDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastpopulatedate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Ttl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ttl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ttl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Copyright<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, copyright: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Copyright(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(copyright, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxItemCount(this, ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadEnclosuresAutomatically(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(downloadenclosuresautomatically, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadenclosuresautomatically: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDownloadEnclosuresAutomatically(this, ::core::mem::transmute_copy(&downloadenclosuresautomatically)).into())
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Merge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Merge(this, ::core::mem::transmute(&feedxml), ::core::mem::transmute(&feedurl)).into())
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, islist: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarkAllItemsRead(this).into())
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWatcher(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnreadItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeed_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Xml: Xml::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            SyncSetting: SyncSetting::<Identity, Impl, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, Impl, OFFSET>,
            Items: Items::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Ttl: Ttl::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Copyright: Copyright::<Identity, Impl, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, Impl, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, Impl, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            IsList: IsList::<Identity, Impl, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeed2_Impl: ::windows_core::BaseImpl + IFeed_Impl {
    fn GetItemByEffectiveId(this: &Self::This, itemeffectiveid: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn LastItemDownloadTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Username(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Password(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCredentials(this: &Self::This, username: &::windows_core::BSTR, password: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClearCredentials(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeed2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFeed);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeed2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemeffectiveid: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemByEffectiveId(this, ::core::mem::transmute_copy(&itemeffectiveid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastitemdownloadtime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastItemDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastitemdownloadtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Username<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Username(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(username, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Password<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, password: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Password(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(password, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, username: ::std::mem::MaybeUninit<::windows_core::BSTR>, password: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&username), ::core::mem::transmute(&password)).into())
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearCredentials(this).into())
        }
        IFeed2_Vtbl {
            base__: <IFeed as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedEnclosure_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Url(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelAsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn DownloadStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DownloadMimeType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RemoveFile(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFile(this: &Self::This, downloadurl: &::windows_core::BSTR, downloadfilepath: &::windows_core::BSTR, downloadmimetype: &::windows_core::BSTR, enclosurefilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedEnclosure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedEnclosure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosureurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mimetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncDownload(this).into())
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncDownload(this).into())
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enclosureurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosureurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadMimeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mimetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFile(this).into())
        }
        unsafe extern "system" fn SetFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, downloadfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, downloadmimetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, enclosurefilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFile(this, ::core::mem::transmute(&downloadurl), ::core::mem::transmute(&downloadfilepath), ::core::mem::transmute(&downloadmimetype), ::core::mem::transmute(&enclosurefilename)).into())
        }
        IFeedEnclosure_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Url: Url::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            LocalPath: LocalPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, Impl, OFFSET>,
            RemoveFile: RemoveFile::<Identity, Impl, OFFSET>,
            SetFile: SetFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Error(this: &Self::This) -> ::windows_core::Result<()>;
    fn FeedDeleted(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedRenamed(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedUrlChanged(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedMoved(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedDownloading(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedDownloadCompleted(this: &Self::This, path: &::windows_core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>;
    fn FeedItemCountChanged(this: &Self::This, path: &::windows_core::BSTR, itemcounttype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Error(this).into())
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDeleted(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedRenamed(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedUrlChanged(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMoved(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloading(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloadCompleted(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&error)).into())
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedItemCountChanged(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into())
        }
        IFeedEvents_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Error: Error::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMoved: FeedMoved::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedFolder_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Feeds(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Subfolders(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn CreateFeed(this: &Self::This, feedname: &::windows_core::BSTR, feedurl: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn CreateSubfolder(this: &Self::This, foldername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFeed(this: &Self::This, feedname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(this: &Self::This, feedname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsSubfolder(this: &Self::This, foldername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetSubfolder(this: &Self::This, foldername: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Rename(this: &Self::This, foldername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(this: &Self::This, newparentpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsRoot(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn TotalUnreadItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetWatcher(this: &Self::This, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedFolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedFolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Feeds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Feeds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subfolders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFeed(this, ::core::mem::transmute(&feedname), ::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSubfolder(this, ::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsFeed(this, ::core::mem::transmute(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedname: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeed(this, ::core::mem::transmute(&feedname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsSubfolder(this, ::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubfolder(this, ::core::mem::transmute(&foldername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foldername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foldername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foldername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&foldername)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folderpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(folderpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newparentpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute(&newparentpath)).into())
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isroot: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalUnreadItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWatcher(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeedFolder_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Feeds: Feeds::<Identity, Impl, OFFSET>,
            Subfolders: Subfolders::<Identity, Impl, OFFSET>,
            CreateFeed: CreateFeed::<Identity, Impl, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, Impl, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            IsRoot: IsRoot::<Identity, Impl, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, Impl, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedFolderEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Error(this: &Self::This) -> ::windows_core::Result<()>;
    fn FolderAdded(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FolderDeleted(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FolderRenamed(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FolderMovedFrom(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FolderMovedTo(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FolderItemCountChanged(this: &Self::This, path: &::windows_core::BSTR, itemcounttype: i32) -> ::windows_core::Result<()>;
    fn FeedAdded(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedDeleted(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedRenamed(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedUrlChanged(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedMovedFrom(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedMovedTo(this: &Self::This, path: &::windows_core::BSTR, oldpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedDownloading(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FeedDownloadCompleted(this: &Self::This, path: &::windows_core::BSTR, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>;
    fn FeedItemCountChanged(this: &Self::This, path: &::windows_core::BSTR, itemcounttype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedFolderEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedFolderEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Error(this).into())
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderAdded(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderDeleted(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderRenamed(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderMovedFrom(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderMovedTo(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderItemCountChanged(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into())
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedAdded(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDeleted(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedRenamed(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedUrlChanged(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMovedFrom(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, oldpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMovedTo(this, ::core::mem::transmute(&path), ::core::mem::transmute(&oldpath)).into())
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloading(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, error: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloadCompleted(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&error)).into())
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemcounttype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedItemCountChanged(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&itemcounttype)).into())
        }
        IFeedFolderEvents_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Error: Error::<Identity, Impl, OFFSET>,
            FolderAdded: FolderAdded::<Identity, Impl, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, Impl, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, Impl, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, Impl, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, Impl, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, Impl, OFFSET>,
            FeedAdded: FeedAdded::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, Impl, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedItem_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Xml(this: &Self::This, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Link(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Guid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PubDate(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Comments(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Author(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enclosure(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsRead(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRead(this: &Self::This, isread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn LocalId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LastDownloadTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includeflags: FEEDS_XML_INCLUDE_FLAGS, xml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this, ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linkurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linkurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Guid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemguid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Guid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PubDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pubdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PubDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pubdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Comments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, comments: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Comments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, author: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Author(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(author, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enclosure(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isread: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isread: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsRead(this, ::core::mem::transmute_copy(&isread)).into())
        }
        unsafe extern "system" fn LocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastdownload: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modified: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeedItem_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Xml: Xml::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Comments: Comments::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Enclosure: Enclosure::<Identity, Impl, OFFSET>,
            IsRead: IsRead::<Identity, Impl, OFFSET>,
            SetIsRead: SetIsRead::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedItem2_Impl: ::windows_core::BaseImpl + IFeedItem_Impl {
    fn EffectiveId(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IFeedItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EffectiveId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, effectiveid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EffectiveId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(effectiveid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeedItem2_Vtbl { base__: <IFeedItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedsEnum_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedsEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedsEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeedsEnum_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFeedsManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RootFolder(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn IsSubscribed(this: &Self::This, feedurl: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExistsFeed(this: &Self::This, feedpath: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFeed(this: &Self::This, feedpath: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetFeedByUrl(this: &Self::This, feedurl: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn ExistsFolder(this: &Self::This, folderpath: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetFolder(this: &Self::This, folderpath: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn DeleteFeed(this: &Self::This, feedpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DeleteFolder(this: &Self::This, folderpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BackgroundSync(this: &Self::This, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::Result<()>;
    fn BackgroundSyncStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDefaultInterval(this: &Self::This, minutes: i32) -> ::windows_core::Result<()>;
    fn AsyncSyncAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Normalize(this: &Self::This, feedxmlin: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ItemCountLimit(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFeedsManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFeedsManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RootFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootFolder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, subscribed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSubscribed(this, ::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscribed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsFeed(this, ::core::mem::transmute(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeed(this, ::core::mem::transmute(&feedpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFeedByUrl(this, ::core::mem::transmute(&feedurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, exists: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsFolder(this, ::core::mem::transmute(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, disp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFolder(this, ::core::mem::transmute(&folderpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFeed(this, ::core::mem::transmute(&feedpath)).into())
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, folderpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFolder(this, ::core::mem::transmute(&folderpath)).into())
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, action: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackgroundSync(this, ::core::mem::transmute_copy(&action)).into())
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackgroundSyncStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minutes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minutes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultInterval(this, ::core::mem::transmute_copy(&minutes)).into())
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncSyncAll(this).into())
        }
        unsafe extern "system" fn Normalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feedxmlin: ::std::mem::MaybeUninit<::windows_core::BSTR>, feedxmlout: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Normalize(this, ::core::mem::transmute(&feedxmlin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(feedxmlout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemcountlimit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemCountLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemcountlimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFeedsManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RootFolder: RootFolder::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, Impl, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, Impl, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, Impl, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, Impl, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, Impl, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPAudioRenderConfig_Impl: ::windows_core::BaseImpl {
    fn audioOutputDevice(this: &Self::This, pbstroutputdevice: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetaudioOutputDevice(this: &Self::This, bstroutputdevice: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPAudioRenderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPAudioRenderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn audioOutputDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstroutputdevice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::audioOutputDevice(this, ::core::mem::transmute_copy(&pbstroutputdevice)).into())
        }
        unsafe extern "system" fn SetaudioOutputDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPAudioRenderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstroutputdevice: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetaudioOutputDevice(this, ::core::mem::transmute(&bstroutputdevice)).into())
        }
        IWMPAudioRenderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            audioOutputDevice: audioOutputDevice::<Identity, Impl, OFFSET>,
            SetaudioOutputDevice: SetaudioOutputDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPCdrom_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn driveSpecifier(this: &Self::This, pbstrdrive: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn playlist(this: &Self::This) -> ::windows_core::Result<IWMPPlaylist>;
    fn eject(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPCdrom {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCdrom {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn driveSpecifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdrive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::driveSpecifier(this, ::core::mem::transmute_copy(&pbstrdrive)).into())
        }
        unsafe extern "system" fn playlist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::playlist(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn eject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdrom_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::eject(this).into())
        }
        IWMPCdrom_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            driveSpecifier: driveSpecifier::<Identity, Impl, OFFSET>,
            playlist: playlist::<Identity, Impl, OFFSET>,
            eject: eject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPCdromBurn_Impl: ::windows_core::BaseImpl {
    fn isAvailable(this: &Self::This, bstritem: &::windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getItemInfo(this: &Self::This, bstritem: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn label(this: &Self::This, pbstrlabel: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Setlabel(this: &Self::This, bstrlabel: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn burnFormat(this: &Self::This, pwmpbf: *mut WMPBurnFormat) -> ::windows_core::Result<()>;
    fn SetburnFormat(this: &Self::This, wmpbf: WMPBurnFormat) -> ::windows_core::Result<()>;
    fn burnPlaylist(this: &Self::This) -> ::windows_core::Result<IWMPPlaylist>;
    fn SetburnPlaylist(this: &Self::This, pplaylist: ::core::option::Option<&IWMPPlaylist>) -> ::windows_core::Result<()>;
    fn refreshStatus(this: &Self::This) -> ::windows_core::Result<()>;
    fn burnState(this: &Self::This, pwmpbs: *mut WMPBurnState) -> ::windows_core::Result<()>;
    fn burnProgress(this: &Self::This, plprogress: *mut i32) -> ::windows_core::Result<()>;
    fn startBurn(this: &Self::This) -> ::windows_core::Result<()>;
    fn stopBurn(this: &Self::This) -> ::windows_core::Result<()>;
    fn erase(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPCdromBurn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCdromBurn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn isAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isAvailable(this, ::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into())
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        unsafe extern "system" fn label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlabel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::label(this, ::core::mem::transmute_copy(&pbstrlabel)).into())
        }
        unsafe extern "system" fn Setlabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrlabel: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setlabel(this, ::core::mem::transmute(&bstrlabel)).into())
        }
        unsafe extern "system" fn burnFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpbf: *mut WMPBurnFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::burnFormat(this, ::core::mem::transmute_copy(&pwmpbf)).into())
        }
        unsafe extern "system" fn SetburnFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmpbf: WMPBurnFormat) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetburnFormat(this, ::core::mem::transmute_copy(&wmpbf)).into())
        }
        unsafe extern "system" fn burnPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::burnPlaylist(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetburnPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetburnPlaylist(this, ::windows_core::from_raw_borrowed(&pplaylist)).into())
        }
        unsafe extern "system" fn refreshStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::refreshStatus(this).into())
        }
        unsafe extern "system" fn burnState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpbs: *mut WMPBurnState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::burnState(this, ::core::mem::transmute_copy(&pwmpbs)).into())
        }
        unsafe extern "system" fn burnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::burnProgress(this, ::core::mem::transmute_copy(&plprogress)).into())
        }
        unsafe extern "system" fn startBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startBurn(this).into())
        }
        unsafe extern "system" fn stopBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stopBurn(this).into())
        }
        unsafe extern "system" fn erase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromBurn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::erase(this).into())
        }
        IWMPCdromBurn_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            isAvailable: isAvailable::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            label: label::<Identity, Impl, OFFSET>,
            Setlabel: Setlabel::<Identity, Impl, OFFSET>,
            burnFormat: burnFormat::<Identity, Impl, OFFSET>,
            SetburnFormat: SetburnFormat::<Identity, Impl, OFFSET>,
            burnPlaylist: burnPlaylist::<Identity, Impl, OFFSET>,
            SetburnPlaylist: SetburnPlaylist::<Identity, Impl, OFFSET>,
            refreshStatus: refreshStatus::<Identity, Impl, OFFSET>,
            burnState: burnState::<Identity, Impl, OFFSET>,
            burnProgress: burnProgress::<Identity, Impl, OFFSET>,
            startBurn: startBurn::<Identity, Impl, OFFSET>,
            stopBurn: stopBurn::<Identity, Impl, OFFSET>,
            erase: erase::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPCdromCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn item(this: &Self::This, lindex: i32) -> ::windows_core::Result<IWMPCdrom>;
    fn getByDriveSpecifier(this: &Self::This, bstrdrivespecifier: &::windows_core::BSTR) -> ::windows_core::Result<IWMPCdrom>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPCdromCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCdromCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByDriveSpecifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdrivespecifier: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcdrom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByDriveSpecifier(this, ::core::mem::transmute(&bstrdrivespecifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcdrom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPCdromCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            getByDriveSpecifier: getByDriveSpecifier::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPCdromRip_Impl: ::windows_core::BaseImpl {
    fn ripState(this: &Self::This, pwmprs: *mut WMPRipState) -> ::windows_core::Result<()>;
    fn ripProgress(this: &Self::This, plprogress: *mut i32) -> ::windows_core::Result<()>;
    fn startRip(this: &Self::This) -> ::windows_core::Result<()>;
    fn stopRip(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPCdromRip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCdromRip {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ripState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmprs: *mut WMPRipState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ripState(this, ::core::mem::transmute_copy(&pwmprs)).into())
        }
        unsafe extern "system" fn ripProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ripProgress(this, ::core::mem::transmute_copy(&plprogress)).into())
        }
        unsafe extern "system" fn startRip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startRip(this).into())
        }
        unsafe extern "system" fn stopRip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCdromRip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stopRip(this).into())
        }
        IWMPCdromRip_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ripState: ripState::<Identity, Impl, OFFSET>,
            ripProgress: ripProgress::<Identity, Impl, OFFSET>,
            startRip: startRip::<Identity, Impl, OFFSET>,
            stopRip: stopRip::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPClosedCaption_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SAMIStyle(this: &Self::This, pbstrsamistyle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSAMIStyle(this: &Self::This, bstrsamistyle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SAMILang(this: &Self::This, pbstrsamilang: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSAMILang(this: &Self::This, bstrsamilang: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SAMIFileName(this: &Self::This, pbstrsamifilename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetSAMIFileName(this: &Self::This, bstrsamifilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn captioningId(this: &Self::This, pbstrcaptioningid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetcaptioningId(this: &Self::This, bstrcaptioningid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPClosedCaption {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPClosedCaption {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SAMIStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsamistyle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAMIStyle(this, ::core::mem::transmute_copy(&pbstrsamistyle)).into())
        }
        unsafe extern "system" fn SetSAMIStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsamistyle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSAMIStyle(this, ::core::mem::transmute(&bstrsamistyle)).into())
        }
        unsafe extern "system" fn SAMILang<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsamilang: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAMILang(this, ::core::mem::transmute_copy(&pbstrsamilang)).into())
        }
        unsafe extern "system" fn SetSAMILang<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsamilang: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSAMILang(this, ::core::mem::transmute(&bstrsamilang)).into())
        }
        unsafe extern "system" fn SAMIFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsamifilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAMIFileName(this, ::core::mem::transmute_copy(&pbstrsamifilename)).into())
        }
        unsafe extern "system" fn SetSAMIFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsamifilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSAMIFileName(this, ::core::mem::transmute(&bstrsamifilename)).into())
        }
        unsafe extern "system" fn captioningId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcaptioningid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::captioningId(this, ::core::mem::transmute_copy(&pbstrcaptioningid)).into())
        }
        unsafe extern "system" fn SetcaptioningId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcaptioningid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcaptioningId(this, ::core::mem::transmute(&bstrcaptioningid)).into())
        }
        IWMPClosedCaption_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SAMIStyle: SAMIStyle::<Identity, Impl, OFFSET>,
            SetSAMIStyle: SetSAMIStyle::<Identity, Impl, OFFSET>,
            SAMILang: SAMILang::<Identity, Impl, OFFSET>,
            SetSAMILang: SetSAMILang::<Identity, Impl, OFFSET>,
            SAMIFileName: SAMIFileName::<Identity, Impl, OFFSET>,
            SetSAMIFileName: SetSAMIFileName::<Identity, Impl, OFFSET>,
            captioningId: captioningId::<Identity, Impl, OFFSET>,
            SetcaptioningId: SetcaptioningId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPClosedCaption2_Impl: ::windows_core::BaseImpl + IWMPClosedCaption_Impl {
    fn SAMILangCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getSAMILangName(this: &Self::This, nindex: i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getSAMILangID(this: &Self::This, nindex: i32, pllangid: *mut i32) -> ::windows_core::Result<()>;
    fn SAMIStyleCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getSAMIStyleName(this: &Self::This, nindex: i32, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPClosedCaption2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPClosedCaption);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPClosedCaption2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SAMILangCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAMILangCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getSAMILangName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getSAMILangName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn getSAMILangID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, pllangid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getSAMILangID(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pllangid)).into())
        }
        unsafe extern "system" fn SAMIStyleCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SAMIStyleCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getSAMIStyleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPClosedCaption2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getSAMIStyleName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pbstrname)).into())
        }
        IWMPClosedCaption2_Vtbl {
            base__: <IWMPClosedCaption as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SAMILangCount: SAMILangCount::<Identity, Impl, OFFSET>,
            getSAMILangName: getSAMILangName::<Identity, Impl, OFFSET>,
            getSAMILangID: getSAMILangID::<Identity, Impl, OFFSET>,
            SAMIStyleCount: SAMIStyleCount::<Identity, Impl, OFFSET>,
            getSAMIStyleName: getSAMIStyleName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPContentContainer_Impl: ::windows_core::BaseImpl {
    fn GetID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPrice(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContentCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetContentPrice(this: &Self::This, idxcontent: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContentID(this: &Self::This, idxcontent: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWMPContentContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPContentContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontentid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontentid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccontent: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentPrice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pbstrprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentPrice(this, ::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idxcontent: u32, pcontentid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentID(this, ::core::mem::transmute_copy(&idxcontent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontentid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPContentContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetPrice: GetPrice::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetContentCount: GetContentCount::<Identity, Impl, OFFSET>,
            GetContentPrice: GetContentPrice::<Identity, Impl, OFFSET>,
            GetContentID: GetContentID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPContentContainerList_Impl: ::windows_core::BaseImpl {
    fn GetTransactionType(this: &Self::This) -> ::windows_core::Result<WMPTransactionType>;
    fn GetContainerCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetContainer(this: &Self::This, idxcontainer: u32) -> ::windows_core::Result<IWMPContentContainer>;
}
impl ::windows_core::Iids for IWMPContentContainerList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPContentContainerList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransactionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmptt: *mut WMPTransactionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransactionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwmptt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContainerCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccontainer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainerCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentContainerList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idxcontainer: u32, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainer(this, ::core::mem::transmute_copy(&idxcontainer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPContentContainerList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransactionType: GetTransactionType::<Identity, Impl, OFFSET>,
            GetContainerCount: GetContainerCount::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPContentPartner_Impl: ::windows_core::BaseImpl {
    fn SetCallback(this: &Self::This, pcallback: ::core::option::Option<&IWMPContentPartnerCallback>) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetItemInfo(this: &Self::This, bstrinfoname: &::windows_core::BSTR, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetContentPartnerInfo(this: &Self::This, bstrinfoname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCommands(this: &Self::This, location: &::windows_core::BSTR, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: &::windows_core::BSTR, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows_core::Result<()>;
    fn InvokeCommand(this: &Self::This, dwcommandid: u32, location: &::windows_core::BSTR, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: &::windows_core::BSTR, citemids: u32, rgitemids: *const u32) -> ::windows_core::Result<()>;
    fn CanBuySilent(this: &Self::This, pinfo: ::core::option::Option<&IWMPContentContainerList>, pbstrtotalprice: *mut ::windows_core::BSTR, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Buy(this: &Self::This, pinfo: ::core::option::Option<&IWMPContentContainerList>, cookie: u32) -> ::windows_core::Result<()>;
    fn GetStreamingURL(this: &Self::This, st: WMPStreamingType, pstreamcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Download(this: &Self::This, pinfo: ::core::option::Option<&IWMPContentContainerList>, cookie: u32) -> ::windows_core::Result<()>;
    fn DownloadTrackComplete(this: &Self::This, hrresult: ::windows_core::HRESULT, contentid: u32, downloadtrackparam: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RefreshLicense(this: &Self::This, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: &::windows_core::BSTR, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: &::windows_core::BSTR, preasoncontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetCatalogURL(this: &Self::This, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::windows_core::BSTR, pexpirationdate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetTemplate(this: &Self::This, task: WMPTaskType, location: &::windows_core::BSTR, pcontext: *const super::super::System::Variant::VARIANT, clicklocation: &::windows_core::BSTR, pclickcontext: *const super::super::System::Variant::VARIANT, bstrfilter: &::windows_core::BSTR, bstrviewparams: &::windows_core::BSTR, pbstrtemplateurl: *mut ::windows_core::BSTR, ptemplatesize: *mut WMPTemplateSize) -> ::windows_core::Result<()>;
    fn UpdateDevice(this: &Self::This, bstrdevicename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetListContents(this: &Self::This, location: &::windows_core::BSTR, pcontext: *const super::super::System::Variant::VARIANT, bstrlisttype: &::windows_core::BSTR, bstrparams: &::windows_core::BSTR, dwlistcookie: u32) -> ::windows_core::Result<()>;
    fn Login(this: &Self::This, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Authenticate(this: &Self::This, userinfo: &super::super::System::Com::BLOB, pwdinfo: &super::super::System::Com::BLOB) -> ::windows_core::Result<()>;
    fn Logout(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendMessage(this: &Self::This, bstrmsg: &::windows_core::BSTR, bstrparam: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StationEvent(this: &Self::This, bstrstationeventtype: &::windows_core::BSTR, stationid: u32, playlistindex: u32, trackid: u32, trackdata: &::windows_core::BSTR, dwsecondsplayed: u32) -> ::windows_core::Result<()>;
    fn CompareContainerListPrices(this: &Self::This, plistbase: ::core::option::Option<&IWMPContentContainerList>, plistcompare: ::core::option::Option<&IWMPContentContainerList>) -> ::windows_core::Result<i32>;
    fn VerifyPermission(this: &Self::This, bstrpermission: &::windows_core::BSTR, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPContentPartner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPContentPartner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallback(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WMPPartnerNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn GetItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemInfo(this, ::core::mem::transmute(&bstrinfoname), ::core::mem::transmute_copy(&pcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentPartnerInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinfoname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdata: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentPartnerInfo(this, ::core::mem::transmute(&bstrinfoname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, citemids: u32, prgitemids: *const u32, pcitemids: *mut u32, pprgitems: *mut *mut WMPContextMenuInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommands(this, ::core::mem::transmute(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&prgitemids), ::core::mem::transmute_copy(&pcitemids), ::core::mem::transmute_copy(&pprgitems)).into())
        }
        unsafe extern "system" fn InvokeCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcommandid: u32, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, plocationcontext: *const super::super::System::Variant::VARIANT, itemlocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, citemids: u32, rgitemids: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeCommand(this, ::core::mem::transmute_copy(&dwcommandid), ::core::mem::transmute(&location), ::core::mem::transmute_copy(&plocationcontext), ::core::mem::transmute(&itemlocation), ::core::mem::transmute_copy(&citemids), ::core::mem::transmute_copy(&rgitemids)).into())
        }
        unsafe extern "system" fn CanBuySilent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, pbstrtotalprice: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, psilentok: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanBuySilent(this, ::windows_core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&pbstrtotalprice), ::core::mem::transmute_copy(&psilentok)).into())
        }
        unsafe extern "system" fn Buy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Buy(this, ::windows_core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn GetStreamingURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, st: WMPStreamingType, pstreamcontext: *const super::super::System::Variant::VARIANT, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreamingURL(this, ::core::mem::transmute_copy(&st), ::core::mem::transmute_copy(&pstreamcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this, ::windows_core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn DownloadTrackComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, contentid: u32, downloadtrackparam: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DownloadTrackComplete(this, ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute(&downloadtrackparam)).into())
        }
        unsafe extern "system" fn RefreshLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32, flocal: super::super::Foundation::VARIANT_BOOL, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, r#type: WMPStreamingType, contentid: u32, bstrrefreshreason: ::std::mem::MaybeUninit<::windows_core::BSTR>, preasoncontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshLicense(this, ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&flocal), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute(&bstrrefreshreason), ::core::mem::transmute_copy(&preasoncontext)).into())
        }
        unsafe extern "system" fn GetCatalogURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcatalogversion: u32, dwcatalogschemaversion: u32, cataloglcid: u32, pdwnewcatalogversion: *mut u32, pbstrcatalogurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexpirationdate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCatalogURL(this, ::core::mem::transmute_copy(&dwcatalogversion), ::core::mem::transmute_copy(&dwcatalogschemaversion), ::core::mem::transmute_copy(&cataloglcid), ::core::mem::transmute_copy(&pdwnewcatalogversion), ::core::mem::transmute_copy(&pbstrcatalogurl), ::core::mem::transmute_copy(&pexpirationdate)).into())
        }
        unsafe extern "system" fn GetTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, task: WMPTaskType, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, clicklocation: ::std::mem::MaybeUninit<::windows_core::BSTR>, pclickcontext: *const super::super::System::Variant::VARIANT, bstrfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrviewparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrtemplateurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ptemplatesize: *mut WMPTemplateSize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTemplate(this, ::core::mem::transmute_copy(&task), ::core::mem::transmute(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&clicklocation), ::core::mem::transmute_copy(&pclickcontext), ::core::mem::transmute(&bstrfilter), ::core::mem::transmute(&bstrviewparams), ::core::mem::transmute_copy(&pbstrtemplateurl), ::core::mem::transmute_copy(&ptemplatesize)).into())
        }
        unsafe extern "system" fn UpdateDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDevice(this, ::core::mem::transmute(&bstrdevicename)).into())
        }
        unsafe extern "system" fn GetListContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, bstrlisttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwlistcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetListContents(this, ::core::mem::transmute(&location), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute(&bstrlisttype), ::core::mem::transmute(&bstrparams), ::core::mem::transmute_copy(&dwlistcookie)).into())
        }
        unsafe extern "system" fn Login<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB, fusedcachedcreds: super::super::Foundation::VARIANT_BOOL, foktocache: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Login(this, ::core::mem::transmute(&userinfo), ::core::mem::transmute(&pwdinfo), ::core::mem::transmute_copy(&fusedcachedcreds), ::core::mem::transmute_copy(&foktocache)).into())
        }
        unsafe extern "system" fn Authenticate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, userinfo: super::super::System::Com::BLOB, pwdinfo: super::super::System::Com::BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Authenticate(this, ::core::mem::transmute(&userinfo), ::core::mem::transmute(&pwdinfo)).into())
        }
        unsafe extern "system" fn Logout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Logout(this).into())
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessage(this, ::core::mem::transmute(&bstrmsg), ::core::mem::transmute(&bstrparam)).into())
        }
        unsafe extern "system" fn StationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrstationeventtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, stationid: u32, playlistindex: u32, trackid: u32, trackdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsecondsplayed: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StationEvent(this, ::core::mem::transmute(&bstrstationeventtype), ::core::mem::transmute_copy(&stationid), ::core::mem::transmute_copy(&playlistindex), ::core::mem::transmute_copy(&trackid), ::core::mem::transmute(&trackdata), ::core::mem::transmute_copy(&dwsecondsplayed)).into())
        }
        unsafe extern "system" fn CompareContainerListPrices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plistbase: *mut ::core::ffi::c_void, plistcompare: *mut ::core::ffi::c_void, presult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareContainerListPrices(this, ::windows_core::from_raw_borrowed(&plistbase), ::windows_core::from_raw_borrowed(&plistcompare)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VerifyPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VerifyPermission(this, ::core::mem::transmute(&bstrpermission), ::core::mem::transmute_copy(&pcontext)).into())
        }
        IWMPContentPartner_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, Impl, OFFSET>,
            GetContentPartnerInfo: GetContentPartnerInfo::<Identity, Impl, OFFSET>,
            GetCommands: GetCommands::<Identity, Impl, OFFSET>,
            InvokeCommand: InvokeCommand::<Identity, Impl, OFFSET>,
            CanBuySilent: CanBuySilent::<Identity, Impl, OFFSET>,
            Buy: Buy::<Identity, Impl, OFFSET>,
            GetStreamingURL: GetStreamingURL::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            DownloadTrackComplete: DownloadTrackComplete::<Identity, Impl, OFFSET>,
            RefreshLicense: RefreshLicense::<Identity, Impl, OFFSET>,
            GetCatalogURL: GetCatalogURL::<Identity, Impl, OFFSET>,
            GetTemplate: GetTemplate::<Identity, Impl, OFFSET>,
            UpdateDevice: UpdateDevice::<Identity, Impl, OFFSET>,
            GetListContents: GetListContents::<Identity, Impl, OFFSET>,
            Login: Login::<Identity, Impl, OFFSET>,
            Authenticate: Authenticate::<Identity, Impl, OFFSET>,
            Logout: Logout::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            StationEvent: StationEvent::<Identity, Impl, OFFSET>,
            CompareContainerListPrices: CompareContainerListPrices::<Identity, Impl, OFFSET>,
            VerifyPermission: VerifyPermission::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPContentPartnerCallback_Impl: ::windows_core::BaseImpl {
    fn Notify(this: &Self::This, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn BuyComplete(this: &Self::This, hrresult: ::windows_core::HRESULT, dwbuycookie: u32) -> ::windows_core::Result<()>;
    fn DownloadTrack(this: &Self::This, cookie: u32, bstrtrackurl: &::windows_core::BSTR, dwservicetrackid: u32, bstrdownloadparams: &::windows_core::BSTR, hrdownload: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetCatalogVersion(this: &Self::This, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows_core::Result<()>;
    fn UpdateDeviceComplete(this: &Self::This, bstrdevicename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangeView(this: &Self::This, bstrtype: &::windows_core::BSTR, bstrid: &::windows_core::BSTR, bstrfilter: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddListContents(this: &Self::This, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows_core::Result<()>;
    fn ListContentsComplete(this: &Self::This, dwlistcookie: u32, hrsuccess: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn SendMessageComplete(this: &Self::This, bstrmsg: &::windows_core::BSTR, bstrparam: &::windows_core::BSTR, bstrresult: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetContentIDsInLibrary(this: &Self::This, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows_core::Result<()>;
    fn RefreshLicenseComplete(this: &Self::This, dwcookie: u32, contentid: u32, hrrefresh: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ShowPopup(this: &Self::This, lindex: i32, bstrparameters: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn VerifyPermissionComplete(this: &Self::This, bstrpermission: &::windows_core::BSTR, pcontext: *const super::super::System::Variant::VARIANT, hrpermission: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPContentPartnerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPContentPartnerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: WMPCallbackNotification, pcontext: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn BuyComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, dwbuycookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuyComplete(this, ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&dwbuycookie)).into())
        }
        unsafe extern "system" fn DownloadTrack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32, bstrtrackurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwservicetrackid: u32, bstrdownloadparams: ::std::mem::MaybeUninit<::windows_core::BSTR>, hrdownload: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DownloadTrack(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute(&bstrtrackurl), ::core::mem::transmute_copy(&dwservicetrackid), ::core::mem::transmute(&bstrdownloadparams), ::core::mem::transmute_copy(&hrdownload)).into())
        }
        unsafe extern "system" fn GetCatalogVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32, pdwschemaversion: *mut u32, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCatalogVersion(this, ::core::mem::transmute_copy(&pdwversion), ::core::mem::transmute_copy(&pdwschemaversion), ::core::mem::transmute_copy(&plcid)).into())
        }
        unsafe extern "system" fn UpdateDeviceComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDeviceComplete(this, ::core::mem::transmute(&bstrdevicename)).into())
        }
        unsafe extern "system" fn ChangeView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeView(this, ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrid), ::core::mem::transmute(&bstrfilter)).into())
        }
        unsafe extern "system" fn AddListContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, citems: u32, prgitems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddListContents(this, ::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&citems), ::core::mem::transmute_copy(&prgitems)).into())
        }
        unsafe extern "system" fn ListContentsComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlistcookie: u32, hrsuccess: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ListContentsComplete(this, ::core::mem::transmute_copy(&dwlistcookie), ::core::mem::transmute_copy(&hrsuccess)).into())
        }
        unsafe extern "system" fn SendMessageComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmsg: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrparam: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessageComplete(this, ::core::mem::transmute(&bstrmsg), ::core::mem::transmute(&bstrparam), ::core::mem::transmute(&bstrresult)).into())
        }
        unsafe extern "system" fn GetContentIDsInLibrary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccontentids: *mut u32, pprgids: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentIDsInLibrary(this, ::core::mem::transmute_copy(&pccontentids), ::core::mem::transmute_copy(&pprgids)).into())
        }
        unsafe extern "system" fn RefreshLicenseComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32, contentid: u32, hrrefresh: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshLicenseComplete(this, ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&contentid), ::core::mem::transmute_copy(&hrrefresh)).into())
        }
        unsafe extern "system" fn ShowPopup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowPopup(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrparameters)).into())
        }
        unsafe extern "system" fn VerifyPermissionComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPContentPartnerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpermission: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontext: *const super::super::System::Variant::VARIANT, hrpermission: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VerifyPermissionComplete(this, ::core::mem::transmute(&bstrpermission), ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrpermission)).into())
        }
        IWMPContentPartnerCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Notify: Notify::<Identity, Impl, OFFSET>,
            BuyComplete: BuyComplete::<Identity, Impl, OFFSET>,
            DownloadTrack: DownloadTrack::<Identity, Impl, OFFSET>,
            GetCatalogVersion: GetCatalogVersion::<Identity, Impl, OFFSET>,
            UpdateDeviceComplete: UpdateDeviceComplete::<Identity, Impl, OFFSET>,
            ChangeView: ChangeView::<Identity, Impl, OFFSET>,
            AddListContents: AddListContents::<Identity, Impl, OFFSET>,
            ListContentsComplete: ListContentsComplete::<Identity, Impl, OFFSET>,
            SendMessageComplete: SendMessageComplete::<Identity, Impl, OFFSET>,
            GetContentIDsInLibrary: GetContentIDsInLibrary::<Identity, Impl, OFFSET>,
            RefreshLicenseComplete: RefreshLicenseComplete::<Identity, Impl, OFFSET>,
            ShowPopup: ShowPopup::<Identity, Impl, OFFSET>,
            VerifyPermissionComplete: VerifyPermissionComplete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPControls_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(this: &Self::This, bstritem: &::windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn play(this: &Self::This) -> ::windows_core::Result<()>;
    fn stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn fastForward(this: &Self::This) -> ::windows_core::Result<()>;
    fn fastReverse(this: &Self::This) -> ::windows_core::Result<()>;
    fn currentPosition(this: &Self::This, pdcurrentposition: *mut f64) -> ::windows_core::Result<()>;
    fn SetcurrentPosition(this: &Self::This, dcurrentposition: f64) -> ::windows_core::Result<()>;
    fn currentPositionString(this: &Self::This, pbstrcurrentposition: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn next(this: &Self::This) -> ::windows_core::Result<()>;
    fn previous(this: &Self::This) -> ::windows_core::Result<()>;
    fn currentItem(this: &Self::This) -> ::windows_core::Result<IWMPMedia>;
    fn SetcurrentItem(this: &Self::This, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn currentMarker(this: &Self::This, plmarker: *mut i32) -> ::windows_core::Result<()>;
    fn SetcurrentMarker(this: &Self::This, lmarker: i32) -> ::windows_core::Result<()>;
    fn playItem(this: &Self::This, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPControls {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPControls {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_isAvailable(this, ::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into())
        }
        unsafe extern "system" fn play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::play(this).into())
        }
        unsafe extern "system" fn stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stop(this).into())
        }
        unsafe extern "system" fn pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pause(this).into())
        }
        unsafe extern "system" fn fastForward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fastForward(this).into())
        }
        unsafe extern "system" fn fastReverse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fastReverse(this).into())
        }
        unsafe extern "system" fn currentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcurrentposition: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentPosition(this, ::core::mem::transmute_copy(&pdcurrentposition)).into())
        }
        unsafe extern "system" fn SetcurrentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dcurrentposition: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentPosition(this, ::core::mem::transmute_copy(&dcurrentposition)).into())
        }
        unsafe extern "system" fn currentPositionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcurrentposition: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentPositionString(this, ::core::mem::transmute_copy(&pbstrcurrentposition)).into())
        }
        unsafe extern "system" fn next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::next(this).into())
        }
        unsafe extern "system" fn previous<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::previous(this).into())
        }
        unsafe extern "system" fn currentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::currentItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmedia, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcurrentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentItem(this, ::windows_core::from_raw_borrowed(&piwmpmedia)).into())
        }
        unsafe extern "system" fn currentMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmarker: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentMarker(this, ::core::mem::transmute_copy(&plmarker)).into())
        }
        unsafe extern "system" fn SetcurrentMarker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmarker: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentMarker(this, ::core::mem::transmute_copy(&lmarker)).into())
        }
        unsafe extern "system" fn playItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::playItem(this, ::windows_core::from_raw_borrowed(&piwmpmedia)).into())
        }
        IWMPControls_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
            play: play::<Identity, Impl, OFFSET>,
            stop: stop::<Identity, Impl, OFFSET>,
            pause: pause::<Identity, Impl, OFFSET>,
            fastForward: fastForward::<Identity, Impl, OFFSET>,
            fastReverse: fastReverse::<Identity, Impl, OFFSET>,
            currentPosition: currentPosition::<Identity, Impl, OFFSET>,
            SetcurrentPosition: SetcurrentPosition::<Identity, Impl, OFFSET>,
            currentPositionString: currentPositionString::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            previous: previous::<Identity, Impl, OFFSET>,
            currentItem: currentItem::<Identity, Impl, OFFSET>,
            SetcurrentItem: SetcurrentItem::<Identity, Impl, OFFSET>,
            currentMarker: currentMarker::<Identity, Impl, OFFSET>,
            SetcurrentMarker: SetcurrentMarker::<Identity, Impl, OFFSET>,
            playItem: playItem::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPControls2_Impl: ::windows_core::BaseImpl + IWMPControls_Impl {
    fn step(this: &Self::This, lstep: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPControls2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPControls);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPControls2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn step<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstep: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::step(this, ::core::mem::transmute_copy(&lstep)).into())
        }
        IWMPControls2_Vtbl { base__: <IWMPControls as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, step: step::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPControls3_Impl: ::windows_core::BaseImpl + IWMPControls2_Impl {
    fn audioLanguageCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getAudioLanguageID(this: &Self::This, lindex: i32, pllangid: *mut i32) -> ::windows_core::Result<()>;
    fn getAudioLanguageDescription(this: &Self::This, lindex: i32, pbstrlangdesc: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn currentAudioLanguage(this: &Self::This, pllangid: *mut i32) -> ::windows_core::Result<()>;
    fn SetcurrentAudioLanguage(this: &Self::This, llangid: i32) -> ::windows_core::Result<()>;
    fn currentAudioLanguageIndex(this: &Self::This, plindex: *mut i32) -> ::windows_core::Result<()>;
    fn SetcurrentAudioLanguageIndex(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
    fn getLanguageName(this: &Self::This, llangid: i32, pbstrlangname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn currentPositionTimecode(this: &Self::This, bstrtimecode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetcurrentPositionTimecode(this: &Self::This, bstrtimecode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPControls3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPControls2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPControls3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn audioLanguageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::audioLanguageCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getAudioLanguageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pllangid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getAudioLanguageID(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pllangid)).into())
        }
        unsafe extern "system" fn getAudioLanguageDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrlangdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getAudioLanguageDescription(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrlangdesc)).into())
        }
        unsafe extern "system" fn currentAudioLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentAudioLanguage(this, ::core::mem::transmute_copy(&pllangid)).into())
        }
        unsafe extern "system" fn SetcurrentAudioLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llangid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentAudioLanguage(this, ::core::mem::transmute_copy(&llangid)).into())
        }
        unsafe extern "system" fn currentAudioLanguageIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentAudioLanguageIndex(this, ::core::mem::transmute_copy(&plindex)).into())
        }
        unsafe extern "system" fn SetcurrentAudioLanguageIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentAudioLanguageIndex(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        unsafe extern "system" fn getLanguageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llangid: i32, pbstrlangname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getLanguageName(this, ::core::mem::transmute_copy(&llangid), ::core::mem::transmute_copy(&pbstrlangname)).into())
        }
        unsafe extern "system" fn currentPositionTimecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtimecode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentPositionTimecode(this, ::core::mem::transmute_copy(&bstrtimecode)).into())
        }
        unsafe extern "system" fn SetcurrentPositionTimecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPControls3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtimecode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentPositionTimecode(this, ::core::mem::transmute(&bstrtimecode)).into())
        }
        IWMPControls3_Vtbl {
            base__: <IWMPControls2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            audioLanguageCount: audioLanguageCount::<Identity, Impl, OFFSET>,
            getAudioLanguageID: getAudioLanguageID::<Identity, Impl, OFFSET>,
            getAudioLanguageDescription: getAudioLanguageDescription::<Identity, Impl, OFFSET>,
            currentAudioLanguage: currentAudioLanguage::<Identity, Impl, OFFSET>,
            SetcurrentAudioLanguage: SetcurrentAudioLanguage::<Identity, Impl, OFFSET>,
            currentAudioLanguageIndex: currentAudioLanguageIndex::<Identity, Impl, OFFSET>,
            SetcurrentAudioLanguageIndex: SetcurrentAudioLanguageIndex::<Identity, Impl, OFFSET>,
            getLanguageName: getLanguageName::<Identity, Impl, OFFSET>,
            currentPositionTimecode: currentPositionTimecode::<Identity, Impl, OFFSET>,
            SetcurrentPositionTimecode: SetcurrentPositionTimecode::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPConvert_Impl: ::windows_core::BaseImpl {
    fn ConvertFile(this: &Self::This, bstrinputfile: &::windows_core::BSTR, bstrdestinationfolder: &::windows_core::BSTR, pbstroutputfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetErrorURL(this: &Self::This, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPConvert {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPConvert {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConvertFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinputfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestinationfolder: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutputfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertFile(this, ::core::mem::transmute(&bstrinputfile), ::core::mem::transmute(&bstrdestinationfolder), ::core::mem::transmute_copy(&pbstroutputfile)).into())
        }
        unsafe extern "system" fn GetErrorURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPConvert_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorURL(this, ::core::mem::transmute_copy(&pbstrurl)).into())
        }
        IWMPConvert_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConvertFile: ConvertFile::<Identity, Impl, OFFSET>,
            GetErrorURL: GetErrorURL::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPCore_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn close(this: &Self::This) -> ::windows_core::Result<()>;
    fn URL(this: &Self::This, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetURL(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn openState(this: &Self::This, pwmpos: *mut WMPOpenState) -> ::windows_core::Result<()>;
    fn playState(this: &Self::This, pwmpps: *mut WMPPlayState) -> ::windows_core::Result<()>;
    fn controls(this: &Self::This) -> ::windows_core::Result<IWMPControls>;
    fn settings(this: &Self::This) -> ::windows_core::Result<IWMPSettings>;
    fn currentMedia(this: &Self::This) -> ::windows_core::Result<IWMPMedia>;
    fn SetcurrentMedia(this: &Self::This, pmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn mediaCollection(this: &Self::This) -> ::windows_core::Result<IWMPMediaCollection>;
    fn playlistCollection(this: &Self::This) -> ::windows_core::Result<IWMPPlaylistCollection>;
    fn versionInfo(this: &Self::This, pbstrversioninfo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn launchURL(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn network(this: &Self::This) -> ::windows_core::Result<IWMPNetwork>;
    fn currentPlaylist(this: &Self::This) -> ::windows_core::Result<IWMPPlaylist>;
    fn SetcurrentPlaylist(this: &Self::This, ppl: ::core::option::Option<&IWMPPlaylist>) -> ::windows_core::Result<()>;
    fn cdromCollection(this: &Self::This) -> ::windows_core::Result<IWMPCdromCollection>;
    fn closedCaption(this: &Self::This) -> ::windows_core::Result<IWMPClosedCaption>;
    fn isOnline(this: &Self::This, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn error(this: &Self::This) -> ::windows_core::Result<IWMPError>;
    fn status(this: &Self::This, pbstrstatus: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPCore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::close(this).into())
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::URL(this, ::core::mem::transmute_copy(&pbstrurl)).into())
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURL(this, ::core::mem::transmute(&bstrurl)).into())
        }
        unsafe extern "system" fn openState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpos: *mut WMPOpenState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::openState(this, ::core::mem::transmute_copy(&pwmpos)).into())
        }
        unsafe extern "system" fn playState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpps: *mut WMPPlayState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::playState(this, ::core::mem::transmute_copy(&pwmpps)).into())
        }
        unsafe extern "system" fn controls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::controls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn settings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsettings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::settings(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsettings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn currentMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::currentMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmedia, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcurrentMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentMedia(this, ::windows_core::from_raw_borrowed(&pmedia)).into())
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::mediaCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediacollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn playlistCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplaylistcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::playlistCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn versionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrversioninfo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::versionInfo(this, ::core::mem::transmute_copy(&pbstrversioninfo)).into())
        }
        unsafe extern "system" fn launchURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::launchURL(this, ::core::mem::transmute(&bstrurl)).into())
        }
        unsafe extern "system" fn network<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqni: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::network(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqni, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn currentPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::currentPlaylist(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcurrentPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcurrentPlaylist(this, ::windows_core::from_raw_borrowed(&ppl)).into())
        }
        unsafe extern "system" fn cdromCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcdromcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::cdromCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcdromcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn closedCaption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclosedcaption: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::closedCaption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclosedcaption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isOnline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfonline: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isOnline(this, ::core::mem::transmute_copy(&pfonline)).into())
        }
        unsafe extern "system" fn error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperror: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::status(this, ::core::mem::transmute_copy(&pbstrstatus)).into())
        }
        IWMPCore_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            close: close::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            openState: openState::<Identity, Impl, OFFSET>,
            playState: playState::<Identity, Impl, OFFSET>,
            controls: controls::<Identity, Impl, OFFSET>,
            settings: settings::<Identity, Impl, OFFSET>,
            currentMedia: currentMedia::<Identity, Impl, OFFSET>,
            SetcurrentMedia: SetcurrentMedia::<Identity, Impl, OFFSET>,
            mediaCollection: mediaCollection::<Identity, Impl, OFFSET>,
            playlistCollection: playlistCollection::<Identity, Impl, OFFSET>,
            versionInfo: versionInfo::<Identity, Impl, OFFSET>,
            launchURL: launchURL::<Identity, Impl, OFFSET>,
            network: network::<Identity, Impl, OFFSET>,
            currentPlaylist: currentPlaylist::<Identity, Impl, OFFSET>,
            SetcurrentPlaylist: SetcurrentPlaylist::<Identity, Impl, OFFSET>,
            cdromCollection: cdromCollection::<Identity, Impl, OFFSET>,
            closedCaption: closedCaption::<Identity, Impl, OFFSET>,
            isOnline: isOnline::<Identity, Impl, OFFSET>,
            error: error::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPCore2_Impl: ::windows_core::BaseImpl + IWMPCore_Impl {
    fn dvd(this: &Self::This) -> ::windows_core::Result<IWMPDVD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPCore2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCore2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn dvd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdvd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dvd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdvd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPCore2_Vtbl { base__: <IWMPCore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, dvd: dvd::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPCore3_Impl: ::windows_core::BaseImpl + IWMPCore2_Impl {
    fn newPlaylist(this: &Self::This, bstrname: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn newMedia(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<IWMPMedia>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPCore3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPCore3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn newPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::newPlaylist(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn newMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPCore3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::newMedia(this, ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmedia, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPCore3_Vtbl {
            base__: <IWMPCore2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            newPlaylist: newPlaylist::<Identity, Impl, OFFSET>,
            newMedia: newMedia::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPDVD_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(this: &Self::This, bstritem: &::windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn domain(this: &Self::This, strdomain: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn topMenu(this: &Self::This) -> ::windows_core::Result<()>;
    fn titleMenu(this: &Self::This) -> ::windows_core::Result<()>;
    fn back(this: &Self::This) -> ::windows_core::Result<()>;
    fn resume(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPDVD {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPDVD {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_isAvailable(this, ::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into())
        }
        unsafe extern "system" fn domain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::domain(this, ::core::mem::transmute_copy(&strdomain)).into())
        }
        unsafe extern "system" fn topMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::topMenu(this).into())
        }
        unsafe extern "system" fn titleMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::titleMenu(this).into())
        }
        unsafe extern "system" fn back<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::back(this).into())
        }
        unsafe extern "system" fn resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDVD_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::resume(this).into())
        }
        IWMPDVD_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
            domain: domain::<Identity, Impl, OFFSET>,
            topMenu: topMenu::<Identity, Impl, OFFSET>,
            titleMenu: titleMenu::<Identity, Impl, OFFSET>,
            back: back::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPDownloadCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn id(this: &Self::This, plid: *mut i32) -> ::windows_core::Result<()>;
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn item(this: &Self::This, litem: i32) -> ::windows_core::Result<IWMPDownloadItem2>;
    fn startDownload(this: &Self::This, bstrsourceurl: &::windows_core::BSTR, bstrtype: &::windows_core::BSTR) -> ::windows_core::Result<IWMPDownloadItem2>;
    fn removeItem(this: &Self::This, litem: i32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPDownloadCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPDownloadCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::id(this, ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, litem: i32, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::item(this, ::core::mem::transmute_copy(&litem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn startDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrsourceurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::startDownload(this, ::core::mem::transmute(&bstrsourceurl), ::core::mem::transmute(&bstrtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, litem: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeItem(this, ::core::mem::transmute_copy(&litem)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IWMPDownloadCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            id: id::<Identity, Impl, OFFSET>,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            startDownload: startDownload::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPDownloadItem_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn sourceURL(this: &Self::This, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn size(this: &Self::This, plsize: *mut i32) -> ::windows_core::Result<()>;
    fn r#type(this: &Self::This, pbstrtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn progress(this: &Self::This, plprogress: *mut i32) -> ::windows_core::Result<()>;
    fn downloadState(this: &Self::This, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows_core::Result<()>;
    fn pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPDownloadItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPDownloadItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn sourceURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::sourceURL(this, ::core::mem::transmute_copy(&pbstrurl)).into())
        }
        unsafe extern "system" fn size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::size(this, ::core::mem::transmute_copy(&plsize)).into())
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::r#type(this, ::core::mem::transmute_copy(&pbstrtype)).into())
        }
        unsafe extern "system" fn progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::progress(this, ::core::mem::transmute_copy(&plprogress)).into())
        }
        unsafe extern "system" fn downloadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpsdls: *mut WMPSubscriptionDownloadState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::downloadState(this, ::core::mem::transmute_copy(&pwmpsdls)).into())
        }
        unsafe extern "system" fn pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pause(this).into())
        }
        unsafe extern "system" fn resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::resume(this).into())
        }
        unsafe extern "system" fn cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::cancel(this).into())
        }
        IWMPDownloadItem_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            sourceURL: sourceURL::<Identity, Impl, OFFSET>,
            size: size::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            progress: progress::<Identity, Impl, OFFSET>,
            downloadState: downloadState::<Identity, Impl, OFFSET>,
            pause: pause::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            cancel: cancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPDownloadItem2_Impl: ::windows_core::BaseImpl + IWMPDownloadItem_Impl {
    fn getItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPDownloadItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPDownloadItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPDownloadItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        IWMPDownloadItem2_Vtbl { base__: <IWMPDownloadItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPDownloadManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn getDownloadCollection(this: &Self::This, lcollectionid: i32) -> ::windows_core::Result<IWMPDownloadCollection>;
    fn createDownloadCollection(this: &Self::This) -> ::windows_core::Result<IWMPDownloadCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPDownloadManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPDownloadManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getDownloadCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcollectionid: i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDownloadCollection(this, ::core::mem::transmute_copy(&lcollectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createDownloadCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPDownloadManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createDownloadCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPDownloadManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getDownloadCollection: getDownloadCollection::<Identity, Impl, OFFSET>,
            createDownloadCollection: createDownloadCollection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IWMPEffects_Impl: ::windows_core::BaseImpl {
    fn Render(this: &Self::This, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn MediaInfo(this: &Self::This, lchannelcount: i32, lsamplerate: i32, bstrtitle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetCapabilities(this: &Self::This, pdwcapabilities: *mut u32) -> ::windows_core::Result<()>;
    fn GetTitle(this: &Self::This, bstrtitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPresetTitle(this: &Self::This, npreset: i32, bstrpresettitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPresetCount(this: &Self::This, pnpresetcount: *mut i32) -> ::windows_core::Result<()>;
    fn SetCurrentPreset(this: &Self::This, npreset: i32) -> ::windows_core::Result<()>;
    fn GetCurrentPreset(this: &Self::This, pnpreset: *mut i32) -> ::windows_core::Result<()>;
    fn DisplayPropertyPage(this: &Self::This, hwndowner: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GoFullscreen(this: &Self::This, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RenderFullScreen(this: &Self::This, plevels: *mut TimedLevel) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IWMPEffects {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEffects {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Render<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel, hdc: super::super::Graphics::Gdi::HDC, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Render(this, ::core::mem::transmute_copy(&plevels), ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prc)).into())
        }
        unsafe extern "system" fn MediaInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lchannelcount: i32, lsamplerate: i32, bstrtitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaInfo(this, ::core::mem::transmute_copy(&lchannelcount), ::core::mem::transmute_copy(&lsamplerate), ::core::mem::transmute(&bstrtitle)).into())
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcapabilities: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::core::mem::transmute_copy(&pdwcapabilities)).into())
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTitle(this, ::core::mem::transmute_copy(&bstrtitle)).into())
        }
        unsafe extern "system" fn GetPresetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npreset: i32, bstrpresettitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresetTitle(this, ::core::mem::transmute_copy(&npreset), ::core::mem::transmute_copy(&bstrpresettitle)).into())
        }
        unsafe extern "system" fn GetPresetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnpresetcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresetCount(this, ::core::mem::transmute_copy(&pnpresetcount)).into())
        }
        unsafe extern "system" fn SetCurrentPreset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, npreset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentPreset(this, ::core::mem::transmute_copy(&npreset)).into())
        }
        unsafe extern "system" fn GetCurrentPreset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnpreset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPreset(this, ::core::mem::transmute_copy(&pnpreset)).into())
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayPropertyPage(this, ::core::mem::transmute_copy(&hwndowner)).into())
        }
        unsafe extern "system" fn GoFullscreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GoFullscreen(this, ::core::mem::transmute_copy(&ffullscreen)).into())
        }
        unsafe extern "system" fn RenderFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plevels: *mut TimedLevel) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderFullScreen(this, ::core::mem::transmute_copy(&plevels)).into())
        }
        IWMPEffects_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Render: Render::<Identity, Impl, OFFSET>,
            MediaInfo: MediaInfo::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            GetPresetTitle: GetPresetTitle::<Identity, Impl, OFFSET>,
            GetPresetCount: GetPresetCount::<Identity, Impl, OFFSET>,
            SetCurrentPreset: SetCurrentPreset::<Identity, Impl, OFFSET>,
            GetCurrentPreset: GetCurrentPreset::<Identity, Impl, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, Impl, OFFSET>,
            GoFullscreen: GoFullscreen::<Identity, Impl, OFFSET>,
            RenderFullScreen: RenderFullScreen::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IWMPEffects2_Impl: ::windows_core::BaseImpl + IWMPEffects_Impl {
    fn SetCore(this: &Self::This, pplayer: ::core::option::Option<&IWMPCore>) -> ::windows_core::Result<()>;
    fn Create(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn NotifyNewMedia(this: &Self::This, pmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn OnWindowMessage(this: &Self::This, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn RenderWindowed(this: &Self::This, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPEffects2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPEffects);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEffects2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplayer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCore(this, ::windows_core::from_raw_borrowed(&pplayer)).into())
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn NotifyNewMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyNewMedia(this, ::windows_core::from_raw_borrowed(&pmedia)).into())
        }
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresultparam: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowMessage(this, ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plresultparam)).into())
        }
        unsafe extern "system" fn RenderWindowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEffects2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *mut TimedLevel, frequiredrender: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderWindowed(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&frequiredrender)).into())
        }
        IWMPEffects2_Vtbl {
            base__: <IWMPEffects as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCore: SetCore::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            NotifyNewMedia: NotifyNewMedia::<Identity, Impl, OFFSET>,
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            RenderWindowed: RenderWindowed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPError_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn clearErrorQueue(this: &Self::This) -> ::windows_core::Result<()>;
    fn errorCount(this: &Self::This, plnumerrors: *mut i32) -> ::windows_core::Result<()>;
    fn get_item(this: &Self::This, dwindex: i32) -> ::windows_core::Result<IWMPErrorItem>;
    fn webHelp(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn clearErrorQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::clearErrorQueue(this).into())
        }
        unsafe extern "system" fn errorCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plnumerrors: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::errorCount(this, ::core::mem::transmute_copy(&plnumerrors)).into())
        }
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: i32, pperroritem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperroritem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn webHelp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::webHelp(this).into())
        }
        IWMPError_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            clearErrorQueue: clearErrorQueue::<Identity, Impl, OFFSET>,
            errorCount: errorCount::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            webHelp: webHelp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPErrorItem_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn errorCode(this: &Self::This, phr: *mut i32) -> ::windows_core::Result<()>;
    fn errorDescription(this: &Self::This, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn errorContext(this: &Self::This, pvarcontext: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn remedy(this: &Self::This, plremedy: *mut i32) -> ::windows_core::Result<()>;
    fn customUrl(this: &Self::This, pbstrcustomurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPErrorItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPErrorItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn errorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phr: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::errorCode(this, ::core::mem::transmute_copy(&phr)).into())
        }
        unsafe extern "system" fn errorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::errorDescription(this, ::core::mem::transmute_copy(&pbstrdescription)).into())
        }
        unsafe extern "system" fn errorContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcontext: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::errorContext(this, ::core::mem::transmute_copy(&pvarcontext)).into())
        }
        unsafe extern "system" fn remedy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plremedy: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remedy(this, ::core::mem::transmute_copy(&plremedy)).into())
        }
        unsafe extern "system" fn customUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcustomurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::customUrl(this, ::core::mem::transmute_copy(&pbstrcustomurl)).into())
        }
        IWMPErrorItem_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            errorDescription: errorDescription::<Identity, Impl, OFFSET>,
            errorContext: errorContext::<Identity, Impl, OFFSET>,
            remedy: remedy::<Identity, Impl, OFFSET>,
            customUrl: customUrl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPErrorItem2_Impl: ::windows_core::BaseImpl + IWMPErrorItem_Impl {
    fn condition(this: &Self::This, plcondition: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPErrorItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPErrorItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPErrorItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn condition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPErrorItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcondition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::condition(this, ::core::mem::transmute_copy(&plcondition)).into())
        }
        IWMPErrorItem2_Vtbl { base__: <IWMPErrorItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, condition: condition::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents_Impl: ::windows_core::BaseImpl {
    fn OpenStateChange(this: &Self::This, newstate: i32);
    fn PlayStateChange(this: &Self::This, newstate: i32);
    fn AudioLanguageChange(this: &Self::This, langid: i32);
    fn StatusChange(this: &Self::This);
    fn ScriptCommand(this: &Self::This, sctype: &::windows_core::BSTR, param: &::windows_core::BSTR);
    fn NewStream(this: &Self::This);
    fn Disconnect(this: &Self::This, result: i32);
    fn Buffering(this: &Self::This, start: super::super::Foundation::VARIANT_BOOL);
    fn Error(this: &Self::This);
    fn Warning(this: &Self::This, warningtype: i32, param: i32, description: &::windows_core::BSTR);
    fn EndOfStream(this: &Self::This, result: i32);
    fn PositionChange(this: &Self::This, oldposition: f64, newposition: f64);
    fn MarkerHit(this: &Self::This, markernum: i32);
    fn DurationUnitChange(this: &Self::This, newdurationunit: i32);
    fn CdromMediaChange(this: &Self::This, cdromnum: i32);
    fn PlaylistChange(this: &Self::This, playlist: ::core::option::Option<&super::super::System::Com::IDispatch>, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistChange(this: &Self::This, change: WMPPlaylistChangeEventType);
    fn CurrentPlaylistItemAvailable(this: &Self::This, bstritemname: &::windows_core::BSTR);
    fn MediaChange(this: &Self::This, item: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CurrentMediaItemAvailable(this: &Self::This, bstritemname: &::windows_core::BSTR);
    fn CurrentItemChange(this: &Self::This, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionChange(this: &Self::This);
    fn MediaCollectionAttributeStringAdded(this: &Self::This, bstrattribname: &::windows_core::BSTR, bstrattribval: &::windows_core::BSTR);
    fn MediaCollectionAttributeStringRemoved(this: &Self::This, bstrattribname: &::windows_core::BSTR, bstrattribval: &::windows_core::BSTR);
    fn MediaCollectionAttributeStringChanged(this: &Self::This, bstrattribname: &::windows_core::BSTR, bstroldattribval: &::windows_core::BSTR, bstrnewattribval: &::windows_core::BSTR);
    fn PlaylistCollectionChange(this: &Self::This);
    fn PlaylistCollectionPlaylistAdded(this: &Self::This, bstrplaylistname: &::windows_core::BSTR);
    fn PlaylistCollectionPlaylistRemoved(this: &Self::This, bstrplaylistname: &::windows_core::BSTR);
    fn PlaylistCollectionPlaylistSetAsDeleted(this: &Self::This, bstrplaylistname: &::windows_core::BSTR, varfisdeleted: super::super::Foundation::VARIANT_BOOL);
    fn ModeChange(this: &Self::This, modename: &::windows_core::BSTR, newvalue: super::super::Foundation::VARIANT_BOOL);
    fn MediaError(this: &Self::This, pmediaobject: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn OpenPlaylistSwitch(this: &Self::This, pitem: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn DomainChange(this: &Self::This, strdomain: &::windows_core::BSTR);
    fn SwitchedToPlayerApplication(this: &Self::This);
    fn SwitchedToControl(this: &Self::This);
    fn PlayerDockedStateChange(this: &Self::This);
    fn PlayerReconnect(this: &Self::This);
    fn Click(this: &Self::This, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn DoubleClick(this: &Self::This, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn KeyDown(this: &Self::This, nkeycode: i16, nshiftstate: i16);
    fn KeyPress(this: &Self::This, nkeyascii: i16);
    fn KeyUp(this: &Self::This, nkeycode: i16, nshiftstate: i16);
    fn MouseDown(this: &Self::This, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseMove(this: &Self::This, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
    fn MouseUp(this: &Self::This, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenStateChange(this, ::core::mem::transmute_copy(&newstate)))
        }
        unsafe extern "system" fn PlayStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayStateChange(this, ::core::mem::transmute_copy(&newstate)))
        }
        unsafe extern "system" fn AudioLanguageChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AudioLanguageChange(this, ::core::mem::transmute_copy(&langid)))
        }
        unsafe extern "system" fn StatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StatusChange(this))
        }
        unsafe extern "system" fn ScriptCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sctype: ::std::mem::MaybeUninit<::windows_core::BSTR>, param: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScriptCommand(this, ::core::mem::transmute(&sctype), ::core::mem::transmute(&param)))
        }
        unsafe extern "system" fn NewStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewStream(this))
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::core::mem::transmute_copy(&result)))
        }
        unsafe extern "system" fn Buffering<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, start: super::super::Foundation::VARIANT_BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Buffering(this, ::core::mem::transmute_copy(&start)))
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Error(this))
        }
        unsafe extern "system" fn Warning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, warningtype: i32, param: i32, description: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Warning(this, ::core::mem::transmute_copy(&warningtype), ::core::mem::transmute_copy(&param), ::core::mem::transmute(&description)))
        }
        unsafe extern "system" fn EndOfStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndOfStream(this, ::core::mem::transmute_copy(&result)))
        }
        unsafe extern "system" fn PositionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldposition: f64, newposition: f64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PositionChange(this, ::core::mem::transmute_copy(&oldposition), ::core::mem::transmute_copy(&newposition)))
        }
        unsafe extern "system" fn MarkerHit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, markernum: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarkerHit(this, ::core::mem::transmute_copy(&markernum)))
        }
        unsafe extern "system" fn DurationUnitChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newdurationunit: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DurationUnitChange(this, ::core::mem::transmute_copy(&newdurationunit)))
        }
        unsafe extern "system" fn CdromMediaChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdromnum: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromMediaChange(this, ::core::mem::transmute_copy(&cdromnum)))
        }
        unsafe extern "system" fn PlaylistChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, playlist: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlaylistChange(this, ::windows_core::from_raw_borrowed(&playlist), ::core::mem::transmute_copy(&change)))
        }
        unsafe extern "system" fn CurrentPlaylistChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, change: WMPPlaylistChangeEventType) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CurrentPlaylistChange(this, ::core::mem::transmute_copy(&change)))
        }
        unsafe extern "system" fn CurrentPlaylistItemAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CurrentPlaylistItemAvailable(this, ::core::mem::transmute(&bstritemname)))
        }
        unsafe extern "system" fn MediaChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaChange(this, ::windows_core::from_raw_borrowed(&item)))
        }
        unsafe extern "system" fn CurrentMediaItemAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CurrentMediaItemAvailable(this, ::core::mem::transmute(&bstritemname)))
        }
        unsafe extern "system" fn CurrentItemChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CurrentItemChange(this, ::windows_core::from_raw_borrowed(&pdispmedia)))
        }
        unsafe extern "system" fn MediaCollectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionChange(this))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionAttributeStringAdded(this, ::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstrattribval)))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionAttributeStringRemoved(this, ::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstrattribval)))
        }
        unsafe extern "system" fn MediaCollectionAttributeStringChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstroldattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewattribval: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionAttributeStringChanged(this, ::core::mem::transmute(&bstrattribname), ::core::mem::transmute(&bstroldattribval), ::core::mem::transmute(&bstrnewattribval)))
        }
        unsafe extern "system" fn PlaylistCollectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlaylistCollectionChange(this))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlaylistCollectionPlaylistAdded(this, ::core::mem::transmute(&bstrplaylistname)))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlaylistCollectionPlaylistRemoved(this, ::core::mem::transmute(&bstrplaylistname)))
        }
        unsafe extern "system" fn PlaylistCollectionPlaylistSetAsDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrplaylistname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlaylistCollectionPlaylistSetAsDeleted(this, ::core::mem::transmute(&bstrplaylistname), ::core::mem::transmute_copy(&varfisdeleted)))
        }
        unsafe extern "system" fn ModeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modename: ::std::mem::MaybeUninit<::windows_core::BSTR>, newvalue: super::super::Foundation::VARIANT_BOOL) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModeChange(this, ::core::mem::transmute(&modename), ::core::mem::transmute_copy(&newvalue)))
        }
        unsafe extern "system" fn MediaError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediaobject: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaError(this, ::windows_core::from_raw_borrowed(&pmediaobject)))
        }
        unsafe extern "system" fn OpenPlaylistSwitch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenPlaylistSwitch(this, ::windows_core::from_raw_borrowed(&pitem)))
        }
        unsafe extern "system" fn DomainChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DomainChange(this, ::core::mem::transmute(&strdomain)))
        }
        unsafe extern "system" fn SwitchedToPlayerApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchedToPlayerApplication(this))
        }
        unsafe extern "system" fn SwitchedToControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchedToControl(this))
        }
        unsafe extern "system" fn PlayerDockedStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayerDockedStateChange(this))
        }
        unsafe extern "system" fn PlayerReconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayerReconnect(this))
        }
        unsafe extern "system" fn Click<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Click(this, ::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy)))
        }
        unsafe extern "system" fn DoubleClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoubleClick(this, ::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy)))
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeyDown(this, ::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate)))
        }
        unsafe extern "system" fn KeyPress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nkeyascii: i16) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeyPress(this, ::core::mem::transmute_copy(&nkeyascii)))
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nkeycode: i16, nshiftstate: i16) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeyUp(this, ::core::mem::transmute_copy(&nkeycode), ::core::mem::transmute_copy(&nshiftstate)))
        }
        unsafe extern "system" fn MouseDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MouseDown(this, ::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy)))
        }
        unsafe extern "system" fn MouseMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MouseMove(this, ::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy)))
        }
        unsafe extern "system" fn MouseUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbutton: i16, nshiftstate: i16, fx: i32, fy: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MouseUp(this, ::core::mem::transmute_copy(&nbutton), ::core::mem::transmute_copy(&nshiftstate), ::core::mem::transmute_copy(&fx), ::core::mem::transmute_copy(&fy)))
        }
        IWMPEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenStateChange: OpenStateChange::<Identity, Impl, OFFSET>,
            PlayStateChange: PlayStateChange::<Identity, Impl, OFFSET>,
            AudioLanguageChange: AudioLanguageChange::<Identity, Impl, OFFSET>,
            StatusChange: StatusChange::<Identity, Impl, OFFSET>,
            ScriptCommand: ScriptCommand::<Identity, Impl, OFFSET>,
            NewStream: NewStream::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Buffering: Buffering::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            Warning: Warning::<Identity, Impl, OFFSET>,
            EndOfStream: EndOfStream::<Identity, Impl, OFFSET>,
            PositionChange: PositionChange::<Identity, Impl, OFFSET>,
            MarkerHit: MarkerHit::<Identity, Impl, OFFSET>,
            DurationUnitChange: DurationUnitChange::<Identity, Impl, OFFSET>,
            CdromMediaChange: CdromMediaChange::<Identity, Impl, OFFSET>,
            PlaylistChange: PlaylistChange::<Identity, Impl, OFFSET>,
            CurrentPlaylistChange: CurrentPlaylistChange::<Identity, Impl, OFFSET>,
            CurrentPlaylistItemAvailable: CurrentPlaylistItemAvailable::<Identity, Impl, OFFSET>,
            MediaChange: MediaChange::<Identity, Impl, OFFSET>,
            CurrentMediaItemAvailable: CurrentMediaItemAvailable::<Identity, Impl, OFFSET>,
            CurrentItemChange: CurrentItemChange::<Identity, Impl, OFFSET>,
            MediaCollectionChange: MediaCollectionChange::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringAdded: MediaCollectionAttributeStringAdded::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringRemoved: MediaCollectionAttributeStringRemoved::<Identity, Impl, OFFSET>,
            MediaCollectionAttributeStringChanged: MediaCollectionAttributeStringChanged::<Identity, Impl, OFFSET>,
            PlaylistCollectionChange: PlaylistCollectionChange::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistAdded: PlaylistCollectionPlaylistAdded::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistRemoved: PlaylistCollectionPlaylistRemoved::<Identity, Impl, OFFSET>,
            PlaylistCollectionPlaylistSetAsDeleted: PlaylistCollectionPlaylistSetAsDeleted::<Identity, Impl, OFFSET>,
            ModeChange: ModeChange::<Identity, Impl, OFFSET>,
            MediaError: MediaError::<Identity, Impl, OFFSET>,
            OpenPlaylistSwitch: OpenPlaylistSwitch::<Identity, Impl, OFFSET>,
            DomainChange: DomainChange::<Identity, Impl, OFFSET>,
            SwitchedToPlayerApplication: SwitchedToPlayerApplication::<Identity, Impl, OFFSET>,
            SwitchedToControl: SwitchedToControl::<Identity, Impl, OFFSET>,
            PlayerDockedStateChange: PlayerDockedStateChange::<Identity, Impl, OFFSET>,
            PlayerReconnect: PlayerReconnect::<Identity, Impl, OFFSET>,
            Click: Click::<Identity, Impl, OFFSET>,
            DoubleClick: DoubleClick::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            KeyPress: KeyPress::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            MouseDown: MouseDown::<Identity, Impl, OFFSET>,
            MouseMove: MouseMove::<Identity, Impl, OFFSET>,
            MouseUp: MouseUp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents2_Impl: ::windows_core::BaseImpl + IWMPEvents_Impl {
    fn DeviceConnect(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>);
    fn DeviceDisconnect(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>);
    fn DeviceStatusChange(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, newstatus: WMPDeviceStatus);
    fn DeviceSyncStateChange(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, newstate: WMPSyncState);
    fn DeviceSyncError(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CreatePartnershipComplete(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, hrresult: ::windows_core::HRESULT);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPEvents2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPEvents);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEvents2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceConnect(this, ::windows_core::from_raw_borrowed(&pdevice)))
        }
        unsafe extern "system" fn DeviceDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceDisconnect(this, ::windows_core::from_raw_borrowed(&pdevice)))
        }
        unsafe extern "system" fn DeviceStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstatus: WMPDeviceStatus) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceStatusChange(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&newstatus)))
        }
        unsafe extern "system" fn DeviceSyncStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, newstate: WMPSyncState) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSyncStateChange(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&newstate)))
        }
        unsafe extern "system" fn DeviceSyncError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSyncError(this, ::windows_core::from_raw_borrowed(&pdevice), ::windows_core::from_raw_borrowed(&pmedia)))
        }
        unsafe extern "system" fn CreatePartnershipComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePartnershipComplete(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hrresult)))
        }
        IWMPEvents2_Vtbl {
            base__: <IWMPEvents as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceConnect: DeviceConnect::<Identity, Impl, OFFSET>,
            DeviceDisconnect: DeviceDisconnect::<Identity, Impl, OFFSET>,
            DeviceStatusChange: DeviceStatusChange::<Identity, Impl, OFFSET>,
            DeviceSyncStateChange: DeviceSyncStateChange::<Identity, Impl, OFFSET>,
            DeviceSyncError: DeviceSyncError::<Identity, Impl, OFFSET>,
            CreatePartnershipComplete: CreatePartnershipComplete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents3_Impl: ::windows_core::BaseImpl + IWMPEvents2_Impl {
    fn CdromRipStateChange(this: &Self::This, pcdromrip: ::core::option::Option<&IWMPCdromRip>, wmprs: WMPRipState);
    fn CdromRipMediaError(this: &Self::This, pcdromrip: ::core::option::Option<&IWMPCdromRip>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnStateChange(this: &Self::This, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, wmpbs: WMPBurnState);
    fn CdromBurnMediaError(this: &Self::This, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, pmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn CdromBurnError(this: &Self::This, pcdromburn: ::core::option::Option<&IWMPCdromBurn>, hrerror: ::windows_core::HRESULT);
    fn LibraryConnect(this: &Self::This, plibrary: ::core::option::Option<&IWMPLibrary>);
    fn LibraryDisconnect(this: &Self::This, plibrary: ::core::option::Option<&IWMPLibrary>);
    fn FolderScanStateChange(this: &Self::This, wmpfss: WMPFolderScanState);
    fn StringCollectionChange(this: &Self::This, pdispstringcollection: ::core::option::Option<&super::super::System::Com::IDispatch>, change: WMPStringCollectionChangeEventType, lcollectionindex: i32);
    fn MediaCollectionMediaAdded(this: &Self::This, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
    fn MediaCollectionMediaRemoved(this: &Self::This, pdispmedia: ::core::option::Option<&super::super::System::Com::IDispatch>);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPEvents3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPEvents2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEvents3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CdromRipStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, wmprs: WMPRipState) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromRipStateChange(this, ::windows_core::from_raw_borrowed(&pcdromrip), ::core::mem::transmute_copy(&wmprs)))
        }
        unsafe extern "system" fn CdromRipMediaError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdromrip: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromRipMediaError(this, ::windows_core::from_raw_borrowed(&pcdromrip), ::windows_core::from_raw_borrowed(&pmedia)))
        }
        unsafe extern "system" fn CdromBurnStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, wmpbs: WMPBurnState) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromBurnStateChange(this, ::windows_core::from_raw_borrowed(&pcdromburn), ::core::mem::transmute_copy(&wmpbs)))
        }
        unsafe extern "system" fn CdromBurnMediaError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, pmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromBurnMediaError(this, ::windows_core::from_raw_borrowed(&pcdromburn), ::windows_core::from_raw_borrowed(&pmedia)))
        }
        unsafe extern "system" fn CdromBurnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdromburn: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CdromBurnError(this, ::windows_core::from_raw_borrowed(&pcdromburn), ::core::mem::transmute_copy(&hrerror)))
        }
        unsafe extern "system" fn LibraryConnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LibraryConnect(this, ::windows_core::from_raw_borrowed(&plibrary)))
        }
        unsafe extern "system" fn LibraryDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibrary: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LibraryDisconnect(this, ::windows_core::from_raw_borrowed(&plibrary)))
        }
        unsafe extern "system" fn FolderScanStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmpfss: WMPFolderScanState) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderScanStateChange(this, ::core::mem::transmute_copy(&wmpfss)))
        }
        unsafe extern "system" fn StringCollectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdispstringcollection: *mut ::core::ffi::c_void, change: WMPStringCollectionChangeEventType, lcollectionindex: i32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StringCollectionChange(this, ::windows_core::from_raw_borrowed(&pdispstringcollection), ::core::mem::transmute_copy(&change), ::core::mem::transmute_copy(&lcollectionindex)))
        }
        unsafe extern "system" fn MediaCollectionMediaAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionMediaAdded(this, ::windows_core::from_raw_borrowed(&pdispmedia)))
        }
        unsafe extern "system" fn MediaCollectionMediaRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdispmedia: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MediaCollectionMediaRemoved(this, ::windows_core::from_raw_borrowed(&pdispmedia)))
        }
        IWMPEvents3_Vtbl {
            base__: <IWMPEvents2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CdromRipStateChange: CdromRipStateChange::<Identity, Impl, OFFSET>,
            CdromRipMediaError: CdromRipMediaError::<Identity, Impl, OFFSET>,
            CdromBurnStateChange: CdromBurnStateChange::<Identity, Impl, OFFSET>,
            CdromBurnMediaError: CdromBurnMediaError::<Identity, Impl, OFFSET>,
            CdromBurnError: CdromBurnError::<Identity, Impl, OFFSET>,
            LibraryConnect: LibraryConnect::<Identity, Impl, OFFSET>,
            LibraryDisconnect: LibraryDisconnect::<Identity, Impl, OFFSET>,
            FolderScanStateChange: FolderScanStateChange::<Identity, Impl, OFFSET>,
            StringCollectionChange: StringCollectionChange::<Identity, Impl, OFFSET>,
            MediaCollectionMediaAdded: MediaCollectionMediaAdded::<Identity, Impl, OFFSET>,
            MediaCollectionMediaRemoved: MediaCollectionMediaRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPEvents4_Impl: ::windows_core::BaseImpl + IWMPEvents3_Impl {
    fn DeviceEstimation(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, hrresult: ::windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPEvents4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPEvents3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPEvents4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceEstimation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPEvents4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT, qwestimatedusedspace: i64, qwestimatedspace: i64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceEstimation(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hrresult), ::core::mem::transmute_copy(&qwestimatedusedspace), ::core::mem::transmute_copy(&qwestimatedspace)))
        }
        IWMPEvents4_Vtbl { base__: <IWMPEvents3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DeviceEstimation: DeviceEstimation::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMPFolderMonitorServices_Impl: ::windows_core::BaseImpl {
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn item(this: &Self::This, lindex: i32, pbstrfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn add(this: &Self::This, bstrfolder: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn remove(this: &Self::This, lindex: i32) -> ::windows_core::Result<()>;
    fn scanState(this: &Self::This, pwmpfss: *mut WMPFolderScanState) -> ::windows_core::Result<()>;
    fn currentFolder(this: &Self::This, pbstrfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn scannedFilesCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn addedFilesCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn updateProgress(this: &Self::This, plprogress: *mut i32) -> ::windows_core::Result<()>;
    fn startScan(this: &Self::This) -> ::windows_core::Result<()>;
    fn stopScan(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPFolderMonitorServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPFolderMonitorServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::item(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrfolder)).into())
        }
        unsafe extern "system" fn add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfolder: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::add(this, ::core::mem::transmute(&bstrfolder)).into())
        }
        unsafe extern "system" fn remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remove(this, ::core::mem::transmute_copy(&lindex)).into())
        }
        unsafe extern "system" fn scanState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpfss: *mut WMPFolderScanState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::scanState(this, ::core::mem::transmute_copy(&pwmpfss)).into())
        }
        unsafe extern "system" fn currentFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::currentFolder(this, ::core::mem::transmute_copy(&pbstrfolder)).into())
        }
        unsafe extern "system" fn scannedFilesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::scannedFilesCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn addedFilesCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addedFilesCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn updateProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::updateProgress(this, ::core::mem::transmute_copy(&plprogress)).into())
        }
        unsafe extern "system" fn startScan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startScan(this).into())
        }
        unsafe extern "system" fn stopScan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPFolderMonitorServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stopScan(this).into())
        }
        IWMPFolderMonitorServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
            add: add::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            scanState: scanState::<Identity, Impl, OFFSET>,
            currentFolder: currentFolder::<Identity, Impl, OFFSET>,
            scannedFilesCount: scannedFilesCount::<Identity, Impl, OFFSET>,
            addedFilesCount: addedFilesCount::<Identity, Impl, OFFSET>,
            updateProgress: updateProgress::<Identity, Impl, OFFSET>,
            startScan: startScan::<Identity, Impl, OFFSET>,
            stopScan: stopScan::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPGraphCreation_Impl: ::windows_core::BaseImpl {
    fn GraphCreationPreRender(this: &Self::This, pfiltergraph: ::core::option::Option<&::windows_core::IUnknown>, preserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GraphCreationPostRender(this: &Self::This, pfiltergraph: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetGraphCreationFlags(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPGraphCreation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPGraphCreation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GraphCreationPreRender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GraphCreationPreRender(this, ::windows_core::from_raw_borrowed(&pfiltergraph), ::windows_core::from_raw_borrowed(&preserved)).into())
        }
        unsafe extern "system" fn GraphCreationPostRender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiltergraph: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GraphCreationPostRender(this, ::windows_core::from_raw_borrowed(&pfiltergraph)).into())
        }
        unsafe extern "system" fn GetGraphCreationFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPGraphCreation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGraphCreationFlags(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        IWMPGraphCreation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GraphCreationPreRender: GraphCreationPreRender::<Identity, Impl, OFFSET>,
            GraphCreationPostRender: GraphCreationPostRender::<Identity, Impl, OFFSET>,
            GetGraphCreationFlags: GetGraphCreationFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary_Impl: ::windows_core::BaseImpl {
    fn name(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn r#type(this: &Self::This, pwmplt: *mut WMPLibraryType) -> ::windows_core::Result<()>;
    fn mediaCollection(this: &Self::This) -> ::windows_core::Result<IWMPMediaCollection>;
    fn isIdentical(this: &Self::This, piwmplibrary: ::core::option::Option<&IWMPLibrary>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::name(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmplt: *mut WMPLibraryType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::r#type(this, ::core::mem::transmute_copy(&pwmplt)).into())
        }
        unsafe extern "system" fn mediaCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwmpmediacollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::mediaCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmediacollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmplibrary: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isIdentical(this, ::windows_core::from_raw_borrowed(&piwmplibrary), ::core::mem::transmute_copy(&pvbool)).into())
        }
        IWMPLibrary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            name: name::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            mediaCollection: mediaCollection::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPLibrary2_Impl: ::windows_core::BaseImpl + IWMPLibrary_Impl {
    fn getItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPLibrary2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPLibrary);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPLibrary2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrary2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        IWMPLibrary2_Vtbl { base__: <IWMPLibrary as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, getItemInfo: getItemInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMPLibraryServices_Impl: ::windows_core::BaseImpl {
    fn getCountByType(this: &Self::This, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getLibraryByType(this: &Self::This, wmplt: WMPLibraryType, lindex: i32) -> ::windows_core::Result<IWMPLibrary>;
}
impl ::windows_core::Iids for IWMPLibraryServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPLibraryServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getCountByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getCountByType(this, ::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getLibraryByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibraryServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmplt: WMPLibraryType, lindex: i32, ppiwmplibrary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getLibraryByType(this, ::core::mem::transmute_copy(&wmplt), ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmplibrary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPLibraryServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getCountByType: getCountByType::<Identity, Impl, OFFSET>,
            getLibraryByType: getLibraryByType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPLibrarySharingServices_Impl: ::windows_core::BaseImpl {
    fn isLibraryShared(this: &Self::This, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn isLibrarySharingEnabled(this: &Self::This, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn showLibrarySharing(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPLibrarySharingServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPLibrarySharingServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn isLibraryShared<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbshared: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isLibraryShared(this, ::core::mem::transmute_copy(&pvbshared)).into())
        }
        unsafe extern "system" fn isLibrarySharingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isLibrarySharingEnabled(this, ::core::mem::transmute_copy(&pvbenabled)).into())
        }
        unsafe extern "system" fn showLibrarySharing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::showLibrarySharing(this).into())
        }
        IWMPLibrarySharingServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            isLibraryShared: isLibraryShared::<Identity, Impl, OFFSET>,
            isLibrarySharingEnabled: isLibrarySharingEnabled::<Identity, Impl, OFFSET>,
            showLibrarySharing: showLibrarySharing::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMedia_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_isIdentical(this: &Self::This, piwmpmedia: ::core::option::Option<&IWMPMedia>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn sourceURL(this: &Self::This, pbstrsourceurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn name(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Setname(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn imageSourceWidth(this: &Self::This, pwidth: *mut i32) -> ::windows_core::Result<()>;
    fn imageSourceHeight(this: &Self::This, pheight: *mut i32) -> ::windows_core::Result<()>;
    fn markerCount(this: &Self::This, pmarkercount: *mut i32) -> ::windows_core::Result<()>;
    fn getMarkerTime(this: &Self::This, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::Result<()>;
    fn getMarkerName(this: &Self::This, markernum: i32, pbstrmarkername: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn duration(this: &Self::This, pduration: *mut f64) -> ::windows_core::Result<()>;
    fn durationString(this: &Self::This, pbstrduration: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn attributeCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getAttributeName(this: &Self::This, lindex: i32, pbstritemname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getItemInfoByAtom(this: &Self::This, latom: i32, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn isMemberOf(this: &Self::This, pplaylist: ::core::option::Option<&IWMPPlaylist>, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn isReadOnlyItem(this: &Self::This, bstritemname: &::windows_core::BSTR, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMedia {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMedia {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_isIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_isIdentical(this, ::windows_core::from_raw_borrowed(&piwmpmedia), ::core::mem::transmute_copy(&pvbool)).into())
        }
        unsafe extern "system" fn sourceURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsourceurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::sourceURL(this, ::core::mem::transmute_copy(&pbstrsourceurl)).into())
        }
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::name(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn Setname<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setname(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn imageSourceWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::imageSourceWidth(this, ::core::mem::transmute_copy(&pwidth)).into())
        }
        unsafe extern "system" fn imageSourceHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::imageSourceHeight(this, ::core::mem::transmute_copy(&pheight)).into())
        }
        unsafe extern "system" fn markerCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmarkercount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::markerCount(this, ::core::mem::transmute_copy(&pmarkercount)).into())
        }
        unsafe extern "system" fn getMarkerTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, markernum: i32, pmarkertime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getMarkerTime(this, ::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pmarkertime)).into())
        }
        unsafe extern "system" fn getMarkerName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, markernum: i32, pbstrmarkername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getMarkerName(this, ::core::mem::transmute_copy(&markernum), ::core::mem::transmute_copy(&pbstrmarkername)).into())
        }
        unsafe extern "system" fn duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pduration: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::duration(this, ::core::mem::transmute_copy(&pduration)).into())
        }
        unsafe extern "system" fn durationString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrduration: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::durationString(this, ::core::mem::transmute_copy(&pbstrduration)).into())
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::attributeCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getAttributeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstritemname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getAttributeName(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstritemname)).into())
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrval)).into())
        }
        unsafe extern "system" fn getItemInfoByAtom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, latom: i32, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfoByAtom(this, ::core::mem::transmute_copy(&latom), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        unsafe extern "system" fn isMemberOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplaylist: *mut ::core::ffi::c_void, pvarfismemberof: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isMemberOf(this, ::windows_core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pvarfismemberof)).into())
        }
        unsafe extern "system" fn isReadOnlyItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarfisreadonly: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isReadOnlyItem(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pvarfisreadonly)).into())
        }
        IWMPMedia_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_isIdentical: get_isIdentical::<Identity, Impl, OFFSET>,
            sourceURL: sourceURL::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Setname: Setname::<Identity, Impl, OFFSET>,
            imageSourceWidth: imageSourceWidth::<Identity, Impl, OFFSET>,
            imageSourceHeight: imageSourceHeight::<Identity, Impl, OFFSET>,
            markerCount: markerCount::<Identity, Impl, OFFSET>,
            getMarkerTime: getMarkerTime::<Identity, Impl, OFFSET>,
            getMarkerName: getMarkerName::<Identity, Impl, OFFSET>,
            duration: duration::<Identity, Impl, OFFSET>,
            durationString: durationString::<Identity, Impl, OFFSET>,
            attributeCount: attributeCount::<Identity, Impl, OFFSET>,
            getAttributeName: getAttributeName::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            setItemInfo: setItemInfo::<Identity, Impl, OFFSET>,
            getItemInfoByAtom: getItemInfoByAtom::<Identity, Impl, OFFSET>,
            isMemberOf: isMemberOf::<Identity, Impl, OFFSET>,
            isReadOnlyItem: isReadOnlyItem::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMedia2_Impl: ::windows_core::BaseImpl + IWMPMedia_Impl {
    fn error(this: &Self::This) -> ::windows_core::Result<IWMPErrorItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMedia2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPMedia);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMedia2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwmperroritem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmperroritem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPMedia2_Vtbl { base__: <IWMPMedia as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, error: error::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMedia3_Impl: ::windows_core::BaseImpl + IWMPMedia2_Impl {
    fn getAttributeCountByType(this: &Self::This, bstrtype: &::windows_core::BSTR, bstrlanguage: &::windows_core::BSTR, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getItemInfoByType(this: &Self::This, bstrtype: &::windows_core::BSTR, bstrlanguage: &::windows_core::BSTR, lindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMedia3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPMedia2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMedia3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getAttributeCountByType(this, ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMedia3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfoByType(this, ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        IWMPMedia3_Vtbl {
            base__: <IWMPMedia2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMediaCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn add(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<IWMPMedia>;
    fn getAll(this: &Self::This) -> ::windows_core::Result<IWMPPlaylist>;
    fn getByName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn getByGenre(this: &Self::This, bstrgenre: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn getByAuthor(this: &Self::This, bstrauthor: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn getByAlbum(this: &Self::This, bstralbum: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn getByAttribute(this: &Self::This, bstrattribute: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn remove(this: &Self::This, pitem: ::core::option::Option<&IWMPMedia>, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getAttributeStringCollection(this: &Self::This, bstrattribute: &::windows_core::BSTR, bstrmediatype: &::windows_core::BSTR) -> ::windows_core::Result<IWMPStringCollection>;
    fn getMediaAtom(this: &Self::This, bstritemname: &::windows_core::BSTR, platom: *mut i32) -> ::windows_core::Result<()>;
    fn setDeleted(this: &Self::This, pitem: ::core::option::Option<&IWMPMedia>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn isDeleted(this: &Self::This, pitem: ::core::option::Option<&IWMPMedia>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMediaCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMediaCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::add(this, ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAll(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByName(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByGenre<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgenre: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByGenre(this, ::core::mem::transmute(&bstrgenre)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByAuthor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrauthor: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByAuthor(this, ::core::mem::transmute(&bstrauthor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByAlbum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstralbum: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByAlbum(this, ::core::mem::transmute(&bstralbum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByAttribute(this, ::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfdeletefile: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remove(this, ::windows_core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfdeletefile)).into())
        }
        unsafe extern "system" fn getAttributeStringCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttributeStringCollection(this, ::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstringcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getMediaAtom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, platom: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getMediaAtom(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&platom)).into())
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setDeleted(this, ::windows_core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into())
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isDeleted(this, ::windows_core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into())
        }
        IWMPMediaCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            add: add::<Identity, Impl, OFFSET>,
            getAll: getAll::<Identity, Impl, OFFSET>,
            getByName: getByName::<Identity, Impl, OFFSET>,
            getByGenre: getByGenre::<Identity, Impl, OFFSET>,
            getByAuthor: getByAuthor::<Identity, Impl, OFFSET>,
            getByAlbum: getByAlbum::<Identity, Impl, OFFSET>,
            getByAttribute: getByAttribute::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            getAttributeStringCollection: getAttributeStringCollection::<Identity, Impl, OFFSET>,
            getMediaAtom: getMediaAtom::<Identity, Impl, OFFSET>,
            setDeleted: setDeleted::<Identity, Impl, OFFSET>,
            isDeleted: isDeleted::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMediaCollection2_Impl: ::windows_core::BaseImpl + IWMPMediaCollection_Impl {
    fn createQuery(this: &Self::This) -> ::windows_core::Result<IWMPQuery>;
    fn getPlaylistByQuery(this: &Self::This, pquery: ::core::option::Option<&IWMPQuery>, bstrmediatype: &::windows_core::BSTR, bstrsortattribute: &::windows_core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IWMPPlaylist>;
    fn getStringCollectionByQuery(this: &Self::This, bstrattribute: &::windows_core::BSTR, pquery: ::core::option::Option<&IWMPQuery>, bstrmediatype: &::windows_core::BSTR, bstrsortattribute: &::windows_core::BSTR, fsortascending: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IWMPStringCollection>;
    fn getByAttributeAndMediaType(this: &Self::This, bstrattribute: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR, bstrmediatype: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMediaCollection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPMediaCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMediaCollection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn createQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getPlaylistByQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppplaylist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getPlaylistByQuery(this, ::windows_core::from_raw_borrowed(&pquery), ::core::mem::transmute(&bstrmediatype), ::core::mem::transmute(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getStringCollectionByQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, pquery: *mut ::core::ffi::c_void, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsortattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, fsortascending: super::super::Foundation::VARIANT_BOOL, ppstringcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getStringCollectionByQuery(this, ::core::mem::transmute(&bstrattribute), ::windows_core::from_raw_borrowed(&pquery), ::core::mem::transmute(&bstrmediatype), ::core::mem::transmute(&bstrsortattribute), ::core::mem::transmute_copy(&fsortascending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstringcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByAttributeAndMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmediatype: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmediaitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByAttributeAndMediaType(this, ::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstrvalue), ::core::mem::transmute(&bstrmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmediaitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPMediaCollection2_Vtbl {
            base__: <IWMPMediaCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            createQuery: createQuery::<Identity, Impl, OFFSET>,
            getPlaylistByQuery: getPlaylistByQuery::<Identity, Impl, OFFSET>,
            getStringCollectionByQuery: getStringCollectionByQuery::<Identity, Impl, OFFSET>,
            getByAttributeAndMediaType: getByAttributeAndMediaType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPMediaPluginRegistrar_Impl: ::windows_core::BaseImpl {
    fn WMPRegisterPlayerPlugin(this: &Self::This, pwszfriendlyname: &::windows_core::PCWSTR, pwszdescription: &::windows_core::PCWSTR, pwszuninstallstring: &::windows_core::PCWSTR, dwpriority: u32, guidplugintype: &::windows_core::GUID, clsid: &::windows_core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn WMPUnRegisterPlayerPlugin(this: &Self::This, guidplugintype: &::windows_core::GUID, clsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPMediaPluginRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMediaPluginRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WMPRegisterPlayerPlugin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: ::windows_core::PCWSTR, pwszdescription: ::windows_core::PCWSTR, pwszuninstallstring: ::windows_core::PCWSTR, dwpriority: u32, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID, cmediatypes: u32, pmediatypes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMPRegisterPlayerPlugin(this, ::core::mem::transmute(&pwszfriendlyname), ::core::mem::transmute(&pwszdescription), ::core::mem::transmute(&pwszuninstallstring), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute(&guidplugintype), ::core::mem::transmute(&clsid), ::core::mem::transmute_copy(&cmediatypes), ::core::mem::transmute_copy(&pmediatypes)).into())
        }
        unsafe extern "system" fn WMPUnRegisterPlayerPlugin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMediaPluginRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidplugintype: ::windows_core::GUID, clsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WMPUnRegisterPlayerPlugin(this, ::core::mem::transmute(&guidplugintype), ::core::mem::transmute(&clsid)).into())
        }
        IWMPMediaPluginRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WMPRegisterPlayerPlugin: WMPRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
            WMPUnRegisterPlayerPlugin: WMPUnRegisterPlayerPlugin::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMetadataPicture_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn mimeType(this: &Self::This, pbstrmimetype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn pictureType(this: &Self::This, pbstrpicturetype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn description(this: &Self::This, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn URL(this: &Self::This, pbstrurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMetadataPicture {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMetadataPicture {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn mimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmimetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::mimeType(this, ::core::mem::transmute_copy(&pbstrmimetype)).into())
        }
        unsafe extern "system" fn pictureType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpicturetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pictureType(this, ::core::mem::transmute_copy(&pbstrpicturetype)).into())
        }
        unsafe extern "system" fn description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::description(this, ::core::mem::transmute_copy(&pbstrdescription)).into())
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::URL(this, ::core::mem::transmute_copy(&pbstrurl)).into())
        }
        IWMPMetadataPicture_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            pictureType: pictureType::<Identity, Impl, OFFSET>,
            description: description::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPMetadataText_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn description(this: &Self::This, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn text(this: &Self::This, pbstrtext: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPMetadataText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPMetadataText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::description(this, ::core::mem::transmute_copy(&pbstrdescription)).into())
        }
        unsafe extern "system" fn text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPMetadataText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::text(this, ::core::mem::transmute_copy(&pbstrtext)).into())
        }
        IWMPMetadataText_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            description: description::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPNetwork_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn bandWidth(this: &Self::This, plbandwidth: *mut i32) -> ::windows_core::Result<()>;
    fn recoveredPackets(this: &Self::This, plrecoveredpackets: *mut i32) -> ::windows_core::Result<()>;
    fn sourceProtocol(this: &Self::This, pbstrsourceprotocol: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn receivedPackets(this: &Self::This, plreceivedpackets: *mut i32) -> ::windows_core::Result<()>;
    fn lostPackets(this: &Self::This, pllostpackets: *mut i32) -> ::windows_core::Result<()>;
    fn receptionQuality(this: &Self::This, plreceptionquality: *mut i32) -> ::windows_core::Result<()>;
    fn bufferingCount(this: &Self::This, plbufferingcount: *mut i32) -> ::windows_core::Result<()>;
    fn bufferingProgress(this: &Self::This, plbufferingprogress: *mut i32) -> ::windows_core::Result<()>;
    fn bufferingTime(this: &Self::This, plbufferingtime: *mut i32) -> ::windows_core::Result<()>;
    fn SetbufferingTime(this: &Self::This, lbufferingtime: i32) -> ::windows_core::Result<()>;
    fn frameRate(this: &Self::This, plframerate: *mut i32) -> ::windows_core::Result<()>;
    fn maxBitRate(this: &Self::This, plbitrate: *mut i32) -> ::windows_core::Result<()>;
    fn bitRate(this: &Self::This, plbitrate: *mut i32) -> ::windows_core::Result<()>;
    fn getProxySettings(this: &Self::This, bstrprotocol: &::windows_core::BSTR, plproxysetting: *mut i32) -> ::windows_core::Result<()>;
    fn setProxySettings(this: &Self::This, bstrprotocol: &::windows_core::BSTR, lproxysetting: i32) -> ::windows_core::Result<()>;
    fn getProxyName(this: &Self::This, bstrprotocol: &::windows_core::BSTR, pbstrproxyname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setProxyName(this: &Self::This, bstrprotocol: &::windows_core::BSTR, bstrproxyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getProxyPort(this: &Self::This, bstrprotocol: &::windows_core::BSTR, lproxyport: *mut i32) -> ::windows_core::Result<()>;
    fn setProxyPort(this: &Self::This, bstrprotocol: &::windows_core::BSTR, lproxyport: i32) -> ::windows_core::Result<()>;
    fn getProxyExceptionList(this: &Self::This, bstrprotocol: &::windows_core::BSTR, pbstrexceptionlist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setProxyExceptionList(this: &Self::This, bstrprotocol: &::windows_core::BSTR, pbstrexceptionlist: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getProxyBypassForLocal(this: &Self::This, bstrprotocol: &::windows_core::BSTR, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setProxyBypassForLocal(this: &Self::This, bstrprotocol: &::windows_core::BSTR, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn maxBandwidth(this: &Self::This, lmaxbandwidth: *mut i32) -> ::windows_core::Result<()>;
    fn SetmaxBandwidth(this: &Self::This, lmaxbandwidth: i32) -> ::windows_core::Result<()>;
    fn downloadProgress(this: &Self::This, pldownloadprogress: *mut i32) -> ::windows_core::Result<()>;
    fn encodedFrameRate(this: &Self::This, plframerate: *mut i32) -> ::windows_core::Result<()>;
    fn framesSkipped(this: &Self::This, plframes: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPNetwork {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNetwork {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn bandWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbandwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::bandWidth(this, ::core::mem::transmute_copy(&plbandwidth)).into())
        }
        unsafe extern "system" fn recoveredPackets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plrecoveredpackets: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::recoveredPackets(this, ::core::mem::transmute_copy(&plrecoveredpackets)).into())
        }
        unsafe extern "system" fn sourceProtocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsourceprotocol: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::sourceProtocol(this, ::core::mem::transmute_copy(&pbstrsourceprotocol)).into())
        }
        unsafe extern "system" fn receivedPackets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plreceivedpackets: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::receivedPackets(this, ::core::mem::transmute_copy(&plreceivedpackets)).into())
        }
        unsafe extern "system" fn lostPackets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllostpackets: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::lostPackets(this, ::core::mem::transmute_copy(&pllostpackets)).into())
        }
        unsafe extern "system" fn receptionQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plreceptionquality: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::receptionQuality(this, ::core::mem::transmute_copy(&plreceptionquality)).into())
        }
        unsafe extern "system" fn bufferingCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbufferingcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::bufferingCount(this, ::core::mem::transmute_copy(&plbufferingcount)).into())
        }
        unsafe extern "system" fn bufferingProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbufferingprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::bufferingProgress(this, ::core::mem::transmute_copy(&plbufferingprogress)).into())
        }
        unsafe extern "system" fn bufferingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbufferingtime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::bufferingTime(this, ::core::mem::transmute_copy(&plbufferingtime)).into())
        }
        unsafe extern "system" fn SetbufferingTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbufferingtime: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetbufferingTime(this, ::core::mem::transmute_copy(&lbufferingtime)).into())
        }
        unsafe extern "system" fn frameRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::frameRate(this, ::core::mem::transmute_copy(&plframerate)).into())
        }
        unsafe extern "system" fn maxBitRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::maxBitRate(this, ::core::mem::transmute_copy(&plbitrate)).into())
        }
        unsafe extern "system" fn bitRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbitrate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::bitRate(this, ::core::mem::transmute_copy(&plbitrate)).into())
        }
        unsafe extern "system" fn getProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, plproxysetting: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getProxySettings(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&plproxysetting)).into())
        }
        unsafe extern "system" fn setProxySettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxysetting: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxySettings(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxysetting)).into())
        }
        unsafe extern "system" fn getProxyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproxyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getProxyName(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pbstrproxyname)).into())
        }
        unsafe extern "system" fn setProxyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrproxyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxyName(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrproxyname)).into())
        }
        unsafe extern "system" fn getProxyPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxyport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getProxyPort(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into())
        }
        unsafe extern "system" fn setProxyPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, lproxyport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxyPort(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&lproxyport)).into())
        }
        unsafe extern "system" fn getProxyExceptionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrexceptionlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getProxyExceptionList(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pbstrexceptionlist)).into())
        }
        unsafe extern "system" fn setProxyExceptionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrexceptionlist: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxyExceptionList(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&pbstrexceptionlist)).into())
        }
        unsafe extern "system" fn getProxyBypassForLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfbypassforlocal: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getProxyBypassForLocal(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&pfbypassforlocal)).into())
        }
        unsafe extern "system" fn setProxyBypassForLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows_core::BSTR>, fbypassforlocal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxyBypassForLocal(this, ::core::mem::transmute(&bstrprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into())
        }
        unsafe extern "system" fn maxBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::maxBandwidth(this, ::core::mem::transmute_copy(&lmaxbandwidth)).into())
        }
        unsafe extern "system" fn SetmaxBandwidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxbandwidth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetmaxBandwidth(this, ::core::mem::transmute_copy(&lmaxbandwidth)).into())
        }
        unsafe extern "system" fn downloadProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldownloadprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::downloadProgress(this, ::core::mem::transmute_copy(&pldownloadprogress)).into())
        }
        unsafe extern "system" fn encodedFrameRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plframerate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::encodedFrameRate(this, ::core::mem::transmute_copy(&plframerate)).into())
        }
        unsafe extern "system" fn framesSkipped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNetwork_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plframes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::framesSkipped(this, ::core::mem::transmute_copy(&plframes)).into())
        }
        IWMPNetwork_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            bandWidth: bandWidth::<Identity, Impl, OFFSET>,
            recoveredPackets: recoveredPackets::<Identity, Impl, OFFSET>,
            sourceProtocol: sourceProtocol::<Identity, Impl, OFFSET>,
            receivedPackets: receivedPackets::<Identity, Impl, OFFSET>,
            lostPackets: lostPackets::<Identity, Impl, OFFSET>,
            receptionQuality: receptionQuality::<Identity, Impl, OFFSET>,
            bufferingCount: bufferingCount::<Identity, Impl, OFFSET>,
            bufferingProgress: bufferingProgress::<Identity, Impl, OFFSET>,
            bufferingTime: bufferingTime::<Identity, Impl, OFFSET>,
            SetbufferingTime: SetbufferingTime::<Identity, Impl, OFFSET>,
            frameRate: frameRate::<Identity, Impl, OFFSET>,
            maxBitRate: maxBitRate::<Identity, Impl, OFFSET>,
            bitRate: bitRate::<Identity, Impl, OFFSET>,
            getProxySettings: getProxySettings::<Identity, Impl, OFFSET>,
            setProxySettings: setProxySettings::<Identity, Impl, OFFSET>,
            getProxyName: getProxyName::<Identity, Impl, OFFSET>,
            setProxyName: setProxyName::<Identity, Impl, OFFSET>,
            getProxyPort: getProxyPort::<Identity, Impl, OFFSET>,
            setProxyPort: setProxyPort::<Identity, Impl, OFFSET>,
            getProxyExceptionList: getProxyExceptionList::<Identity, Impl, OFFSET>,
            setProxyExceptionList: setProxyExceptionList::<Identity, Impl, OFFSET>,
            getProxyBypassForLocal: getProxyBypassForLocal::<Identity, Impl, OFFSET>,
            setProxyBypassForLocal: setProxyBypassForLocal::<Identity, Impl, OFFSET>,
            maxBandwidth: maxBandwidth::<Identity, Impl, OFFSET>,
            SetmaxBandwidth: SetmaxBandwidth::<Identity, Impl, OFFSET>,
            downloadProgress: downloadProgress::<Identity, Impl, OFFSET>,
            encodedFrameRate: encodedFrameRate::<Identity, Impl, OFFSET>,
            framesSkipped: framesSkipped::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstate_Impl: ::windows_core::BaseImpl {
    fn GetDesiredSize(this: &Self::This, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn SetRects(this: &Self::This, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetRects(this: &Self::This, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetWindowless(this: &Self::This, fwindowless: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetWindowless(this: &Self::This, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetFullScreen(this: &Self::This, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFullScreen(this: &Self::This, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPNodeRealEstate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeRealEstate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDesiredSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesiredSize(this, ::core::mem::transmute_copy(&psize)).into())
        }
        unsafe extern "system" fn SetRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrc: *const super::super::Foundation::RECT, pdest: *const super::super::Foundation::RECT, pclip: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRects(this, ::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into())
        }
        unsafe extern "system" fn GetRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrc: *mut super::super::Foundation::RECT, pdest: *mut super::super::Foundation::RECT, pclip: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRects(this, ::core::mem::transmute_copy(&psrc), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pclip)).into())
        }
        unsafe extern "system" fn SetWindowless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fwindowless: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWindowless(this, ::core::mem::transmute_copy(&fwindowless)).into())
        }
        unsafe extern "system" fn GetWindowless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfwindowless: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWindowless(this, ::core::mem::transmute_copy(&pfwindowless)).into())
        }
        unsafe extern "system" fn SetFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFullScreen(this, ::core::mem::transmute_copy(&ffullscreen)).into())
        }
        unsafe extern "system" fn GetFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFullScreen(this, ::core::mem::transmute_copy(&pffullscreen)).into())
        }
        IWMPNodeRealEstate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDesiredSize: GetDesiredSize::<Identity, Impl, OFFSET>,
            SetRects: SetRects::<Identity, Impl, OFFSET>,
            GetRects: GetRects::<Identity, Impl, OFFSET>,
            SetWindowless: SetWindowless::<Identity, Impl, OFFSET>,
            GetWindowless: GetWindowless::<Identity, Impl, OFFSET>,
            SetFullScreen: SetFullScreen::<Identity, Impl, OFFSET>,
            GetFullScreen: GetFullScreen::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeRealEstateHost_Impl: ::windows_core::BaseImpl {
    fn OnDesiredSizeChange(this: &Self::This, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn OnFullScreenTransition(this: &Self::This, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPNodeRealEstateHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeRealEstateHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDesiredSizeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDesiredSizeChange(this, ::core::mem::transmute_copy(&psize)).into())
        }
        unsafe extern "system" fn OnFullScreenTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeRealEstateHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffullscreen: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFullScreenTransition(this, ::core::mem::transmute_copy(&ffullscreen)).into())
        }
        IWMPNodeRealEstateHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDesiredSizeChange: OnDesiredSizeChange::<Identity, Impl, OFFSET>,
            OnFullScreenTransition: OnFullScreenTransition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPNodeWindowed_Impl: ::windows_core::BaseImpl {
    fn SetOwnerWindow(this: &Self::This, hwnd: isize) -> ::windows_core::Result<()>;
    fn GetOwnerWindow(this: &Self::This, phwnd: *mut isize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPNodeWindowed {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeWindowed {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOwnerWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOwnerWindow(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn GetOwnerWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOwnerWindow(this, ::core::mem::transmute_copy(&phwnd)).into())
        }
        IWMPNodeWindowed_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOwnerWindow: SetOwnerWindow::<Identity, Impl, OFFSET>,
            GetOwnerWindow: GetOwnerWindow::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowedHost_Impl: ::windows_core::BaseImpl {
    fn OnWindowMessageFromRenderer(this: &Self::This, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPNodeWindowedHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeWindowedHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWindowMessageFromRenderer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowedHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowMessageFromRenderer(this, ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into())
        }
        IWMPNodeWindowedHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWindowMessageFromRenderer: OnWindowMessageFromRenderer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowless_Impl: ::windows_core::BaseImpl + IWMPWindowMessageSink_Impl {
    fn OnDraw(this: &Self::This, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPNodeWindowless {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPWindowMessageSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowless_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeWindowless {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: isize, prcdraw: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDraw(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&prcdraw)).into())
        }
        IWMPNodeWindowless_Vtbl { base__: <IWMPWindowMessageSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnDraw: OnDraw::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPNodeWindowlessHost_Impl: ::windows_core::BaseImpl {
    fn InvalidateRect(this: &Self::This, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPNodeWindowlessHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPNodeWindowlessHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InvalidateRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPNodeWindowlessHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateRect(this, ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&ferase)).into())
        }
        IWMPNodeWindowlessHost_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, InvalidateRect: InvalidateRect::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlayer_Impl: ::windows_core::BaseImpl + IWMPCore_Impl {
    fn enabled(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setenabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn fullScreen(this: &Self::This, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetfullScreen(this: &Self::This, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn enableContextMenu(this: &Self::This, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetenableContextMenu(this: &Self::This, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetuiMode(this: &Self::This, bstrmode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn uiMode(this: &Self::This, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlayer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enabled(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setenabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fullScreen(this, ::core::mem::transmute_copy(&pbfullscreen)).into())
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetfullScreen(this, ::core::mem::transmute_copy(&bfullscreen)).into())
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enableContextMenu(this, ::core::mem::transmute_copy(&pbenablecontextmenu)).into())
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetenableContextMenu(this, ::core::mem::transmute_copy(&benablecontextmenu)).into())
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuiMode(this, ::core::mem::transmute(&bstrmode)).into())
        }
        unsafe extern "system" fn uiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::uiMode(this, ::core::mem::transmute_copy(&pbstrmode)).into())
        }
        IWMPPlayer_Vtbl {
            base__: <IWMPCore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlayer2_Impl: ::windows_core::BaseImpl + IWMPCore_Impl {
    fn enabled(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setenabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn fullScreen(this: &Self::This, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetfullScreen(this: &Self::This, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn enableContextMenu(this: &Self::This, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetenableContextMenu(this: &Self::This, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetuiMode(this: &Self::This, bstrmode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn uiMode(this: &Self::This, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn stretchToFit(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetstretchToFit(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn windowlessVideo(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetwindowlessVideo(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlayer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enabled(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setenabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fullScreen(this, ::core::mem::transmute_copy(&pbfullscreen)).into())
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetfullScreen(this, ::core::mem::transmute_copy(&bfullscreen)).into())
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enableContextMenu(this, ::core::mem::transmute_copy(&pbenablecontextmenu)).into())
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetenableContextMenu(this, ::core::mem::transmute_copy(&benablecontextmenu)).into())
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuiMode(this, ::core::mem::transmute(&bstrmode)).into())
        }
        unsafe extern "system" fn uiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::uiMode(this, ::core::mem::transmute_copy(&pbstrmode)).into())
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stretchToFit(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetstretchToFit(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::windowlessVideo(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetwindowlessVideo(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        IWMPPlayer2_Vtbl {
            base__: <IWMPCore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlayer3_Impl: ::windows_core::BaseImpl + IWMPCore2_Impl {
    fn enabled(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setenabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn fullScreen(this: &Self::This, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetfullScreen(this: &Self::This, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn enableContextMenu(this: &Self::This, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetenableContextMenu(this: &Self::This, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetuiMode(this: &Self::This, bstrmode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn uiMode(this: &Self::This, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn stretchToFit(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetstretchToFit(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn windowlessVideo(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetwindowlessVideo(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlayer3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayer3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enabled(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setenabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fullScreen(this, ::core::mem::transmute_copy(&pbfullscreen)).into())
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetfullScreen(this, ::core::mem::transmute_copy(&bfullscreen)).into())
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enableContextMenu(this, ::core::mem::transmute_copy(&pbenablecontextmenu)).into())
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetenableContextMenu(this, ::core::mem::transmute_copy(&benablecontextmenu)).into())
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuiMode(this, ::core::mem::transmute(&bstrmode)).into())
        }
        unsafe extern "system" fn uiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::uiMode(this, ::core::mem::transmute_copy(&pbstrmode)).into())
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stretchToFit(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetstretchToFit(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::windowlessVideo(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetwindowlessVideo(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        IWMPPlayer3_Vtbl {
            base__: <IWMPCore2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlayer4_Impl: ::windows_core::BaseImpl + IWMPCore3_Impl {
    fn enabled(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setenabled(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn fullScreen(this: &Self::This, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetfullScreen(this: &Self::This, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn enableContextMenu(this: &Self::This, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetenableContextMenu(this: &Self::This, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetuiMode(this: &Self::This, bstrmode: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn uiMode(this: &Self::This, pbstrmode: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn stretchToFit(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetstretchToFit(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn windowlessVideo(this: &Self::This, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetwindowlessVideo(this: &Self::This, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn isRemote(this: &Self::This, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn playerApplication(this: &Self::This) -> ::windows_core::Result<IWMPPlayerApplication>;
    fn openPlayer(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlayer4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPCore3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayer4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enabled(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setenabled(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn fullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbfullscreen: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fullScreen(this, ::core::mem::transmute_copy(&pbfullscreen)).into())
        }
        unsafe extern "system" fn SetfullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfullscreen: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetfullScreen(this, ::core::mem::transmute_copy(&bfullscreen)).into())
        }
        unsafe extern "system" fn enableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenablecontextmenu: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enableContextMenu(this, ::core::mem::transmute_copy(&pbenablecontextmenu)).into())
        }
        unsafe extern "system" fn SetenableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benablecontextmenu: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetenableContextMenu(this, ::core::mem::transmute_copy(&benablecontextmenu)).into())
        }
        unsafe extern "system" fn SetuiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuiMode(this, ::core::mem::transmute(&bstrmode)).into())
        }
        unsafe extern "system" fn uiMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::uiMode(this, ::core::mem::transmute_copy(&pbstrmode)).into())
        }
        unsafe extern "system" fn stretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stretchToFit(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetstretchToFit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetstretchToFit(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn windowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::windowlessVideo(this, ::core::mem::transmute_copy(&pbenabled)).into())
        }
        unsafe extern "system" fn SetwindowlessVideo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetwindowlessVideo(this, ::core::mem::transmute_copy(&benabled)).into())
        }
        unsafe extern "system" fn isRemote<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarfisremote: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isRemote(this, ::core::mem::transmute_copy(&pvarfisremote)).into())
        }
        unsafe extern "system" fn playerApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiwmpplayerapplication: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::playerApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpplayerapplication, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn openPlayer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayer4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::openPlayer(this, ::core::mem::transmute(&bstrurl)).into())
        }
        IWMPPlayer4_Vtbl {
            base__: <IWMPCore3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
            fullScreen: fullScreen::<Identity, Impl, OFFSET>,
            SetfullScreen: SetfullScreen::<Identity, Impl, OFFSET>,
            enableContextMenu: enableContextMenu::<Identity, Impl, OFFSET>,
            SetenableContextMenu: SetenableContextMenu::<Identity, Impl, OFFSET>,
            SetuiMode: SetuiMode::<Identity, Impl, OFFSET>,
            uiMode: uiMode::<Identity, Impl, OFFSET>,
            stretchToFit: stretchToFit::<Identity, Impl, OFFSET>,
            SetstretchToFit: SetstretchToFit::<Identity, Impl, OFFSET>,
            windowlessVideo: windowlessVideo::<Identity, Impl, OFFSET>,
            SetwindowlessVideo: SetwindowlessVideo::<Identity, Impl, OFFSET>,
            isRemote: isRemote::<Identity, Impl, OFFSET>,
            playerApplication: playerApplication::<Identity, Impl, OFFSET>,
            openPlayer: openPlayer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlayerApplication_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn switchToPlayerApplication(this: &Self::This) -> ::windows_core::Result<()>;
    fn switchToControl(this: &Self::This) -> ::windows_core::Result<()>;
    fn playerDocked(this: &Self::This, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn hasDisplay(this: &Self::This, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlayerApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayerApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn switchToPlayerApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::switchToPlayerApplication(this).into())
        }
        unsafe extern "system" fn switchToControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::switchToControl(this).into())
        }
        unsafe extern "system" fn playerDocked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbplayerdocked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::playerDocked(this, ::core::mem::transmute_copy(&pbplayerdocked)).into())
        }
        unsafe extern "system" fn hasDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbhasdisplay: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::hasDisplay(this, ::core::mem::transmute_copy(&pbhasdisplay)).into())
        }
        IWMPPlayerApplication_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            switchToPlayerApplication: switchToPlayerApplication::<Identity, Impl, OFFSET>,
            switchToControl: switchToControl::<Identity, Impl, OFFSET>,
            playerDocked: playerDocked::<Identity, Impl, OFFSET>,
            hasDisplay: hasDisplay::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPPlayerServices_Impl: ::windows_core::BaseImpl {
    fn activateUIPlugin(this: &Self::This, bstrplugin: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setTaskPane(this: &Self::This, bstrtaskpane: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setTaskPaneURL(this: &Self::This, bstrtaskpane: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, bstrfriendlyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPPlayerServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayerServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn activateUIPlugin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrplugin: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::activateUIPlugin(this, ::core::mem::transmute(&bstrplugin)).into())
        }
        unsafe extern "system" fn setTaskPane<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setTaskPane(this, ::core::mem::transmute(&bstrtaskpane)).into())
        }
        unsafe extern "system" fn setTaskPaneURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrtaskpane: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrfriendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setTaskPaneURL(this, ::core::mem::transmute(&bstrtaskpane), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrfriendlyname)).into())
        }
        IWMPPlayerServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            activateUIPlugin: activateUIPlugin::<Identity, Impl, OFFSET>,
            setTaskPane: setTaskPane::<Identity, Impl, OFFSET>,
            setTaskPaneURL: setTaskPaneURL::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPPlayerServices2_Impl: ::windows_core::BaseImpl + IWMPPlayerServices_Impl {
    fn setBackgroundProcessingPriority(this: &Self::This, bstrpriority: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPPlayerServices2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPPlayerServices);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlayerServices2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn setBackgroundProcessingPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlayerServices2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpriority: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setBackgroundProcessingPriority(this, ::core::mem::transmute(&bstrpriority)).into())
        }
        IWMPPlayerServices2_Vtbl {
            base__: <IWMPPlayerServices as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            setBackgroundProcessingPriority: setBackgroundProcessingPriority::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlaylist_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn name(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Setname(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn attributeCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn get_attributeName(this: &Self::This, lindex: i32, pbstrattributename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_item(this: &Self::This, lindex: i32) -> ::windows_core::Result<IWMPMedia>;
    fn getItemInfo(this: &Self::This, bstrname: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setItemInfo(this: &Self::This, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_isIdentical(this: &Self::This, piwmpplaylist: ::core::option::Option<&IWMPPlaylist>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn insertItem(this: &Self::This, lindex: i32, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn appendItem(this: &Self::This, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn removeItem(this: &Self::This, piwmpmedia: ::core::option::Option<&IWMPMedia>) -> ::windows_core::Result<()>;
    fn moveItem(this: &Self::This, lindexold: i32, lindexnew: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlaylist {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlaylist {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::name(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn Setname<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setname(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn attributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::attributeCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn get_attributeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrattributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_attributeName(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrattributename)).into())
        }
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppiwmpmedia: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiwmpmedia, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        unsafe extern "system" fn setItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setItemInfo(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn get_isIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpplaylist: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_isIdentical(this, ::windows_core::from_raw_borrowed(&piwmpplaylist), ::core::mem::transmute_copy(&pvbool)).into())
        }
        unsafe extern "system" fn clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::clear(this).into())
        }
        unsafe extern "system" fn insertItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::insertItem(this, ::core::mem::transmute_copy(&lindex), ::windows_core::from_raw_borrowed(&piwmpmedia)).into())
        }
        unsafe extern "system" fn appendItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::appendItem(this, ::windows_core::from_raw_borrowed(&piwmpmedia)).into())
        }
        unsafe extern "system" fn removeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpmedia: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeItem(this, ::windows_core::from_raw_borrowed(&piwmpmedia)).into())
        }
        unsafe extern "system" fn moveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindexold: i32, lindexnew: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::moveItem(this, ::core::mem::transmute_copy(&lindexold), ::core::mem::transmute_copy(&lindexnew)).into())
        }
        IWMPPlaylist_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            count: count::<Identity, Impl, OFFSET>,
            name: name::<Identity, Impl, OFFSET>,
            Setname: Setname::<Identity, Impl, OFFSET>,
            attributeCount: attributeCount::<Identity, Impl, OFFSET>,
            get_attributeName: get_attributeName::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            setItemInfo: setItemInfo::<Identity, Impl, OFFSET>,
            get_isIdentical: get_isIdentical::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            insertItem: insertItem::<Identity, Impl, OFFSET>,
            appendItem: appendItem::<Identity, Impl, OFFSET>,
            removeItem: removeItem::<Identity, Impl, OFFSET>,
            moveItem: moveItem::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlaylistArray_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn item(this: &Self::This, lindex: i32) -> ::windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlaylistArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlaylistArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::item(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPPlaylistArray_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPPlaylistCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn newPlaylist(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylist>;
    fn getAll(this: &Self::This) -> ::windows_core::Result<IWMPPlaylistArray>;
    fn getByName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IWMPPlaylistArray>;
    fn remove(this: &Self::This, pitem: ::core::option::Option<&IWMPPlaylist>) -> ::windows_core::Result<()>;
    fn setDeleted(this: &Self::This, pitem: ::core::option::Option<&IWMPPlaylist>, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn isDeleted(this: &Self::This, pitem: ::core::option::Option<&IWMPPlaylist>, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn importPlaylist(this: &Self::This, pitem: ::core::option::Option<&IWMPPlaylist>) -> ::windows_core::Result<IWMPPlaylist>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPPlaylistCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlaylistCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn newPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::newPlaylist(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAll(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppplaylistarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getByName(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppplaylistarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remove(this, ::windows_core::from_raw_borrowed(&pitem)).into())
        }
        unsafe extern "system" fn setDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, varfisdeleted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setDeleted(this, ::windows_core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&varfisdeleted)).into())
        }
        unsafe extern "system" fn isDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, pvarfisdeleted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isDeleted(this, ::windows_core::from_raw_borrowed(&pitem), ::core::mem::transmute_copy(&pvarfisdeleted)).into())
        }
        unsafe extern "system" fn importPlaylist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlaylistCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void, ppimporteditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::importPlaylist(this, ::windows_core::from_raw_borrowed(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimporteditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPPlaylistCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            newPlaylist: newPlaylist::<Identity, Impl, OFFSET>,
            getAll: getAll::<Identity, Impl, OFFSET>,
            getByName: getByName::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            setDeleted: setDeleted::<Identity, Impl, OFFSET>,
            isDeleted: isDeleted::<Identity, Impl, OFFSET>,
            importPlaylist: importPlaylist::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPPlugin_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, dwplaybackcontext: usize) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetID(this: &Self::This, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, pdwflags: *mut u32) -> ::windows_core::Result<()>;
    fn AdviseWMPServices(this: &Self::This, pwmpservices: ::core::option::Option<&IWMPServices>) -> ::windows_core::Result<()>;
    fn UnAdviseWMPServices(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPPlugin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPlugin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwplaybackcontext: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&dwplaybackcontext)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetID(this, ::core::mem::transmute_copy(&pguid)).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&pdwflags)).into())
        }
        unsafe extern "system" fn AdviseWMPServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpservices: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseWMPServices(this, ::windows_core::from_raw_borrowed(&pwmpservices)).into())
        }
        unsafe extern "system" fn UnAdviseWMPServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPlugin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnAdviseWMPServices(this).into())
        }
        IWMPPlugin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            AdviseWMPServices: AdviseWMPServices::<Identity, Impl, OFFSET>,
            UnAdviseWMPServices: UnAdviseWMPServices::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPPluginEnable_Impl: ::windows_core::BaseImpl {
    fn SetEnable(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetEnable(this: &Self::This, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPPluginEnable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPluginEnable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEnable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnable(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn GetEnable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginEnable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEnable(this, ::core::mem::transmute_copy(&pfenable)).into())
        }
        IWMPPluginEnable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetEnable: SetEnable::<Identity, Impl, OFFSET>,
            GetEnable: GetEnable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IWMPPluginUI_Impl: ::windows_core::BaseImpl {
    fn SetCore(this: &Self::This, pcore: ::core::option::Option<&IWMPCore>) -> ::windows_core::Result<()>;
    fn Create(this: &Self::This, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn DisplayPropertyPage(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, pwszname: &::windows_core::PCWSTR, pvarproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, pwszname: &::windows_core::PCWSTR, pvarproperty: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(this: &Self::This, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IWMPPluginUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPPluginUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCore(this, ::windows_core::from_raw_borrowed(&pcore)).into())
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, phwndwindow: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&phwndwindow)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn DisplayPropertyPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayPropertyPage(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pvarproperty: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pvarproperty: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pvarproperty)).into())
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPPluginUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsg: *mut super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&lpmsg)).into())
        }
        IWMPPluginUI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCore: SetCore::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            DisplayPropertyPage: DisplayPropertyPage::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPQuery_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn addCondition(this: &Self::This, bstrattribute: &::windows_core::BSTR, bstroperator: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn beginNextGroup(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn addCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrattribute: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstroperator: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addCondition(this, ::core::mem::transmute(&bstrattribute), ::core::mem::transmute(&bstroperator), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn beginNextGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::beginNextGroup(this).into())
        }
        IWMPQuery_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            addCondition: addCondition::<Identity, Impl, OFFSET>,
            beginNextGroup: beginNextGroup::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IWMPRemoteMediaServices_Impl: ::windows_core::BaseImpl {
    fn GetServiceType(this: &Self::This, pbstrtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetApplicationName(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetScriptableObject(this: &Self::This, pbstrname: *mut ::windows_core::BSTR, ppdispatch: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn GetCustomUIMode(this: &Self::This, pbstrfile: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IWMPRemoteMediaServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPRemoteMediaServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetServiceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetServiceType(this, ::core::mem::transmute_copy(&pbstrtype)).into())
        }
        unsafe extern "system" fn GetApplicationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApplicationName(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn GetScriptableObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptableObject(this, ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&ppdispatch)).into())
        }
        unsafe extern "system" fn GetCustomUIMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRemoteMediaServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCustomUIMode(this, ::core::mem::transmute_copy(&pbstrfile)).into())
        }
        IWMPRemoteMediaServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetServiceType: GetServiceType::<Identity, Impl, OFFSET>,
            GetApplicationName: GetApplicationName::<Identity, Impl, OFFSET>,
            GetScriptableObject: GetScriptableObject::<Identity, Impl, OFFSET>,
            GetCustomUIMode: GetCustomUIMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPRenderConfig_Impl: ::windows_core::BaseImpl {
    fn SetinProcOnly(this: &Self::This, finproc: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn inProcOnly(this: &Self::This, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPRenderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPRenderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetinProcOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetinProcOnly(this, ::core::mem::transmute_copy(&finproc)).into())
        }
        unsafe extern "system" fn inProcOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPRenderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfinproc: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::inProcOnly(this, ::core::mem::transmute_copy(&pfinproc)).into())
        }
        IWMPRenderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetinProcOnly: SetinProcOnly::<Identity, Impl, OFFSET>,
            inProcOnly: inProcOnly::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPServices_Impl: ::windows_core::BaseImpl {
    fn GetStreamTime(this: &Self::This, prt: *mut i64) -> ::windows_core::Result<()>;
    fn GetStreamState(this: &Self::This, pstate: *mut WMPServices_StreamState) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStreamTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamTime(this, ::core::mem::transmute_copy(&prt)).into())
        }
        unsafe extern "system" fn GetStreamState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut WMPServices_StreamState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamState(this, ::core::mem::transmute_copy(&pstate)).into())
        }
        IWMPServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStreamTime: GetStreamTime::<Identity, Impl, OFFSET>,
            GetStreamState: GetStreamState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPSettings_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_isAvailable(this: &Self::This, bstritem: &::windows_core::BSTR, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn autoStart(this: &Self::This, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetautoStart(this: &Self::This, fautostart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn baseURL(this: &Self::This, pbstrbaseurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetbaseURL(this: &Self::This, bstrbaseurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn defaultFrame(this: &Self::This, pbstrdefaultframe: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetdefaultFrame(this: &Self::This, bstrdefaultframe: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn invokeURLs(this: &Self::This, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetinvokeURLs(this: &Self::This, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn mute(this: &Self::This, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setmute(this: &Self::This, fmute: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn playCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn SetplayCount(this: &Self::This, lcount: i32) -> ::windows_core::Result<()>;
    fn rate(this: &Self::This, pdrate: *mut f64) -> ::windows_core::Result<()>;
    fn Setrate(this: &Self::This, drate: f64) -> ::windows_core::Result<()>;
    fn balance(this: &Self::This, plbalance: *mut i32) -> ::windows_core::Result<()>;
    fn Setbalance(this: &Self::This, lbalance: i32) -> ::windows_core::Result<()>;
    fn volume(this: &Self::This, plvolume: *mut i32) -> ::windows_core::Result<()>;
    fn Setvolume(this: &Self::This, lvolume: i32) -> ::windows_core::Result<()>;
    fn getMode(this: &Self::This, bstrmode: &::windows_core::BSTR, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setMode(this: &Self::This, bstrmode: &::windows_core::BSTR, varfmode: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn enableErrorDialogs(this: &Self::This, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetenableErrorDialogs(this: &Self::This, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_isAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritem: ::std::mem::MaybeUninit<::windows_core::BSTR>, pisavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_isAvailable(this, ::core::mem::transmute(&bstritem), ::core::mem::transmute_copy(&pisavailable)).into())
        }
        unsafe extern "system" fn autoStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfautostart: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::autoStart(this, ::core::mem::transmute_copy(&pfautostart)).into())
        }
        unsafe extern "system" fn SetautoStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fautostart: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetautoStart(this, ::core::mem::transmute_copy(&fautostart)).into())
        }
        unsafe extern "system" fn baseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::baseURL(this, ::core::mem::transmute_copy(&pbstrbaseurl)).into())
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrbaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetbaseURL(this, ::core::mem::transmute(&bstrbaseurl)).into())
        }
        unsafe extern "system" fn defaultFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdefaultframe: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::defaultFrame(this, ::core::mem::transmute_copy(&pbstrdefaultframe)).into())
        }
        unsafe extern "system" fn SetdefaultFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdefaultframe: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetdefaultFrame(this, ::core::mem::transmute(&bstrdefaultframe)).into())
        }
        unsafe extern "system" fn invokeURLs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfinvokeurls: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::invokeURLs(this, ::core::mem::transmute_copy(&pfinvokeurls)).into())
        }
        unsafe extern "system" fn SetinvokeURLs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finvokeurls: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetinvokeURLs(this, ::core::mem::transmute_copy(&finvokeurls)).into())
        }
        unsafe extern "system" fn mute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfmute: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::mute(this, ::core::mem::transmute_copy(&pfmute)).into())
        }
        unsafe extern "system" fn Setmute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmute: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setmute(this, ::core::mem::transmute_copy(&fmute)).into())
        }
        unsafe extern "system" fn playCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::playCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn SetplayCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetplayCount(this, ::core::mem::transmute_copy(&lcount)).into())
        }
        unsafe extern "system" fn rate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdrate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::rate(this, ::core::mem::transmute_copy(&pdrate)).into())
        }
        unsafe extern "system" fn Setrate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drate: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setrate(this, ::core::mem::transmute_copy(&drate)).into())
        }
        unsafe extern "system" fn balance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::balance(this, ::core::mem::transmute_copy(&plbalance)).into())
        }
        unsafe extern "system" fn Setbalance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setbalance(this, ::core::mem::transmute_copy(&lbalance)).into())
        }
        unsafe extern "system" fn volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::volume(this, ::core::mem::transmute_copy(&plvolume)).into())
        }
        unsafe extern "system" fn Setvolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setvolume(this, ::core::mem::transmute_copy(&lvolume)).into())
        }
        unsafe extern "system" fn getMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarfmode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getMode(this, ::core::mem::transmute(&bstrmode), ::core::mem::transmute_copy(&pvarfmode)).into())
        }
        unsafe extern "system" fn setMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmode: ::std::mem::MaybeUninit<::windows_core::BSTR>, varfmode: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setMode(this, ::core::mem::transmute(&bstrmode), ::core::mem::transmute_copy(&varfmode)).into())
        }
        unsafe extern "system" fn enableErrorDialogs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenableerrordialogs: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::enableErrorDialogs(this, ::core::mem::transmute_copy(&pfenableerrordialogs)).into())
        }
        unsafe extern "system" fn SetenableErrorDialogs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenableerrordialogs: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetenableErrorDialogs(this, ::core::mem::transmute_copy(&fenableerrordialogs)).into())
        }
        IWMPSettings_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_isAvailable: get_isAvailable::<Identity, Impl, OFFSET>,
            autoStart: autoStart::<Identity, Impl, OFFSET>,
            SetautoStart: SetautoStart::<Identity, Impl, OFFSET>,
            baseURL: baseURL::<Identity, Impl, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, Impl, OFFSET>,
            defaultFrame: defaultFrame::<Identity, Impl, OFFSET>,
            SetdefaultFrame: SetdefaultFrame::<Identity, Impl, OFFSET>,
            invokeURLs: invokeURLs::<Identity, Impl, OFFSET>,
            SetinvokeURLs: SetinvokeURLs::<Identity, Impl, OFFSET>,
            mute: mute::<Identity, Impl, OFFSET>,
            Setmute: Setmute::<Identity, Impl, OFFSET>,
            playCount: playCount::<Identity, Impl, OFFSET>,
            SetplayCount: SetplayCount::<Identity, Impl, OFFSET>,
            rate: rate::<Identity, Impl, OFFSET>,
            Setrate: Setrate::<Identity, Impl, OFFSET>,
            balance: balance::<Identity, Impl, OFFSET>,
            Setbalance: Setbalance::<Identity, Impl, OFFSET>,
            volume: volume::<Identity, Impl, OFFSET>,
            Setvolume: Setvolume::<Identity, Impl, OFFSET>,
            getMode: getMode::<Identity, Impl, OFFSET>,
            setMode: setMode::<Identity, Impl, OFFSET>,
            enableErrorDialogs: enableErrorDialogs::<Identity, Impl, OFFSET>,
            SetenableErrorDialogs: SetenableErrorDialogs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPSettings2_Impl: ::windows_core::BaseImpl + IWMPSettings_Impl {
    fn defaultAudioLanguage(this: &Self::This, pllangid: *mut i32) -> ::windows_core::Result<()>;
    fn mediaAccessRights(this: &Self::This, pbstrrights: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn requestMediaAccessRights(this: &Self::This, bstrdesiredaccess: &::windows_core::BSTR, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPSettings2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPSettings);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSettings2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn defaultAudioLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllangid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::defaultAudioLanguage(this, ::core::mem::transmute_copy(&pllangid)).into())
        }
        unsafe extern "system" fn mediaAccessRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrights: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::mediaAccessRights(this, ::core::mem::transmute_copy(&pbstrrights)).into())
        }
        unsafe extern "system" fn requestMediaAccessRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSettings2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdesiredaccess: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvbaccepted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::requestMediaAccessRights(this, ::core::mem::transmute(&bstrdesiredaccess), ::core::mem::transmute_copy(&pvbaccepted)).into())
        }
        IWMPSettings2_Vtbl {
            base__: <IWMPSettings as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            defaultAudioLanguage: defaultAudioLanguage::<Identity, Impl, OFFSET>,
            mediaAccessRights: mediaAccessRights::<Identity, Impl, OFFSET>,
            requestMediaAccessRights: requestMediaAccessRights::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPSkinManager_Impl: ::windows_core::BaseImpl {
    fn SetVisualStyle(this: &Self::This, bstrpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPSkinManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSkinManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSkinManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVisualStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSkinManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVisualStyle(this, ::core::mem::transmute(&bstrpath)).into())
        }
        IWMPSkinManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetVisualStyle: SetVisualStyle::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPStringCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn count(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn item(this: &Self::This, lindex: i32, pbstrstring: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPStringCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPStringCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::count(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::item(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pbstrstring)).into())
        }
        IWMPStringCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            count: count::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWMPStringCollection2_Impl: ::windows_core::BaseImpl + IWMPStringCollection_Impl {
    fn isIdentical(this: &Self::This, piwmpstringcollection2: ::core::option::Option<&IWMPStringCollection2>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getItemInfo(this: &Self::This, lcollectionindex: i32, bstritemname: &::windows_core::BSTR, pbstrvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getAttributeCountByType(this: &Self::This, lcollectionindex: i32, bstrtype: &::windows_core::BSTR, bstrlanguage: &::windows_core::BSTR, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getItemInfoByType(this: &Self::This, lcollectionindex: i32, bstrtype: &::windows_core::BSTR, bstrlanguage: &::windows_core::BSTR, lattributeindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWMPStringCollection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPStringCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPStringCollection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn isIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piwmpstringcollection2: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isIdentical(this, ::windows_core::from_raw_borrowed(&piwmpstringcollection2), ::core::mem::transmute_copy(&pvbool)).into())
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrvalue)).into())
        }
        unsafe extern "system" fn getAttributeCountByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getAttributeCountByType(this, ::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getItemInfoByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPStringCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcollectionindex: i32, bstrtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrlanguage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lattributeindex: i32, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfoByType(this, ::core::mem::transmute_copy(&lcollectionindex), ::core::mem::transmute(&bstrtype), ::core::mem::transmute(&bstrlanguage), ::core::mem::transmute_copy(&lattributeindex), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        IWMPStringCollection2_Vtbl {
            base__: <IWMPStringCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            getAttributeCountByType: getAttributeCountByType::<Identity, Impl, OFFSET>,
            getItemInfoByType: getItemInfoByType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService_Impl: ::windows_core::BaseImpl {
    fn allowPlay(this: &Self::This, hwnd: super::super::Foundation::HWND, pmedia: ::core::option::Option<&IWMPMedia>, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn allowCDBurn(this: &Self::This, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<&IWMPPlaylist>, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn allowPDATransfer(this: &Self::This, hwnd: super::super::Foundation::HWND, pplaylist: ::core::option::Option<&IWMPPlaylist>, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn startBackgroundProcessing(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPSubscriptionService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSubscriptionService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn allowPlay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pmedia: *mut ::core::ffi::c_void, pfallowplay: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::allowPlay(this, ::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&pmedia), ::core::mem::transmute_copy(&pfallowplay)).into())
        }
        unsafe extern "system" fn allowCDBurn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowburn: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::allowCDBurn(this, ::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pfallowburn)).into())
        }
        unsafe extern "system" fn allowPDATransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pplaylist: *mut ::core::ffi::c_void, pfallowtransfer: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::allowPDATransfer(this, ::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&pplaylist), ::core::mem::transmute_copy(&pfallowtransfer)).into())
        }
        unsafe extern "system" fn startBackgroundProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startBackgroundProcessing(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        IWMPSubscriptionService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            allowPlay: allowPlay::<Identity, Impl, OFFSET>,
            allowCDBurn: allowCDBurn::<Identity, Impl, OFFSET>,
            allowPDATransfer: allowPDATransfer::<Identity, Impl, OFFSET>,
            startBackgroundProcessing: startBackgroundProcessing::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSubscriptionService2_Impl: ::windows_core::BaseImpl + IWMPSubscriptionService_Impl {
    fn stopBackgroundProcessing(this: &Self::This) -> ::windows_core::Result<()>;
    fn serviceEvent(this: &Self::This, event: WMPSubscriptionServiceEvent) -> ::windows_core::Result<()>;
    fn deviceAvailable(this: &Self::This, bstrdevicename: &::windows_core::BSTR, pcb: ::core::option::Option<&IWMPSubscriptionServiceCallback>) -> ::windows_core::Result<()>;
    fn prepareForSync(this: &Self::This, bstrfilename: &::windows_core::BSTR, bstrdevicename: &::windows_core::BSTR, pcb: ::core::option::Option<&IWMPSubscriptionServiceCallback>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPSubscriptionService2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPSubscriptionService);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSubscriptionService2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn stopBackgroundProcessing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stopBackgroundProcessing(this).into())
        }
        unsafe extern "system" fn serviceEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: WMPSubscriptionServiceEvent) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::serviceEvent(this, ::core::mem::transmute_copy(&event)).into())
        }
        unsafe extern "system" fn deviceAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deviceAvailable(this, ::core::mem::transmute(&bstrdevicename), ::windows_core::from_raw_borrowed(&pcb)).into())
        }
        unsafe extern "system" fn prepareForSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionService2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::prepareForSync(this, ::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrdevicename), ::windows_core::from_raw_borrowed(&pcb)).into())
        }
        IWMPSubscriptionService2_Vtbl {
            base__: <IWMPSubscriptionService as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            stopBackgroundProcessing: stopBackgroundProcessing::<Identity, Impl, OFFSET>,
            serviceEvent: serviceEvent::<Identity, Impl, OFFSET>,
            deviceAvailable: deviceAvailable::<Identity, Impl, OFFSET>,
            prepareForSync: prepareForSync::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPSubscriptionServiceCallback_Impl: ::windows_core::BaseImpl {
    fn onComplete(this: &Self::This, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPSubscriptionServiceCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSubscriptionServiceCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSubscriptionServiceCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onComplete(this, ::core::mem::transmute_copy(&hrresult)).into())
        }
        IWMPSubscriptionServiceCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, onComplete: onComplete::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice_Impl: ::windows_core::BaseImpl {
    fn friendlyName(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetfriendlyName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn deviceName(this: &Self::This, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn deviceId(this: &Self::This, pbstrdeviceid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn partnershipIndex(this: &Self::This, plindex: *mut i32) -> ::windows_core::Result<()>;
    fn connected(this: &Self::This, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn status(this: &Self::This, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::Result<()>;
    fn syncState(this: &Self::This, pwmpss: *mut WMPSyncState) -> ::windows_core::Result<()>;
    fn progress(this: &Self::This, plprogress: *mut i32) -> ::windows_core::Result<()>;
    fn getItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, pbstrval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn createPartnership(this: &Self::This, vbshowui: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn deletePartnership(this: &Self::This) -> ::windows_core::Result<()>;
    fn start(this: &Self::This) -> ::windows_core::Result<()>;
    fn stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn showSettings(this: &Self::This) -> ::windows_core::Result<()>;
    fn isIdentical(this: &Self::This, pdevice: ::core::option::Option<&IWMPSyncDevice>, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPSyncDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSyncDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn friendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::friendlyName(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn SetfriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetfriendlyName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn deviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deviceName(this, ::core::mem::transmute_copy(&pbstrname)).into())
        }
        unsafe extern "system" fn deviceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdeviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deviceId(this, ::core::mem::transmute_copy(&pbstrdeviceid)).into())
        }
        unsafe extern "system" fn partnershipIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::partnershipIndex(this, ::core::mem::transmute_copy(&plindex)).into())
        }
        unsafe extern "system" fn connected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvbconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::connected(this, ::core::mem::transmute_copy(&pvbconnected)).into())
        }
        unsafe extern "system" fn status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpds: *mut WMPDeviceStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::status(this, ::core::mem::transmute_copy(&pwmpds)).into())
        }
        unsafe extern "system" fn syncState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwmpss: *mut WMPSyncState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::syncState(this, ::core::mem::transmute_copy(&pwmpss)).into())
        }
        unsafe extern "system" fn progress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plprogress: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::progress(this, ::core::mem::transmute_copy(&plprogress)).into())
        }
        unsafe extern "system" fn getItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute_copy(&pbstrval)).into())
        }
        unsafe extern "system" fn createPartnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vbshowui: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::createPartnership(this, ::core::mem::transmute_copy(&vbshowui)).into())
        }
        unsafe extern "system" fn deletePartnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deletePartnership(this).into())
        }
        unsafe extern "system" fn start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::start(this).into())
        }
        unsafe extern "system" fn stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stop(this).into())
        }
        unsafe extern "system" fn showSettings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::showSettings(this).into())
        }
        unsafe extern "system" fn isIdentical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pvbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::isIdentical(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pvbool)).into())
        }
        IWMPSyncDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            friendlyName: friendlyName::<Identity, Impl, OFFSET>,
            SetfriendlyName: SetfriendlyName::<Identity, Impl, OFFSET>,
            deviceName: deviceName::<Identity, Impl, OFFSET>,
            deviceId: deviceId::<Identity, Impl, OFFSET>,
            partnershipIndex: partnershipIndex::<Identity, Impl, OFFSET>,
            connected: connected::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            syncState: syncState::<Identity, Impl, OFFSET>,
            progress: progress::<Identity, Impl, OFFSET>,
            getItemInfo: getItemInfo::<Identity, Impl, OFFSET>,
            createPartnership: createPartnership::<Identity, Impl, OFFSET>,
            deletePartnership: deletePartnership::<Identity, Impl, OFFSET>,
            start: start::<Identity, Impl, OFFSET>,
            stop: stop::<Identity, Impl, OFFSET>,
            showSettings: showSettings::<Identity, Impl, OFFSET>,
            isIdentical: isIdentical::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPSyncDevice2_Impl: ::windows_core::BaseImpl + IWMPSyncDevice_Impl {
    fn setItemInfo(this: &Self::This, bstritemname: &::windows_core::BSTR, bstrval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPSyncDevice2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPSyncDevice);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSyncDevice2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn setItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstritemname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setItemInfo(this, ::core::mem::transmute(&bstritemname), ::core::mem::transmute(&bstrval)).into())
        }
        IWMPSyncDevice2_Vtbl { base__: <IWMPSyncDevice as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, setItemInfo: setItemInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMPSyncDevice3_Impl: ::windows_core::BaseImpl + IWMPSyncDevice2_Impl {
    fn estimateSyncSize(this: &Self::This, pnonruleplaylist: ::core::option::Option<&IWMPPlaylist>, prulesplaylist: ::core::option::Option<&IWMPPlaylist>) -> ::windows_core::Result<()>;
    fn cancelEstimation(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IWMPSyncDevice3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWMPSyncDevice2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSyncDevice3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn estimateSyncSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnonruleplaylist: *mut ::core::ffi::c_void, prulesplaylist: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::estimateSyncSize(this, ::windows_core::from_raw_borrowed(&pnonruleplaylist), ::windows_core::from_raw_borrowed(&prulesplaylist)).into())
        }
        unsafe extern "system" fn cancelEstimation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncDevice3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::cancelEstimation(this).into())
        }
        IWMPSyncDevice3_Vtbl {
            base__: <IWMPSyncDevice2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            estimateSyncSize: estimateSyncSize::<Identity, Impl, OFFSET>,
            cancelEstimation: cancelEstimation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWMPSyncServices_Impl: ::windows_core::BaseImpl {
    fn deviceCount(this: &Self::This, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn getDevice(this: &Self::This, lindex: i32) -> ::windows_core::Result<IWMPSyncDevice>;
}
impl ::windows_core::Iids for IWMPSyncServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPSyncServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn deviceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deviceCount(this, ::core::mem::transmute_copy(&plcount)).into())
        }
        unsafe extern "system" fn getDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPSyncServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDevice(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWMPSyncServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            deviceCount: deviceCount::<Identity, Impl, OFFSET>,
            getDevice: getDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPTranscodePolicy_Impl: ::windows_core::BaseImpl {
    fn allowTranscode(this: &Self::This, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPTranscodePolicy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPTranscodePolicy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPTranscodePolicy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn allowTranscode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPTranscodePolicy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvballow: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::allowTranscode(this, ::core::mem::transmute_copy(&pvballow)).into())
        }
        IWMPTranscodePolicy_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, allowTranscode: allowTranscode::<Identity, Impl, OFFSET> }
    };
}
pub trait IWMPUserEventSink_Impl: ::windows_core::BaseImpl {
    fn NotifyUserEvent(this: &Self::This, eventcode: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWMPUserEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPUserEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPUserEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyUserEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPUserEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyUserEvent(this, ::core::mem::transmute_copy(&eventcode)).into())
        }
        IWMPUserEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifyUserEvent: NotifyUserEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Media_MediaFoundation\"`"]
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub trait IWMPVideoRenderConfig_Impl: ::windows_core::BaseImpl {
    fn SetpresenterActivate(this: &Self::This, pactivate: ::core::option::Option<&super::MediaFoundation::IMFActivate>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl ::windows_core::Iids for IWMPVideoRenderConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Media_MediaFoundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPVideoRenderConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetpresenterActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPVideoRenderConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetpresenterActivate(this, ::windows_core::from_raw_borrowed(&pactivate)).into())
        }
        IWMPVideoRenderConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetpresenterActivate: SetpresenterActivate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPWindowMessageSink_Impl: ::windows_core::BaseImpl {
    fn OnWindowMessage(this: &Self::This, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWMPWindowMessageSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPWindowMessageSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWMPWindowMessageSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWMPWindowMessageSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plret: *mut super::super::Foundation::LRESULT, pfhandled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowMessage(this, ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&plret), ::core::mem::transmute_copy(&pfhandled)).into())
        }
        IWMPWindowMessageSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed_Impl: ::windows_core::BaseImpl {
    fn Xml(this: &Self::This, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Rename(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Url(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetUrl(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LocalId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Move(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Parent(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn LastWriteTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Download(this: &Self::This) -> ::windows_core::Result<()>;
    fn AsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelAsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn SyncSetting(this: &Self::This) -> ::windows_core::Result<FEEDS_SYNC_SETTING>;
    fn SetSyncSetting(this: &Self::This, fss: FEEDS_SYNC_SETTING) -> ::windows_core::Result<()>;
    fn Interval(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetInterval(this: &Self::This, uiinterval: u32) -> ::windows_core::Result<()>;
    fn LastDownloadTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn LocalEnclosurePath(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Items(this: &Self::This) -> ::windows_core::Result<IXFeedsEnum>;
    fn GetItem(this: &Self::This, uiid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn MarkAllItemsRead(this: &Self::This) -> ::windows_core::Result<()>;
    fn MaxItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxItemCount(this: &Self::This, uimaxitemcount: u32) -> ::windows_core::Result<()>;
    fn DownloadEnclosuresAutomatically(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDownloadEnclosuresAutomatically(this: &Self::This, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DownloadStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn Merge(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Link(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Image(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn LastBuildDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn PubDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Ttl(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Copyright(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsList(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(this: &Self::This, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnreadItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXFeed {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeed {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiitemcount: u32, sortproperty: FEEDS_XML_SORT_PROPERTY, sortorder: FEEDS_XML_SORT_ORDER, filterflags: FEEDS_XML_FILTER_FLAGS, includeflags: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this, ::core::mem::transmute_copy(&uiitemcount), ::core::mem::transmute_copy(&sortproperty), ::core::mem::transmute_copy(&sortorder), ::core::mem::transmute_copy(&filterflags), ::core::mem::transmute_copy(&includeflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn Url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUrl(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn LocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Parent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn LastWriteTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastwritetime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastWriteTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastwritetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this).into())
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncDownload(this).into())
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncDownload(this).into())
        }
        unsafe extern "system" fn SyncSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfss: *mut FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncSetting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfss, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyncSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fss: FEEDS_SYNC_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncSetting(this, ::core::mem::transmute_copy(&fss)).into())
        }
        unsafe extern "system" fn Interval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Interval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiinterval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInterval(this, ::core::mem::transmute_copy(&uiinterval)).into())
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastdownloadtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalEnclosurePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalEnclosurePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Items<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Items(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItem(this, ::core::mem::transmute_copy(&uiid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn MarkAllItemsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MarkAllItemsRead(this).into())
        }
        unsafe extern "system" fn MaxItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puimaxitemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puimaxitemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uimaxitemcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxItemCount(this, ::core::mem::transmute_copy(&uimaxitemcount)).into())
        }
        unsafe extern "system" fn DownloadEnclosuresAutomatically<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbdownloadenclosuresautomatically: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadEnclosuresAutomatically(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbdownloadenclosuresautomatically, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDownloadEnclosuresAutomatically<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bdownloadenclosuresautomatically: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDownloadEnclosuresAutomatically(this, ::core::mem::transmute_copy(&bdownloadenclosuresautomatically)).into())
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Merge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Merge(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszhomepage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszhomepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszimageurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Image(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastBuildDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastbuilddate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastBuildDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastbuilddate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PubDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PubDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstpubdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Ttl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puittl: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ttl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puittl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszlanguage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlanguage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Copyright<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcopyright: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Copyright(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcopyright, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbislist: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbislist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWatcher(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn UnreadItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiunreaditemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnreadItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiunreaditemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiitemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiitemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXFeed_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Xml: Xml::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            LastWriteTime: LastWriteTime::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            SyncSetting: SyncSetting::<Identity, Impl, OFFSET>,
            SetSyncSetting: SetSyncSetting::<Identity, Impl, OFFSET>,
            Interval: Interval::<Identity, Impl, OFFSET>,
            SetInterval: SetInterval::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            LocalEnclosurePath: LocalEnclosurePath::<Identity, Impl, OFFSET>,
            Items: Items::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            MarkAllItemsRead: MarkAllItemsRead::<Identity, Impl, OFFSET>,
            MaxItemCount: MaxItemCount::<Identity, Impl, OFFSET>,
            SetMaxItemCount: SetMaxItemCount::<Identity, Impl, OFFSET>,
            DownloadEnclosuresAutomatically: DownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            SetDownloadEnclosuresAutomatically: SetDownloadEnclosuresAutomatically::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            LastBuildDate: LastBuildDate::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Ttl: Ttl::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Copyright: Copyright::<Identity, Impl, OFFSET>,
            IsList: IsList::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            UnreadItemCount: UnreadItemCount::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeed2_Impl: ::windows_core::BaseImpl + IXFeed_Impl {
    fn GetItemByEffectiveId(this: &Self::This, uieffectiveid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn LastItemDownloadTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Username(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Password(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCredentials(this: &Self::This, pszusername: &::windows_core::PCWSTR, pszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ClearCredentials(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXFeed2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXFeed);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeed2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemByEffectiveId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uieffectiveid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemByEffectiveId(this, ::core::mem::transmute_copy(&uieffectiveid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn LastItemDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastitemdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastItemDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastitemdownloadtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Username<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszusername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Username(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Password<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Password(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpassword, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszusername: ::windows_core::PCWSTR, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&pszusername), ::core::mem::transmute(&pszpassword)).into())
        }
        unsafe extern "system" fn ClearCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeed2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearCredentials(this).into())
        }
        IXFeed2_Vtbl {
            base__: <IXFeed as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemByEffectiveId: GetItemByEffectiveId::<Identity, Impl, OFFSET>,
            LastItemDownloadTime: LastItemDownloadTime::<Identity, Impl, OFFSET>,
            Username: Username::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            ClearCredentials: ClearCredentials::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXFeedEnclosure_Impl: ::windows_core::BaseImpl {
    fn Url(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Length(this: &Self::This) -> ::windows_core::Result<u32>;
    fn AsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn CancelAsyncDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn DownloadStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_STATUS>;
    fn LastDownloadError(this: &Self::This) -> ::windows_core::Result<FEEDS_DOWNLOAD_ERROR>;
    fn LocalPath(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Parent(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn DownloadMimeType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn RemoveFile(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFile(this: &Self::This, pszdownloadurl: &::windows_core::PCWSTR, pszdownloadfilepath: &::windows_core::PCWSTR, pszdownloadmimetype: &::windows_core::PCWSTR, pszenclosurefilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXFeedEnclosure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedEnclosure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszmimetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puilength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puilength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncDownload(this).into())
        }
        unsafe extern "system" fn CancelAsyncDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelAsyncDownload(this).into())
        }
        unsafe extern "system" fn DownloadStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfds: *mut FEEDS_DOWNLOAD_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfde: *mut FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Parent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DownloadMimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszmimetype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadMimeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszmimetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFile(this).into())
        }
        unsafe extern "system" fn SetFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEnclosure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdownloadurl: ::windows_core::PCWSTR, pszdownloadfilepath: ::windows_core::PCWSTR, pszdownloadmimetype: ::windows_core::PCWSTR, pszenclosurefilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFile(this, ::core::mem::transmute(&pszdownloadurl), ::core::mem::transmute(&pszdownloadfilepath), ::core::mem::transmute(&pszdownloadmimetype), ::core::mem::transmute(&pszenclosurefilename)).into())
        }
        IXFeedEnclosure_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Url: Url::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            AsyncDownload: AsyncDownload::<Identity, Impl, OFFSET>,
            CancelAsyncDownload: CancelAsyncDownload::<Identity, Impl, OFFSET>,
            DownloadStatus: DownloadStatus::<Identity, Impl, OFFSET>,
            LastDownloadError: LastDownloadError::<Identity, Impl, OFFSET>,
            LocalPath: LocalPath::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            DownloadMimeType: DownloadMimeType::<Identity, Impl, OFFSET>,
            RemoveFile: RemoveFile::<Identity, Impl, OFFSET>,
            SetFile: SetFile::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXFeedEvents_Impl: ::windows_core::BaseImpl {
    fn Error(this: &Self::This) -> ::windows_core::Result<()>;
    fn FeedDeleted(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedRenamed(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedUrlChanged(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedMoved(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedDownloading(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedDownloadCompleted(this: &Self::This, pszpath: &::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>;
    fn FeedItemCountChanged(this: &Self::This, pszpath: &::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXFeedEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Error(this).into())
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDeleted(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedRenamed(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedUrlChanged(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMoved(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloading(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloadCompleted(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&fde)).into())
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedItemCountChanged(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into())
        }
        IXFeedEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Error: Error::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMoved: FeedMoved::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXFeedFolder_Impl: ::windows_core::BaseImpl {
    fn Feeds(this: &Self::This) -> ::windows_core::Result<IXFeedsEnum>;
    fn Subfolders(this: &Self::This) -> ::windows_core::Result<IXFeedsEnum>;
    fn CreateFeed(this: &Self::This, pszname: &::windows_core::PCWSTR, pszurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateSubfolder(this: &Self::This, pszname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ExistsFeed(this: &Self::This, pszname: &::windows_core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ExistsSubfolder(this: &Self::This, pszname: &::windows_core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFeed(this: &Self::This, pszname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetSubfolder(this: &Self::This, pszname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Rename(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Move(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Parent(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsRoot(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetWatcher(this: &Self::This, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn TotalUnreadItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn TotalItemCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXFeedFolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedFolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Feeds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Feeds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Subfolders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subfolders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFeed(this, ::core::mem::transmute(&pszname), ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSubfolder(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pbfeedexists: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExistsFeed(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pbfeedexists)).into())
        }
        unsafe extern "system" fn ExistsSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pbsubfolderexists: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExistsSubfolder(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pbsubfolderexists)).into())
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeed(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetSubfolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSubfolder(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Rename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rename(this, ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Parent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn IsRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisrootfeedfolder: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisrootfeedfolder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: FEEDS_EVENTS_SCOPE, mask: FEEDS_EVENTS_MASK, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWatcher(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn TotalUnreadItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puitotalunreaditemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalUnreadItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puitotalunreaditemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puitotalitemcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puitotalitemcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXFeedFolder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Feeds: Feeds::<Identity, Impl, OFFSET>,
            Subfolders: Subfolders::<Identity, Impl, OFFSET>,
            CreateFeed: CreateFeed::<Identity, Impl, OFFSET>,
            CreateSubfolder: CreateSubfolder::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            ExistsSubfolder: ExistsSubfolder::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetSubfolder: GetSubfolder::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Rename: Rename::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            IsRoot: IsRoot::<Identity, Impl, OFFSET>,
            GetWatcher: GetWatcher::<Identity, Impl, OFFSET>,
            TotalUnreadItemCount: TotalUnreadItemCount::<Identity, Impl, OFFSET>,
            TotalItemCount: TotalItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXFeedFolderEvents_Impl: ::windows_core::BaseImpl {
    fn Error(this: &Self::This) -> ::windows_core::Result<()>;
    fn FolderAdded(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FolderDeleted(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FolderRenamed(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FolderMovedFrom(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FolderMovedTo(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FolderItemCountChanged(this: &Self::This, pszpath: &::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::Result<()>;
    fn FeedAdded(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedDeleted(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedRenamed(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedUrlChanged(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedMovedFrom(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedMovedTo(this: &Self::This, pszpath: &::windows_core::PCWSTR, pszoldpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedDownloading(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FeedDownloadCompleted(this: &Self::This, pszpath: &::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::Result<()>;
    fn FeedItemCountChanged(this: &Self::This, pszpath: &::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXFeedFolderEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedFolderEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Error(this).into())
        }
        unsafe extern "system" fn FolderAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderAdded(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FolderDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderDeleted(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FolderRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderRenamed(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FolderMovedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderMovedFrom(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FolderMovedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderMovedTo(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FolderItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FolderItemCountChanged(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into())
        }
        unsafe extern "system" fn FeedAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedAdded(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedDeleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDeleted(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedRenamed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedRenamed(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FeedUrlChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedUrlChanged(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedMovedFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMovedFrom(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FeedMovedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pszoldpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedMovedTo(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute(&pszoldpath)).into())
        }
        unsafe extern "system" fn FeedDownloading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloading(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn FeedDownloadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, fde: FEEDS_DOWNLOAD_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedDownloadCompleted(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&fde)).into())
        }
        unsafe extern "system" fn FeedItemCountChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedFolderEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, feicfflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FeedItemCountChanged(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&feicfflags)).into())
        }
        IXFeedFolderEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Error: Error::<Identity, Impl, OFFSET>,
            FolderAdded: FolderAdded::<Identity, Impl, OFFSET>,
            FolderDeleted: FolderDeleted::<Identity, Impl, OFFSET>,
            FolderRenamed: FolderRenamed::<Identity, Impl, OFFSET>,
            FolderMovedFrom: FolderMovedFrom::<Identity, Impl, OFFSET>,
            FolderMovedTo: FolderMovedTo::<Identity, Impl, OFFSET>,
            FolderItemCountChanged: FolderItemCountChanged::<Identity, Impl, OFFSET>,
            FeedAdded: FeedAdded::<Identity, Impl, OFFSET>,
            FeedDeleted: FeedDeleted::<Identity, Impl, OFFSET>,
            FeedRenamed: FeedRenamed::<Identity, Impl, OFFSET>,
            FeedUrlChanged: FeedUrlChanged::<Identity, Impl, OFFSET>,
            FeedMovedFrom: FeedMovedFrom::<Identity, Impl, OFFSET>,
            FeedMovedTo: FeedMovedTo::<Identity, Impl, OFFSET>,
            FeedDownloading: FeedDownloading::<Identity, Impl, OFFSET>,
            FeedDownloadCompleted: FeedDownloadCompleted::<Identity, Impl, OFFSET>,
            FeedItemCountChanged: FeedItemCountChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItem_Impl: ::windows_core::BaseImpl {
    fn Xml(this: &Self::This, fxif: FEEDS_XML_INCLUDE_FLAGS) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Link(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Guid(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn PubDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Comments(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Author(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Enclosure(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsRead(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsRead(this: &Self::This, bisread: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn LocalId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Parent(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn DownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn LastDownloadTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn Modified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXFeedItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fxif: FEEDS_XML_INCLUDE_FLAGS, pps: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this, ::core::mem::transmute_copy(&fxif)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Guid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszguid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Guid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PubDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstpubdate: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PubDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstpubdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Comments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Comments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszauthor: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Author(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszauthor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enclosure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enclosure(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn IsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbisread: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bisread: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsRead(this, ::core::mem::transmute_copy(&bisread)).into())
        }
        unsafe extern "system" fn LocalId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Parent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn DownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastDownloadTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstlastdownloadtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastDownloadTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstlastdownloadtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Modified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstmodifiedtime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Modified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstmodifiedtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXFeedItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Xml: Xml::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            Guid: Guid::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            PubDate: PubDate::<Identity, Impl, OFFSET>,
            Comments: Comments::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Enclosure: Enclosure::<Identity, Impl, OFFSET>,
            IsRead: IsRead::<Identity, Impl, OFFSET>,
            SetIsRead: SetIsRead::<Identity, Impl, OFFSET>,
            LocalId: LocalId::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DownloadUrl: DownloadUrl::<Identity, Impl, OFFSET>,
            LastDownloadTime: LastDownloadTime::<Identity, Impl, OFFSET>,
            Modified: Modified::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedItem2_Impl: ::windows_core::BaseImpl + IXFeedItem_Impl {
    fn EffectiveId(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXFeedItem2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXFeedItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedItem2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EffectiveId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedItem2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puieffectiveid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EffectiveId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puieffectiveid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXFeedItem2_Vtbl { base__: <IXFeedItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EffectiveId: EffectiveId::<Identity, Impl, OFFSET> }
    };
}
pub trait IXFeedsEnum_Impl: ::windows_core::BaseImpl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Item(this: &Self::This, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXFeedsEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedsEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puicount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Item(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IXFeedsEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXFeedsManager_Impl: ::windows_core::BaseImpl {
    fn RootFolder(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsSubscribed(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ExistsFeed(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFeed(this: &Self::This, pszpath: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetFeedByUrl(this: &Self::This, pszurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ExistsFolder(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFolder(this: &Self::This, pszpath: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DeleteFeed(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeleteFolder(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn BackgroundSync(this: &Self::This, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::Result<()>;
    fn BackgroundSyncStatus(this: &Self::This) -> ::windows_core::Result<FEEDS_BACKGROUNDSYNC_STATUS>;
    fn DefaultInterval(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetDefaultInterval(this: &Self::This, uiinterval: u32) -> ::windows_core::Result<()>;
    fn AsyncSyncAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Normalize(this: &Self::This, pstreamin: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn ItemCountLimit(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXFeedsManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXFeedsManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RootFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RootFolder(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pbsubscribed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSubscribed(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsubscribed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExistsFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbfeedexists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsFeed(this, ::core::mem::transmute(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbfeedexists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeed(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetFeedByUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFeedByUrl(this, ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn ExistsFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, pbfolderexists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExistsFolder(this, ::core::mem::transmute(&pszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbfolderexists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFolder(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn DeleteFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFeed(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn DeleteFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFolder(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn BackgroundSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fbsa: FEEDS_BACKGROUNDSYNC_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackgroundSync(this, ::core::mem::transmute_copy(&fbsa)).into())
        }
        unsafe extern "system" fn BackgroundSyncStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfbss: *mut FEEDS_BACKGROUNDSYNC_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackgroundSyncStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbss, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiinterval: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultInterval(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiinterval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiinterval: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultInterval(this, ::core::mem::transmute_copy(&uiinterval)).into())
        }
        unsafe extern "system" fn AsyncSyncAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncSyncAll(this).into())
        }
        unsafe extern "system" fn Normalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstreamin: *mut ::core::ffi::c_void, ppstreamout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Normalize(this, ::windows_core::from_raw_borrowed(&pstreamin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstreamout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemCountLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXFeedsManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiitemcountlimit: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemCountLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puiitemcountlimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXFeedsManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RootFolder: RootFolder::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            ExistsFeed: ExistsFeed::<Identity, Impl, OFFSET>,
            GetFeed: GetFeed::<Identity, Impl, OFFSET>,
            GetFeedByUrl: GetFeedByUrl::<Identity, Impl, OFFSET>,
            ExistsFolder: ExistsFolder::<Identity, Impl, OFFSET>,
            GetFolder: GetFolder::<Identity, Impl, OFFSET>,
            DeleteFeed: DeleteFeed::<Identity, Impl, OFFSET>,
            DeleteFolder: DeleteFolder::<Identity, Impl, OFFSET>,
            BackgroundSync: BackgroundSync::<Identity, Impl, OFFSET>,
            BackgroundSyncStatus: BackgroundSyncStatus::<Identity, Impl, OFFSET>,
            DefaultInterval: DefaultInterval::<Identity, Impl, OFFSET>,
            SetDefaultInterval: SetDefaultInterval::<Identity, Impl, OFFSET>,
            AsyncSyncAll: AsyncSyncAll::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            ItemCountLimit: ItemCountLimit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _WMPOCXEvents_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _WMPOCXEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _WMPOCXEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _WMPOCXEvents {
    const VTABLE: Self::Vtable = { _WMPOCXEvents_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}

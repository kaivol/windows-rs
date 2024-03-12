#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IInkAnalysisNode_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Kind(this: &Self::This) -> ::windows_core::Result<InkAnalysisNodeKind>;
    fn BoundingRect(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::Rect>;
    fn RotatedBoundingRect(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>;
    fn Children(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>;
    fn Parent(this: &Self::This) -> ::windows_core::Result<IInkAnalysisNode>;
    fn GetStrokeIds(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IInkAnalysisNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkAnalysisNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BoundingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundingRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RotatedBoundingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RotatedBoundingRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStrokeIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalysisNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkAnalysisNode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Kind: Kind::<Identity, Impl, OFFSET>,
            BoundingRect: BoundingRect::<Identity, Impl, OFFSET>,
            RotatedBoundingRect: RotatedBoundingRect::<Identity, Impl, OFFSET>,
            Children: Children::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            GetStrokeIds: GetStrokeIds::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInkAnalyzerFactory_Impl: ::windows_core::BaseImpl {
    fn CreateAnalyzer(this: &Self::This) -> ::windows_core::Result<InkAnalyzer>;
}
impl ::windows_core::Iids for IInkAnalyzerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkAnalyzerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAnalyzer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkAnalyzerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAnalyzer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkAnalyzerFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateAnalyzer: CreateAnalyzer::<Identity, Impl, OFFSET>,
        }
    };
}

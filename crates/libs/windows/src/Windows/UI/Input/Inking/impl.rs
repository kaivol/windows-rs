#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IInkPointFactory_Impl: ::windows_core::BaseImpl {
    fn CreateInkPoint(this: &Self::This, position: &super::super::super::Foundation::Point, pressure: f32) -> ::windows_core::Result<InkPoint>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IInkPointFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPointFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkPointFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInkPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPointFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInkPoint(this, ::core::mem::transmute(&position), pressure) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkPointFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInkPoint: CreateInkPoint::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInkPresenterRulerFactory_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, inkpresenter: ::core::option::Option<&InkPresenter>) -> ::windows_core::Result<InkPresenterRuler>;
}
impl ::windows_core::Iids for IInkPresenterRulerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterRulerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkPresenterRulerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterRulerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inkpresenter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::windows_core::from_raw_borrowed(&inkpresenter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkPresenterRulerFactory_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Create: Create::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Foundation_Numerics\"`"]
#[cfg(feature = "Foundation_Numerics")]
pub trait IInkPresenterStencil_Impl: ::windows_core::BaseImpl {
    fn Kind(this: &Self::This) -> ::windows_core::Result<InkPresenterStencilKind>;
    fn IsVisible(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsVisible(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn BackgroundColor(this: &Self::This) -> ::windows_core::Result<super::super::Color>;
    fn SetBackgroundColor(this: &Self::This, value: &super::super::Color) -> ::windows_core::Result<()>;
    fn ForegroundColor(this: &Self::This) -> ::windows_core::Result<super::super::Color>;
    fn SetForegroundColor(this: &Self::This, value: &super::super::Color) -> ::windows_core::Result<()>;
    fn Transform(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(this: &Self::This, value: &super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows_core::Iids for IInkPresenterStencil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Numerics")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkPresenterStencil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Kind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsVisible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsVisible(this, value).into())
        }
        unsafe extern "system" fn BackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackgroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackgroundColor(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ForegroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForegroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForegroundColor(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Transform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Transform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterStencil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransform(this, ::core::mem::transmute(&value)).into())
        }
        IInkPresenterStencil_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Kind: Kind::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
            SetIsVisible: SetIsVisible::<Identity, Impl, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, Impl, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
pub trait IInkRecognizerContainer_Impl: ::windows_core::BaseImpl {
    fn SetDefaultRecognizer(this: &Self::This, recognizer: ::core::option::Option<&InkRecognizer>) -> ::windows_core::Result<()>;
    fn RecognizeAsync(this: &Self::This, strokecollection: ::core::option::Option<&InkStrokeContainer>, recognitiontarget: InkRecognitionTarget) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>;
    fn GetRecognizers(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::Iids for IInkRecognizerContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation_Collections")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkRecognizerContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDefaultRecognizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recognizer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultRecognizer(this, ::windows_core::from_raw_borrowed(&recognizer)).into())
        }
        unsafe extern "system" fn RecognizeAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokecollection: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecognizeAsync(this, ::windows_core::from_raw_borrowed(&strokecollection), recognitiontarget) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecognizers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkRecognizerContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecognizers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkRecognizerContainer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDefaultRecognizer: SetDefaultRecognizer::<Identity, Impl, OFFSET>,
            RecognizeAsync: RecognizeAsync::<Identity, Impl, OFFSET>,
            GetRecognizers: GetRecognizers::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
pub trait IInkStrokeContainer_Impl: ::windows_core::BaseImpl {
    fn BoundingRect(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn AddStroke(this: &Self::This, stroke: ::core::option::Option<&InkStroke>) -> ::windows_core::Result<()>;
    fn DeleteSelected(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn MoveSelected(this: &Self::This, translation: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithPolyLine(this: &Self::This, polyline: ::core::option::Option<&super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn SelectWithLine(this: &Self::This, from: &super::super::super::Foundation::Point, to: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn CopySelectedToClipboard(this: &Self::This) -> ::windows_core::Result<()>;
    fn PasteFromClipboard(this: &Self::This, position: &super::super::super::Foundation::Point) -> ::windows_core::Result<super::super::super::Foundation::Rect>;
    fn CanPasteFromClipboard(this: &Self::This) -> ::windows_core::Result<bool>;
    fn LoadAsync(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::Storage::Streams::IInputStream>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>>;
    fn SaveAsync(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::Storage::Streams::IOutputStream>) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn UpdateRecognitionResults(this: &Self::This, recognitionresults: ::core::option::Option<&super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>) -> ::windows_core::Result<()>;
    fn GetStrokes(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>>;
    fn GetRecognitionResults(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl ::windows_core::Iids for IInkStrokeContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkStrokeContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BoundingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundingRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddStroke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStroke(this, ::windows_core::from_raw_borrowed(&stroke)).into())
        }
        unsafe extern "system" fn DeleteSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeleteSelected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveSelected(this, ::core::mem::transmute(&translation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectWithPolyLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, polyline: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectWithPolyLine(this, ::windows_core::from_raw_borrowed(&polyline)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectWithLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectWithLine(this, ::core::mem::transmute(&from), ::core::mem::transmute(&to)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopySelectedToClipboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySelectedToClipboard(this).into())
        }
        unsafe extern "system" fn PasteFromClipboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PasteFromClipboard(this, ::core::mem::transmute(&position)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanPasteFromClipboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanPasteFromClipboard(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadAsync(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SaveAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveAsync(this, ::windows_core::from_raw_borrowed(&outputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateRecognitionResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recognitionresults: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateRecognitionResults(this, ::windows_core::from_raw_borrowed(&recognitionresults)).into())
        }
        unsafe extern "system" fn GetStrokes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecognitionResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkStrokeContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecognitionResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInkStrokeContainer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BoundingRect: BoundingRect::<Identity, Impl, OFFSET>,
            AddStroke: AddStroke::<Identity, Impl, OFFSET>,
            DeleteSelected: DeleteSelected::<Identity, Impl, OFFSET>,
            MoveSelected: MoveSelected::<Identity, Impl, OFFSET>,
            SelectWithPolyLine: SelectWithPolyLine::<Identity, Impl, OFFSET>,
            SelectWithLine: SelectWithLine::<Identity, Impl, OFFSET>,
            CopySelectedToClipboard: CopySelectedToClipboard::<Identity, Impl, OFFSET>,
            PasteFromClipboard: PasteFromClipboard::<Identity, Impl, OFFSET>,
            CanPasteFromClipboard: CanPasteFromClipboard::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            SaveAsync: SaveAsync::<Identity, Impl, OFFSET>,
            UpdateRecognitionResults: UpdateRecognitionResults::<Identity, Impl, OFFSET>,
            GetStrokes: GetStrokes::<Identity, Impl, OFFSET>,
            GetRecognitionResults: GetRecognitionResults::<Identity, Impl, OFFSET>,
        }
    };
}

pub trait ID2D1SimplifiedGeometrySink_Impl: ::windows_core::BaseImpl {
    fn SetFillMode(this: &Self::This, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(this: &Self::This, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(this: &Self::This, startpoint: &D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(this: &Self::This, points: *const D2D_POINT_2F, pointscount: u32);
    fn AddBeziers(this: &Self::This, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(this: &Self::This, figureend: D2D1_FIGURE_END);
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ID2D1SimplifiedGeometrySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ID2D1SimplifiedGeometrySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFillMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillmode: D2D1_FILL_MODE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillMode(this, ::core::mem::transmute_copy(&fillmode)))
        }
        unsafe extern "system" fn SetSegmentFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vertexflags: D2D1_PATH_SEGMENT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSegmentFlags(this, ::core::mem::transmute_copy(&vertexflags)))
        }
        unsafe extern "system" fn BeginFigure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginFigure(this, ::core::mem::transmute(&startpoint), ::core::mem::transmute_copy(&figurebegin)))
        }
        unsafe extern "system" fn AddLines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, points: *const D2D_POINT_2F, pointscount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLines(this, ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointscount)))
        }
        unsafe extern "system" fn AddBeziers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBeziers(this, ::core::mem::transmute_copy(&beziers), ::core::mem::transmute_copy(&bezierscount)))
        }
        unsafe extern "system" fn EndFigure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, figureend: D2D1_FIGURE_END) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndFigure(this, ::core::mem::transmute_copy(&figureend)))
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        ID2D1SimplifiedGeometrySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFillMode: SetFillMode::<Identity, Impl, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, Impl, OFFSET>,
            BeginFigure: BeginFigure::<Identity, Impl, OFFSET>,
            AddLines: AddLines::<Identity, Impl, OFFSET>,
            AddBeziers: AddBeziers::<Identity, Impl, OFFSET>,
            EndFigure: EndFigure::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}

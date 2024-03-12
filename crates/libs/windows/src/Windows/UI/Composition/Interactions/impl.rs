pub trait ICompositionInteractionSource_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ICompositionInteractionSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionInteractionSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionInteractionSource {
    const VTABLE: Self::Vtable = { ICompositionInteractionSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IInteractionTrackerOwner_Impl: ::windows_core::BaseImpl {
    fn CustomAnimationStateEntered(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerCustomAnimationStateEnteredArgs>) -> ::windows_core::Result<()>;
    fn IdleStateEntered(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerIdleStateEnteredArgs>) -> ::windows_core::Result<()>;
    fn InertiaStateEntered(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerInertiaStateEnteredArgs>) -> ::windows_core::Result<()>;
    fn InteractingStateEntered(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerInteractingStateEnteredArgs>) -> ::windows_core::Result<()>;
    fn RequestIgnored(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerRequestIgnoredArgs>) -> ::windows_core::Result<()>;
    fn ValuesChanged(this: &Self::This, sender: ::core::option::Option<&InteractionTracker>, args: ::core::option::Option<&InteractionTrackerValuesChangedArgs>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInteractionTrackerOwner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInteractionTrackerOwner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CustomAnimationStateEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CustomAnimationStateEntered(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        unsafe extern "system" fn IdleStateEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IdleStateEntered(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        unsafe extern "system" fn InertiaStateEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InertiaStateEntered(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        unsafe extern "system" fn InteractingStateEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InteractingStateEntered(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        unsafe extern "system" fn RequestIgnored<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestIgnored(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        unsafe extern "system" fn ValuesChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInteractionTrackerOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValuesChanged(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&args)).into())
        }
        IInteractionTrackerOwner_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CustomAnimationStateEntered: CustomAnimationStateEntered::<Identity, Impl, OFFSET>,
            IdleStateEntered: IdleStateEntered::<Identity, Impl, OFFSET>,
            InertiaStateEntered: InertiaStateEntered::<Identity, Impl, OFFSET>,
            InteractingStateEntered: InteractingStateEntered::<Identity, Impl, OFFSET>,
            RequestIgnored: RequestIgnored::<Identity, Impl, OFFSET>,
            ValuesChanged: ValuesChanged::<Identity, Impl, OFFSET>,
        }
    };
}

use super::*;

pub fn writer(writer: &Writer, def: TypeDef) -> TokenStream {
    if def.kind() != TypeKind::Interface || (!writer.implement && def.has_attribute("ExclusiveToAttribute")) {
        return quote! {};
    }

    if matches!(type_def_vtables(def).first(), Some(Type::IUnknown)) {
        gen_unknown_base(writer, def)
    } else {
        gen_no_unknown_base(writer, def)
    }
}

pub fn gen_unknown_base(writer: &Writer, def: TypeDef) -> TokenStream {
    let generics = &type_def_generics(def);
    let type_ident = to_ident(def.name());
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let constraints = writer.generic_constraints(generics);
    let generic_names = writer.generic_names(generics);
    let named_phantoms = writer.generic_named_phantoms(generics);
    let cfg = type_def_cfg_impl(def, generics);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let type_ident = quote! { #type_ident<#generic_names> };
    let vtables = type_def_vtables(def);

    let mut required_traits = vec![];
    let mut required_iids = vec![];

    let parent = match vtables.last() {
        Some(Type::IUnknown) => quote! { ::windows_core::IUnknown },
        Some(Type::IInspectable) => quote! { ::windows_core::IInspectable },
        Some(Type::TypeDef(def, generics)) => {
            required_traits.push(writer.type_def_name_imp(*def, generics, "_Impl"));
            writer.type_def_name_imp(*def, generics, "")
        }
        _ => panic!(),
    };

    if def.flags().contains(TypeAttributes::WindowsRuntime) {
        // TODO: this awkward wrapping of TypeDefs needs fixing
        for interface in type_interfaces(&Type::TypeDef(def, generics.to_vec())) {
            if let Type::TypeDef(def, generics) = interface.ty {
                required_traits.push(writer.type_def_name_imp(def, &generics, "_Impl"));
                required_iids.push(writer.type_def_name_imp(def, &generics, ""));
            }
        }
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_traits = def.methods().map(|method| {
        let name = method_names.add(method);
        let signature = method_def_signature(def.namespace(), method, generics);
        let signature_tokens = writer.impl_signature(def, &signature, true);
        quote! { fn #name #signature_tokens; }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_impls = def.methods().map(|method| {
        let name = method_names.add(method);
        let signature = method_def_signature(def.namespace(), method, generics);
        let vtbl_signature = writer.vtbl_signature(def, generics, &signature);

        let invoke_upcall = if def.flags().contains(TypeAttributes::WindowsRuntime) { winrt_methods::gen_upcall(writer, &signature, quote! { Impl::#name }, Some(quote! { this })) } else { com_methods::gen_upcall(writer, &signature, quote! { Impl::#name }) };

        quote! {
            unsafe extern "system" fn #name<#constraints Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: #impl_ident<#generic_names>, const OFFSET: usize> #vtbl_signature {
                Identity::call_impl::<_,OFFSET>(this, |this| {
                    #invoke_upcall
                })
            }
        }
    });

    let mut methods = quote! {};
    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    for method in def.methods() {
        let name = method_names.add(method);
        methods.combine(&quote! { #name: #name::<#generic_names Identity, Impl, OFFSET>, });
    }

    let required_iids = (!required_iids.is_empty()).then(|| {
        quote! {
            const REQUIRED_IIDS: &'static [::windows_core::GUID] =
                &[#(<#required_iids as ::windows_core::ComInterface>::IID,)*];
        }
    });

    quote! {
        #doc
        #features
        pub trait #impl_ident<#generic_names> : ::windows_core::BaseImpl + #(#required_traits)+* where #constraints {
            #(#method_traits)*
        }
        #features
        impl<#constraints> ::windows_core::Iids for #type_ident {
            const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(#parent);
            #required_iids
        }
        #features
        impl<
            Identity: ::windows_core::ImplProvider<Impl = Impl>,
            Impl: #impl_ident<#generic_names>,
            const OFFSET: usize,
            #constraints
        > ::windows_core::Vtable<Identity, OFFSET> for #type_ident {
            const VTABLE: Self::Vtable = {
                #(#method_impls)*
                #vtbl_ident {
                    base__: <#parent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
                    #methods
                    #(#named_phantoms)*
                }
            };
            const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
        }
    }
}

pub fn gen_no_unknown_base(writer: &Writer, def: TypeDef) -> TokenStream {
    let generics = &type_def_generics(def);
    let type_ident = to_ident(def.name());
    let impl_ident = type_ident.join("_Impl");
    let vtbl_ident = type_ident.join("_Vtbl");
    let implvtbl_ident = impl_ident.join("Vtbl");
    let generic_names = writer.generic_names(generics);
    let named_phantoms = writer.generic_named_phantoms(generics);
    let cfg = type_def_cfg_impl(def, generics);
    let doc = writer.cfg_doc(&cfg);
    let features = writer.cfg_features(&cfg);
    let type_ident = quote! { #type_ident<#generic_names> };
    let vtables = type_def_vtables(def);

    let requires = if let Some(Type::TypeDef(def, _)) = vtables.last() { Some(writer.type_def_name_imp(*def, &[], "_Impl")) } else { None };

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_traits = def.methods().map(|method| {
        let name = method_names.add(method);
        let signature = method_def_signature(def.namespace(), method, generics);
        let signature_tokens = writer.impl_signature(def, &signature, false);
        quote! { fn #name #signature_tokens; }
    });

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    let method_impls = def.methods().map(|method| {
        let name = method_names.add(method);
        let signature = method_def_signature(def.namespace(), method, generics);
        let vtbl_signature = writer.vtbl_signature(def, generics, &signature);

        let invoke_upcall = com_methods::gen_upcall(writer, &signature, quote! { Impl::#name });

        quote! {
            unsafe extern "system" fn #name<Impl: #impl_ident> #vtbl_signature {
                let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
                let this = &*((*this).this as *const Impl);
                #invoke_upcall
            }
        }
    });

    let mut methods = quote! {};

    if let Some(Type::TypeDef(def, generics)) = vtables.last() {
        let name = writer.type_def_name_imp(*def, generics, "_Vtbl");
        methods.combine(&quote! { base__: #name::new::<Impl>(), });
    }

    let mut method_names = MethodNames::new();
    method_names.add_vtable_types(def);

    for method in def.methods() {
        let name = method_names.add(method);
        methods.combine(&quote! { #name: #name::<Impl>, });
    }

    quote! {
        #doc
        #features
        pub trait #impl_ident : Sized + #requires {
            #(#method_traits)*
        }
        #features
        impl #vtbl_ident {
            pub const fn new<Impl: #impl_ident>() -> #vtbl_ident {
                #(#method_impls)*
                Self{
                    #methods
                    #(#named_phantoms)*
                }
            }
        }
        #[doc(hidden)]
        #features
        struct #implvtbl_ident<T: #impl_ident> (::std::marker::PhantomData<T>);
        #features
        impl<T: #impl_ident> #implvtbl_ident<T> {
            const VTABLE: #vtbl_ident = #vtbl_ident::new::<T>();
        }
        #features
        impl #type_ident {
            pub fn new<'a, T: #impl_ident>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
                let this = ::windows_core::ScopedHeap { vtable: &#implvtbl_ident::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
                let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
                unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
            }
        }
    }
}

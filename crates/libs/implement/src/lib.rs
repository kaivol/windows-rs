use quote::{quote, ToTokens};

/// Implements one or more COM interfaces.
///
/// # Example
/// ```rust,ignore
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// fn main() {
///     let object: IValue = Value(123).into();
///     // Call interface methods...
/// }
/// ```
#[proc_macro_attribute]
pub fn implement(attributes: proc_macro::TokenStream, original_type: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attributes = syn::parse_macro_input!(attributes as ImplementAttributes);
    let interfaces = attributes.implement;

    let original_type2 = original_type.clone();
    let original_type2 = syn::parse_macro_input!(original_type2 as syn::ItemStruct);
    let original_ident = original_type2.ident;
    let mut constraints = quote! {};

    if let Some(where_clause) = original_type2.generics.where_clause {
        where_clause.predicates.to_tokens(&mut constraints);
    }

    let generics = if original_type2.generics.lt_token.is_some() {
        let mut params = quote! {};
        original_type2.generics.params.to_tokens(&mut params);
        quote! { <#params> }
    } else {
        quote! { <> }
    };

    let original_type: proc_macro2::TokenStream = original_type.into();
    let interface_names = interfaces.iter().map(ImplementType::to_ident);
    let trust_level = proc_macro2::Literal::usize_unsuffixed(attributes.trust_level);
    quote! {
        #original_type

        impl #generics ::windows::core::ComObjectImpl for #original_ident #generics where #constraints {
            type Interfaces = (#(#interface_names,)*);

            fn get_trust_level() -> i32 {
                #trust_level
            }
        }
    }
    .into()
}

#[derive(Default, Clone)]
struct ImplementType {
    type_name: String,
    generics: Vec<ImplementType>,
}

impl ImplementType {
    fn to_ident(&self) -> proc_macro2::TokenStream {
        let type_name = syn::parse_str::<proc_macro2::TokenStream>(&self.type_name).expect("Invalid token stream");
        let generics = self.generics.iter().map(|g| g.to_ident());
        quote! { #type_name::<#(#generics,)*> }
    }
}

#[derive(Default)]
struct ImplementAttributes {
    pub implement: Vec<ImplementType>,
    pub trust_level: usize,
}

impl syn::parse::Parse for ImplementAttributes {
    fn parse(cursor: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let mut input = Self::default();

        while !cursor.is_empty() {
            input.parse_implement(cursor)?;
        }

        Ok(input)
    }
}

impl ImplementAttributes {
    fn parse_implement(&mut self, cursor: syn::parse::ParseStream) -> syn::parse::Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(&tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<syn::Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(&mut self, tree: &UseTree2, namespace: &mut String) -> syn::parse::Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(&input.tree, namespace)?;
            }
            UseTree2::Name(_) => {
                self.implement.push(tree.to_element_type(namespace)?);
            }
            UseTree2::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(tree, namespace)?;
                }
            }
            UseTree2::TrustLevel(input) => self.trust_level = *input,
        }

        Ok(())
    }
}

enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
    TrustLevel(usize),
}

impl UseTree2 {
    fn to_element_type(&self, namespace: &mut String) -> syn::parse::Result<ImplementType> {
        match self {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push_str("::");
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(namespace)
            }
            UseTree2::Name(input) => {
                let mut type_name = input.ident.to_string();

                if !namespace.is_empty() {
                    type_name = format!("{namespace}::{type_name}");
                }

                let mut generics = vec![];

                for g in &input.generics {
                    generics.push(g.to_element_type(&mut String::new())?);
                }

                Ok(ImplementType { type_name, generics })
            }
            UseTree2::Group(input) => Err(syn::parse::Error::new(input.brace_token.span.join(), "Syntax not supported")),
            _ => unimplemented!(),
        }
    }
}

struct UsePath2 {
    pub ident: syn::Ident,
    pub tree: Box<UseTree2>,
}

struct UseName2 {
    pub ident: syn::Ident,
    pub generics: Vec<UseTree2>,
}

struct UseGroup2 {
    pub brace_token: syn::token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, syn::Token![,]>,
}

impl syn::parse::Parse for UseTree2 {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(syn::Ident::parse_any)?;
            if input.peek(syn::Token![::]) {
                input.parse::<syn::Token![::]>()?;
                Ok(UseTree2::Path(UsePath2 { ident, tree: Box::new(input.parse()?) }))
            } else if input.peek(syn::Token![=]) {
                if ident != "TrustLevel" {
                    return Err(syn::parse::Error::new(ident.span(), "Unrecognized key-value pair"));
                }
                input.parse::<syn::Token![=]>()?;
                let span = input.span();
                let value = input.call(syn::Ident::parse_any)?;
                match value.to_string().as_str() {
                    "Partial" => Ok(UseTree2::TrustLevel(1)),
                    "Full" => Ok(UseTree2::TrustLevel(2)),
                    _ => Err(syn::parse::Error::new(span, "`TrustLevel` must be `Partial` or `Full`")),
                }
            } else {
                let generics = if input.peek(syn::Token![<]) {
                    input.parse::<syn::Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<syn::Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<syn::Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(syn::token::Brace) {
            let content;
            let brace_token = syn::braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse, syn::Token![,])?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}

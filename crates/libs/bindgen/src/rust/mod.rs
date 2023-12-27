mod cfg;
mod classes;
mod com_methods;
mod constants;
mod delegates;
mod enums;
mod extensions;
mod functions;
mod handles;
mod implements;
mod interfaces;
mod iterators;
mod method_names;
mod standalone;
mod structs;
mod try_format;
mod winrt_methods;
mod writer;
use super::*;
use crate::{metadata, Error, Result, Tree};
use cfg::*;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn from_reader(reader: &'static metadata::Reader, mut config: BTreeMap<&str, &str>, output: &str, externals: StringPatriciaMap<Option<String>>) -> Result<()> {
    let mut writer = Writer::new(reader, output, externals);
    writer.package = config.remove("package").is_some();
    writer.flatten = config.remove("flatten").is_some();
    writer.std = config.remove("std").is_some();
    writer.sys = writer.std || config.remove("sys").is_some();
    writer.implement = config.remove("implement").is_some();
    writer.minimal = config.remove("minimal").is_some();
    writer.no_inner_attributes = config.remove("no-inner-attributes").is_some();
    writer.single_namespace = config.remove("single-namespace").is_some();
    writer.default_namespaces = config.remove("default-namespaces").map(|namespaces| namespaces.split(',').map(ToOwned::to_owned).collect()).unwrap_or_default();

    if writer.package && writer.flatten {
        return Err(Error::new("cannot combine `package` and `flatten` configuration values"));
    }

    if writer.implement && writer.sys {
        return Err(Error::new("cannot combine `implement` and `sys` configuration values"));
    }

    if let Some((key, _)) = config.first_key_value() {
        return Err(Error::new(&format!("invalid configuration value `{key}`")));
    }

    if writer.single_namespace && !writer.externals.is_empty() {
        return Err(Error::new("cannot combine `single-namespace` and external dependencies"));
    }

    if writer.package {
        gen_package(&writer)
    } else {
        gen_file(&writer)
    }
}

fn gen_file(writer: &Writer) -> Result<()> {
    // TODO: harmonize this output code so we don't need these two wildly differnt code paths
    // there should be a simple way to generate the with or without namespaces.

    if writer.flatten {
        let tokens = standalone::standalone_imp(writer);
        write_to_file(&writer.output, try_format(writer, &tokens))
    } else {
        let mut tokens = String::new();
        let root = Tree::new(writer.reader);

        for tree in root.nested.values() {
            tokens.push_str(&namespace(writer, tree));
        }

        write_to_file(&writer.output, try_format(writer, &tokens))
    }
}

pub fn gen_package(writer: &Writer) -> Result<()> {
    let directory = directory(&writer.output);
    let root = Tree::new(writer.reader);

    for tree in root.nested.values() {
        _ = std::fs::remove_dir_all(format!("{directory}/{}", tree.namespace));
    }

    let trees = root.flatten();

    let result = trees
        .par_iter()
        .map(|tree| {
            let directory = format!("{directory}/{}", tree.namespace.replace('.', "/"));
            let mut tokens = namespace(writer, tree);

            let tokens_impl = if !writer.sys { namespace_impl(writer, tree) } else { String::new() };

            if !writer.sys && !tokens_impl.is_empty() {
                tokens.push_str("#[cfg(feature = \"implement\")]\n::core::include!(\"impl.rs\");\n");
            }

            let output = format!("{directory}/mod.rs");
            write_to_file(&output, try_format(writer, &tokens))?;

            if !writer.sys && !tokens_impl.is_empty() {
                let output = format!("{directory}/impl.rs");
                write_to_file(&output, try_format(writer, &tokens_impl))?;
            }

            let mut dependencies = BTreeSet::new();

            for item in writer.reader.namespace_items(tree.namespace) {
                match item {
                    Item::Type(typ) => {
                        dependencies.extend(type_def_cfg(typ, &[]).types.into_keys());
                        dependencies.extend(typ.methods().flat_map(|method| signature_cfg(method).types.into_keys()));
                    }
                    Item::Fn(method, _) => dependencies.extend(signature_cfg(method).types.into_keys()),
                    Item::Const(field) => dependencies.extend(field_cfg(field).types.into_keys()),
                }
            }

            Ok::<_, Error>((tree.namespace, dependencies))
        })
        .collect::<Result<Vec<_>>>()?;

    let cargo_toml = format!("{}/Cargo.toml", super::directory(directory));
    let mut toml = String::new();

    for line in read_file_lines(&cargo_toml)? {
        toml.push_str(&line);
        toml.push('\n');

        if line == "# generated features" {
            break;
        }
    }

    let mut features = BTreeMap::<String, Vec<String>>::new();

    result.iter().flat_map(|(_, dep)| dep).copied().filter(|&namespace| writer.external_crate(namespace).is_some()).collect::<BTreeSet<_>>().into_iter().for_each(|namespace| {
        let (prefix, crat) = writer.external_crate(namespace).unwrap();
        let sub = namespace.trim_start_matches(prefix).trim_start_matches('.').replace('.', "_");
        let sub = crat.split_once("::").map_or_else(
            || format!("{crat}/{sub}"),
            |(crat, begin_sub)| {
                let begin_sub = begin_sub.replace("::", "_");
                if sub.is_empty() {
                    format!("{crat}/{begin_sub}")
                } else {
                    format!("{crat}/{begin_sub}_{sub}")
                }
            },
        );

        features.entry(writer.to_feature(namespace)).or_default().push(sub);
    });

    for (namespace, sub_dependencies) in result.into_iter().skip(if writer.single_namespace { 1 } else { 0 }) {
        let feature = writer.to_feature(namespace);

        let deps = features.entry(feature.clone()).or_default();

        if let Some(pos) = feature.rfind('_') {
            deps.push(feature[..pos].to_owned());
        }

        sub_dependencies.into_iter().filter(|&dep| writer.default_namespaces.contains(dep) && dep != namespace).for_each(|d| deps.push(writer.to_feature(d)));
    }

    toml.extend(features.into_iter().map(|(feature, deps)| {
        if deps.is_empty() {
            format!("{feature} = []\n")
        } else {
            let deps = deps.join("\", \"");
            format!("{feature} = [\"{deps}\"]\n")
        }
    }));

    write_to_file(&cargo_toml, toml)
}

use method_names::*;
use std::fmt::Write;
use tokens::*;
use try_format::*;
use windows_metadata::Item;
use writer::*;

fn namespace(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut tokens = TokenStream::new();

    for (name, tree) in &tree.nested {
        let name = to_ident(name);
        let feature = writer.to_feature(tree.namespace);
        let doc = format!(r#"Required features: `\"{feature}\"`"#);
        if writer.package {
            tokens.combine(&quote! {
                #[cfg(feature = #feature)]
                #[doc = #doc]
                pub mod #name;
            });
        } else {
            tokens.combine(&quote! { pub mod #name });
            tokens.push_str("{");
            tokens.push_str(&namespace(writer, tree));
            tokens.push_str("}");
        }
    }

    let mut functions = std::collections::BTreeMap::<&str, TokenStream>::new();
    let mut types = std::collections::BTreeMap::<metadata::TypeKind, BTreeMap<&str, TokenStream>>::new();

    for item in writer.reader.namespace_items(writer.namespace) {
        match item {
            metadata::Item::Type(def) => {
                let type_name = def.type_name();
                if metadata::REMAP_TYPES.iter().any(|(x, _)| x == &type_name) {
                    continue;
                }
                if metadata::CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
                    continue;
                }
                let name = type_name.name;
                let kind = def.kind();
                match kind {
                    metadata::TypeKind::Class => {
                        if def.flags().contains(metadata::TypeAttributes::WindowsRuntime) {
                            types.entry(kind).or_default().insert(name, classes::writer(writer, def));
                        }
                    }
                    metadata::TypeKind::Interface => types.entry(kind).or_default().entry(name).or_default().combine(&interfaces::writer(writer, def)),
                    metadata::TypeKind::Enum => types.entry(kind).or_default().entry(name).or_default().combine(&enums::writer(writer, def)),
                    metadata::TypeKind::Struct => {
                        if def.fields().next().is_none() {
                            if let Some(guid) = metadata::type_def_guid(def) {
                                let ident = to_ident(name);
                                let value = writer.guid(&guid);
                                let guid = writer.type_name(&metadata::Type::GUID);
                                let cfg = type_def_cfg(def, &[]);
                                let doc = writer.cfg_doc(&cfg);
                                let constant = quote! {
                                    #doc
                                    pub const #ident: #guid = #value;
                                };
                                types.entry(metadata::TypeKind::Class).or_default().entry(name).or_default().combine(&constant);
                                continue;
                            }
                        }
                        types.entry(kind).or_default().entry(name).or_default().combine(&structs::writer(writer, def));
                    }
                    metadata::TypeKind::Delegate => types.entry(kind).or_default().entry(name).or_default().combine(&delegates::writer(writer, def)),
                }
            }
            metadata::Item::Fn(def, namespace) => {
                let name = def.name();
                functions.entry(name).or_default().combine(&functions::writer(writer, namespace, def));
            }
            metadata::Item::Const(def) => {
                let name = def.name();
                types.entry(metadata::TypeKind::Class).or_default().entry(name).or_default().combine(&constants::writer(writer, def));
            }
        }
    }

    for function in functions.values() {
        tokens.combine(function);
    }

    for ty in types.values().flat_map(|v| v.values()) {
        tokens.combine(ty);
    }

    tokens.combine(&extensions::gen_mod(writer, tree.namespace));

    if writer.implement {
        tokens.push_str(&namespace_impl(writer, tree));
    }

    tokens.into_string()
}

fn namespace_impl(writer: &Writer, tree: &Tree) -> String {
    let writer = &mut writer.clone();
    writer.namespace = tree.namespace;
    let mut types = std::collections::BTreeMap::<&str, TokenStream>::new();

    for item in writer.reader.namespace_items(tree.namespace) {
        if let metadata::Item::Type(def) = item {
            let type_name = def.type_name();
            if metadata::CORE_TYPES.iter().any(|(x, _)| x == &type_name) {
                continue;
            }
            if def.kind() != metadata::TypeKind::Interface {
                continue;
            }
            let tokens = implements::writer(writer, def);

            if !tokens.is_empty() {
                types.insert(type_name.name, tokens);
            }
        }
    }

    let types = types.values();

    let mut tokens = quote! {
        #(#types)*
    };

    tokens.combine(&extensions::gen_impl(tree.namespace));
    tokens.into_string()
}

/// Expand a possibly empty generics list with a new generic
fn expand_generics(generics: TokenStream, new: TokenStream) -> TokenStream {
    if generics.is_empty() {
        quote!(#new)
    } else {
        quote!(#generics, #new)
    }
}

/// Expand a possibly emppty where clause with a new generic constraint
fn expand_where_clause(where_clause: TokenStream, generic: TokenStream) -> TokenStream {
    if where_clause.is_empty() {
        quote!(where #generic)
    } else {
        quote!(#where_clause #generic)
    }
}

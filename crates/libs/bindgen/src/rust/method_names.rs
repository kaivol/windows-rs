use super::*;
use metadata::HasAttributes;

pub struct MethodNames(BTreeMap<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn add(&mut self, method: metadata::MethodDef) -> TokenStream {
        let name = method_def_special_name(method);
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{name}{overload}").into()
        } else {
            to_ident(&name)
        }
    }

    pub fn add_vtable_types(&mut self, def: metadata::TypeDef) {
        for def in metadata::type_def_vtables(def) {
            if let metadata::Type::TypeDef(def, _) = def {
                for method in def.methods() {
                    self.add(method);
                }
            }
        }
    }
}

fn method_def_special_name(row: metadata::MethodDef) -> String {
    let name = row.name();
    if row.flags().contains(metadata::MethodAttributes::SpecialName) {
        return if let Some(getter) = name.strip_prefix("get_") {
            getter.to_owned()
        } else if let Some(setter) = name.strip_prefix("put_") {
            format!("Set{}", setter)
        } else if let Some(add) = name.strip_prefix("add_") {
            add.to_owned()
        } else if let Some(remove) = name.strip_prefix("remove_") {
            format!("Remove{}", remove)
        } else {
            name.to_string()
        };
    } else if let Some(attribute) = row.find_attribute("OverloadAttribute") {
        for (_, arg) in attribute.args() {
            if let metadata::Value::String(overload) = arg {
                if let Some(suffix) = overload.strip_prefix(name) {
                    if suffix.chars().all(|c| c.is_numeric()) {
                        return name.to_owned();
                    }
                }
                return overload;
            }
        }
    }
    name.to_string()
}

use super::*;
use std::collections::btree_map::Entry;

pub struct MethodNames(BTreeMap<String, Kind>);

enum Kind {
    Base { next_overload: u32 },
    Overload { base: String },
}

impl MethodNames {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, gen: &Gen, method: MethodDef) -> TokenStream {
        let name = gen.reader.method_def_special_name(method);
        self.add_inner(name)
    }

    fn add_inner(&mut self, name: String) -> TokenStream {
        match self.0.get_mut(&name) {
            Some(Kind::Base { next_overload }) => {
                let overload_name = format!("{name}{}", *next_overload);
                *next_overload += 1;
                self.insert_and_to_indent(overload_name, Kind::Overload { base: name })
            }
            Some(Kind::Overload { base }) => {
                let base = base.clone();
                if let Some(Kind::Base { next_overload }) = self.0.get_mut(&base) {
                    let overload_name = format!("{base}{}", *next_overload);
                    *next_overload += 1;
                    self.insert_and_to_indent(overload_name, Kind::Overload { base })
                } else {
                    unreachable!()
                }
            }
            None => self.insert_and_to_indent(name, Kind::Base { next_overload: 2 }),
        }
    }

    fn insert_and_to_indent(&mut self, name: String, kind: Kind) -> TokenStream {
        let result = to_ident(&name);
        match self.0.entry(name) {
            Entry::Vacant(v) => {
                v.insert(kind);
                result
            }
            Entry::Occupied(o) => panic!("Prevented generation of duplicate method name {}", o.key()),
        }
    }

    pub fn add_vtable_types(&mut self, gen: &Gen, def: TypeDef) {
        for def in gen.reader.type_def_vtables(def) {
            if let Type::TypeDef((def, _)) = def {
                for method in gen.reader.type_def_methods(def) {
                    self.add(gen, method);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overload() {
        let mut method_names = MethodNames::new();
        assert_eq!(method_names.add_inner("SetFoo".into()), "SetFoo".into());
        assert_eq!(method_names.add_inner("SetFoo".into()), "SetFoo2".into());
        assert_eq!(method_names.add_inner("SetFoo2".into()), "SetFoo3".into());
        assert_eq!(method_names.add_inner("SetFoo".into()), "SetFoo4".into());
    }
}

use super::*;

pub struct MethodNames(BTreeMap<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, gen: &Gen, method: MethodDef) -> TokenStream {
        let name = gen.reader.method_def_special_name(method);
        self.add_inner(name)
    }

    fn add_inner(&mut self, name: String) -> TokenStream {
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            let name = format!("{name}{overload}");
            self.add_inner(name)
        } else {
            to_ident(&name)
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

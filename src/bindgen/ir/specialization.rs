/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use syn;

use bindgen::dependencies::Dependencies;
use bindgen::ir::{AnnotationSet, Cfg, Documentation};
use bindgen::ir::{GenericPath, ItemContainer, Item, PrimitiveType};
use bindgen::library::Library;

/// A type alias that generates a copy of its aliasee with a new name. If the type
/// alias has generic values, it specializes its aliasee. This is useful for
/// presenting an interface that includes generic types without mangling.
#[derive(Debug, Clone)]
pub struct Specialization {
    pub name: String,
    pub generic_params: Vec<String>,
    pub aliased: GenericPath,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl Specialization {
    pub fn load(name: String,
                generics: &syn::Generics,
                ty: &syn::Ty,
                attrs: &Vec<syn::Attribute>,
                mod_cfg: &Option<Cfg>) -> Result<Specialization, String>
    {
        match ty {
            &syn::Ty::Path(ref _q, ref p) => {
                let generic_params = generics.ty_params.iter()
                                                       .map(|x| x.ident.to_string())
                                                       .collect::<Vec<_>>();

                let path = GenericPath::load(p)?;

                if PrimitiveType::maybe(&path.name).is_some() {
                    return Err(format!("can't specialize a primitive"));
                }

                Ok(Specialization {
                    name: name,
                    generic_params: generic_params,
                    aliased: path,
                    cfg: Cfg::append(mod_cfg, Cfg::load(attrs)),
                    annotations: AnnotationSet::load(attrs)?,
                    documentation: Documentation::load(attrs),
                })
            }
            _ => {
                Err(format!("not a path"))
            }
        }
    }

    pub fn resolve_specialization(&self, library: &Library) -> Result<Box<Item>, String> {
        if let Some(items) = library.get_items(&self.aliased.name) {
            assert!(items.len() > 0);

            if items.len() > 1 {
                warn!("specializing an aliased type with multiple definitions");
            }

            match items[0] {
                ItemContainer::OpaqueItem(ref aliased) => {
                    aliased.specialize(library, self)
                }
                ItemContainer::Struct(ref aliased) => {
                    aliased.specialize(library, self)
                }
                ItemContainer::Enum(ref aliased) => {
                    aliased.specialize(library, self)
                }
                ItemContainer::Typedef(ref aliased) => {
                    aliased.specialize(library, self)
                }
                ItemContainer::Specialization(ref aliased) => {
                    aliased.specialize(library, self)
                }
            }
        } else {
            Err(format!("couldn't find aliased type"))
        }
    }
}

impl Item for Specialization {
    fn name(&self) -> &str {
        &self.name
    }

    fn cfg(&self) -> &Option<Cfg> {
        &self.cfg
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::Specialization(self.clone())
    }

    fn specialize(&self, library: &Library, aliasee: &Specialization) -> Result<Box<Item>, String> {
        if aliasee.aliased.generics.len() !=
           self.generic_params.len() {
            return Err(format!("incomplete specialization"));
        }

        let mappings = self.generic_params.iter()
                                          .zip(aliasee.aliased.generics.iter())
                                          .collect::<Vec<_>>();

        let generics = self.aliased.generics.iter()
                                            .map(|x| x.specialize(&mappings))
                                            .collect();

        Specialization {
            name: aliasee.name.clone(),
            generic_params: aliasee.generic_params.clone(),
            aliased: GenericPath::new(self.aliased.name.clone(), generics),
            cfg: aliasee.cfg.clone(),
            annotations: aliasee.annotations.clone(),
            documentation: aliasee.documentation.clone(),
        }.resolve_specialization(library)
    }

    fn add_dependencies(&self, _: &Library, _: &mut Dependencies) {
        unreachable!("Specialization's must be specialized before dependency gathering.");
    }
}

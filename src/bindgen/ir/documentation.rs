/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::io::Write;

use syn;

use bindgen::config::{Config, Language};
use bindgen::writer::{Source, SourceWriter};

#[derive(Debug, Clone)]
pub struct Documentation {
    pub doc_comment: Vec<String>,
}

impl Documentation {
    pub fn load(attrs: &[syn::Attribute]) -> Self {
        let mut doc = Vec::new();

        for attr in attrs {
            if attr.style == syn::AttrStyle::Outer {
                // syn expands documentation comments into #[doc ...] attributes
                if let Some(syn::Meta::NameValue(syn::MetaNameValue {
                    ident,
                    lit: syn::Lit::Str(comment),
                    ..
                })) = attr.interpret_meta()
                {
                    let name = ident.to_string();
                    let comment = comment.value();

                    if &*name == "doc" {
                        let line = comment.trim_left_matches(" ").trim_right();
                        if !line.starts_with("cbindgen:") {
                            doc.push(line.to_owned());
                        }
                    }
                }
            }
        }

        Documentation { doc_comment: doc }
    }

    pub fn none() -> Self {
        Documentation {
            doc_comment: Vec::new(),
        }
    }
}

impl Source for Documentation {
    fn write<F: Write>(&self, config: &Config, out: &mut SourceWriter<F>) {
        if self.doc_comment.is_empty() || !config.documentation {
            return;
        }

        if config.language == Language::C {
            out.write("/*");
            out.new_line();
        }
        for line in &self.doc_comment {
            if config.language != Language::C {
                out.write("//");
            } else {
                out.write(" *");
            }
            if line.len() != 0 {
                out.write(" ");
            }
            write!(out, "{}", line);
            out.new_line();
        }
        if config.language == Language::C {
            out.write(" */");
            out.new_line();
        }
    }
}

pub(crate) fn attr_is_doc(attr: &syn::Attribute) -> bool {
    let path = &attr.path;
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments[0].arguments.is_empty()
        && path.segments[0].ident == "doc"
}

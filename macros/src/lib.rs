#![feature(plugin_registrar, quote, box_syntax, rustc_private)]
#![allow(unused_imports)]

extern crate syntax;
extern crate syntax_ext;
extern crate rustc;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ext::base::{MultiDecorator, MultiModifier};
use syntax::parse::token::intern;
use rustc::hir::map::blocks::MaybeFnLike;

use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::codemap::Span;
use syntax::ptr::P;

use syntax::ast::{Item, MetaItem};
use syntax::attr;
use syntax_ext::deriving::generic::{combine_substructure, EnumMatching, FieldInfo, MethodDef, Struct, Substructure, TraitDef, ty};
use syntax::parse::token::InternedString;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("pg_export"), MultiModifier(box expand_pg_export));
}

pub fn expand_pg_export(cx: &mut ExtCtxt, span: Span, _: &MetaItem, item: Annotatable) -> Annotatable {

    //TODO: enforce func type check
    // if !func.is_fn_like() {
    //     cx.span_err(span, "you can only export a function to PostgreSQL.");
    // }

    match item {
        Annotatable::Item(it) => {
            let mut new_it = (*it).clone();
            new_it.attrs.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_word_item(InternedString::new("no_mangle"))));
            Annotatable::Item(P(new_it))
        }
        Annotatable::ImplItem(it) => {
            let mut new_it = (*it).clone();
            new_it.attrs.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_word_item(InternedString::new("no_mangle"))));
            Annotatable::ImplItem(P(new_it))
        }
        Annotatable::TraitItem(tt) => {
            let mut new_it = (*tt).clone();
            new_it.attrs.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_word_item(InternedString::new("no_mangle"))));
            Annotatable::TraitItem(P(new_it))
        }
    }

}

/// Postgres has a macro called `PG_MODULE_MAGIC` that is supposed
/// to be called within extensions. This generates a bunch
/// of metadata structures that Postgres reads to determine
/// the compatibility of the extension.
///
/// `Pg_magic_func` is the function Postgres will call
/// to check compatibility with memcmp, so there can't be
/// any alignment differences.
///
/// Usage:
///
/// ```notrust
/// pg_module!(90500)
/// ```
#[macro_export]
macro_rules! pg_module {
    (version: $vers:expr) => {
        static mut Pg_magic_data: postgres_extension::Pg_magic_struct =
            postgres_extension::Pg_magic_struct {
                len: 0 as c_int,
                version: $vers,
                funcmaxargs: 100,
                indexmaxkeys: 32,
                nameddatalen: 64,
                float4byval: 1,
                float8byval: 1
            };


        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern fn Pg_magic_func() -> &'static postgres_extension::Pg_magic_struct {
            use std::mem::size_of;
            use libc::{c_int};

            unsafe {
                Pg_magic_data = postgres_extension::Pg_magic_struct {
                    len: size_of::<postgres_extension::Pg_magic_struct>() as c_int,
                    version: $vers / 100,
                    funcmaxargs: 100,
                    indexmaxkeys: 32,
                    nameddatalen: 64,
                    float4byval: 1,
                    float8byval: 1
                };

                &Pg_magic_data
            }
        }
    }
}

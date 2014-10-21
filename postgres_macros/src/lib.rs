#![feature(macro_rules, plugin_registrar, quote, phase)]
#![allow(unused_imports)]

#[phase(plugin,link)]
extern crate syntax;
#[phase(plugin, link)]
extern crate rustc;

use rustc::lint::LintPassObject;
use rustc::plugin::Registry;
use syntax::ext::base::{Decorator, Modifier};
use syntax::parse::token::intern;
use syntax::ast_map::blocks::MaybeFnLike;
use syntax::abi;

use syntax::ext::base::ExtCtxt;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::ast::{Item, MetaItem, Expr};
use syntax::ast;
use syntax::attr;
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::{combine_substructure, EnumMatching, FieldInfo, MethodDef, Struct, Substructure, TraitDef, ty};
use syntax::parse::token::InternedString;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("pg_export"), Modifier(box expand_pg_export));
}

pub fn expand_pg_export(cx: &mut ExtCtxt, span: Span, _: &MetaItem, item: P<Item>) -> P<Item> {
    let mut func = (*item).clone();

    if !func.is_fn_like() {
        cx.span_err(span, "you can only export a function to PostgreSQL.");
    }

    func.attrs.push(attr::mk_attr_outer(attr::mk_attr_id(), attr::mk_word_item(InternedString::new("no_mangle"))));

    match (*item).node {
        ast::ItemFn(_, _, mut _abi, _, _) => {
            _abi = abi::C;
        },
        _ => {}
    }

    P(func)
}

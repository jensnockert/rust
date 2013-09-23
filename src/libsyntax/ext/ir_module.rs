// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
 * Inline IR support.
 */

use abi;
use ast;
use ast::token_tree;
use codemap::Span;
use ext::base;
use ext::base::*;
use opt_vec::Empty;
use parse;
use parse::token;
use parse::token::keywords;
use parse::token::special_idents;

pub fn expand_ir_module(cx: @ExtCtxt, sp: Span, tts: &[token_tree]) -> MacResult {
    let p = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_owned());

    p.expect_keyword(keywords::Extern);

    let abis = p.parse_opt_abis().unwrap_or(abi::AbiSet::C());

    p.expect(&token::LBRACE);

    let mut items = ~[];

    let is_raw_ir = match *p.token {
        token::IDENT(sid, false) => { token::intern(&"ir") == sid.name },
        _ => false
    };

    if is_raw_ir {
        p.bump();

        let ir = p.parse_expr();
        let src = expr_to_str(cx, ir, "inline IR must be a string literal.");

        items.push(@ast::foreign_item {
            ident: special_idents::anon,
            attrs: ~[],
            node: ast::foreign_item_raw_ir(src),
            id: ast::DUMMY_NODE_ID,
            span: ir.span,
            vis: ast::inherited,
        });

        p.expect(&token::SEMI);
    }

    while p.eat_keyword(keywords::Fn) {
        let fn_id = p.parse_ident();
        let fn_decl = p.parse_fn_decl();
        
        p.expect(&token::LBRACE);

        let ir = p.parse_expr();

        let src = expr_to_str(cx, ir, "inline IR must be a string literal.");

        p.expect(&token::RBRACE);
        
        items.push(@ast::foreign_item {
            ident: fn_id,
            attrs: ~[],
            node: ast::foreign_item_ir_fn(fn_decl, src),
            id: ast::DUMMY_NODE_ID,
            span: ir.span,
            vis: ast::inherited,
        });
    }

    p.expect(&token::RBRACE);

    let module = ast::foreign_mod {
        sort: ast::anonymous,
        abis: abis,
        view_items: ~[],
        items: items
    };

    MRItem(@ast::item {
        ident: special_idents::clownshoes_foreign_mod,
        attrs: ~[],
        id: ast::DUMMY_NODE_ID,
        node: ast::item_foreign_mod(module),
        vis: ast::private,
        span: sp
    })
}

// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*
 * Syntax support for SIMD vectors.
 *
 * To create a SIMD vector type, use `type u8x16 = simd!(u8, 16);` or similar.
 */

use ast;
use opt_vec;

use codemap::span;
use ext::base;
use parse;
use parse::token;

use std::vec;

pub fn expand_simd(cx: @base::ExtCtxt, sp: span, tts: &[ast::token_tree]) -> base::MacResult {
    let parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), vec::to_owned(tts));

    let ident = parser.parse_ident();
    parser.expect(&token::COLON);
    let ty = parser.parse_ty(false); //Parameter is useless, apparently
    parser.expect(&token::BINOP(token::STAR));
    let expr = parser.parse_expr();

    let vector = ast::Ty {
        id: cx.next_id(),
        node: ast::ty_simd_vec(~ty, expr),
        span: sp
    };

    base::MRItem(@ast::item {
        ident: ident,
        attrs: ~[],
        id: cx.next_id(),
        node: ast::item_ty(vector, ast::Generics { lifetimes: opt_vec::Empty, ty_params: opt_vec::Empty }),
        span: sp,
        vis: ast::inherited
    })
}
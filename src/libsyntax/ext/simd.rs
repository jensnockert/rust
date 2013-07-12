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

use codemap::span;
use ext::base;
use ext::build::AstBuilder;
use parse;

pub fn expand_simd(cx: @base::ExtCtxt, sp: span, tts: &[ast::token_tree]) -> base::MacResult {
    let parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_owned());

    let ident = parser.parse_ident(); parser.expect(&parse::token::COLON);
    let ty = parser.parse_ty(false); parser.expect(&parse::token::BINOP(parse::token::STAR));
    let lit = parser.parse_lit();

    match lit.node {
        ast::lit_int_unsuffixed(n) => {
            base::MRItem(cx.item_ty(sp, ident, cx.ty(sp, ast::ty_simd_vec(~ty, n as uint))))
        }
        _ => cx.span_bug(lit.span, "SIMD vector length needs to be an unprefixed integer literal")
    }
}
// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*! rustc compiler intrinsics.

The corresponding definitions are in librustc/middle/trans/foreign.rs.

# Atomics

The atomic intrinsics provide common atomic operations on machine
words, with multiple possible memory orderings. They obey the same
semantics as C++11. See the LLVM documentation on [[atomics]].

[atomics]: http://llvm.org/docs/Atomics.html

A quick refresher on memory ordering:

* Acquire - a barrier for acquiring a lock. Subsequent reads and writes
  take place after the barrier.
* Release - a barrier for releasing a lock. Preceding reads and writes
  take place before the barrier.
* Sequentially consistent - sequentially consistent operations are
  guaranteed to happen in order. This is the standard mode for working
  with atomic types and is equivalent to Java's `volatile`.

*/

// This is needed to prevent duplicate lang item definitions.
#[cfg(test)]
pub use realstd::unstable::intrinsics::{TyDesc, Opaque, TyVisitor};

use ptr;

pub type GlueFn = extern "Rust" fn(*i8);

// NB: this has to be kept in sync with `type_desc` in `rt`
#[lang="ty_desc"]
#[cfg(not(test))]
pub struct TyDesc {
    // sizeof(T)
    size: uint,

    // alignof(T)
    align: uint,

    // Called on a copy of a value of type `T` *after* memcpy
    take_glue: GlueFn,

    // Called when a value of type `T` is no longer needed
    drop_glue: GlueFn,

    // Called by drop glue when a value of type `T` can be freed
    free_glue: GlueFn,

    // Called by reflection visitor to visit a value of type `T`
    visit_glue: GlueFn,

    // If T represents a box pointer (`@U` or `~U`), then
    // `borrow_offset` is the amount that the pointer must be adjusted
    // to find the payload.  This is always derivable from the type
    // `U`, but in the case of `@Trait` or `~Trait` objects, the type
    // `U` is unknown.
    borrow_offset: uint,

    // Name corresponding to the type
    name: &'static str
}

#[lang="opaque"]
#[cfg(not(test))]
pub enum Opaque { }

#[lang="ty_visitor"]
#[cfg(not(test))]
pub trait TyVisitor {
    fn visit_bot(&mut self) -> bool;
    fn visit_nil(&mut self) -> bool;
    fn visit_bool(&mut self) -> bool;

    fn visit_int(&mut self) -> bool;
    fn visit_i8(&mut self) -> bool;
    fn visit_i16(&mut self) -> bool;
    fn visit_i32(&mut self) -> bool;
    fn visit_i64(&mut self) -> bool;

    fn visit_uint(&mut self) -> bool;
    fn visit_u8(&mut self) -> bool;
    fn visit_u16(&mut self) -> bool;
    fn visit_u32(&mut self) -> bool;
    fn visit_u64(&mut self) -> bool;

    fn visit_float(&mut self) -> bool;
    fn visit_f32(&mut self) -> bool;
    fn visit_f64(&mut self) -> bool;

    fn visit_char(&mut self) -> bool;

    fn visit_estr_box(&mut self) -> bool;
    fn visit_estr_uniq(&mut self) -> bool;
    fn visit_estr_slice(&mut self) -> bool;
    fn visit_estr_fixed(&mut self, n: uint, sz: uint, align: uint) -> bool;

    fn visit_box(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq_managed(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_ptr(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_rptr(&mut self, mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_vec(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_unboxed_vec(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_box(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq_managed(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_slice(&mut self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_fixed(&mut self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_enter_rec(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_rec_field(&mut self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_rec(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_class(&mut self, name: &str, named_fields: bool, n_fields: uint,
                         sz: uint, align: uint) -> bool;
    fn visit_class_field(&mut self, i: uint, name: &str, named: bool,
                         mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_class(&mut self, name: &str, named_fields: bool, n_fields: uint,
                         sz: uint, align: uint) -> bool;

    fn visit_enter_tup(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_tup_field(&mut self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_tup(&mut self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_enum(&mut self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;
    fn visit_enter_enum_variant(&mut self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_enum_variant_field(&mut self, i: uint, offset: uint, inner: *TyDesc) -> bool;
    fn visit_leave_enum_variant(&mut self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_leave_enum(&mut self, n_variants: uint,
                        get_disr: extern unsafe fn(ptr: *Opaque) -> int,
                        sz: uint, align: uint) -> bool;

    fn visit_enter_fn(&mut self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;
    fn visit_fn_input(&mut self, i: uint, mode: uint, inner: *TyDesc) -> bool;
    fn visit_fn_output(&mut self, retstyle: uint, inner: *TyDesc) -> bool;
    fn visit_leave_fn(&mut self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;

    fn visit_trait(&mut self, name: &str) -> bool;
    fn visit_param(&mut self, i: uint) -> bool;
    fn visit_self(&mut self) -> bool;
    fn visit_type(&mut self) -> bool;
    fn visit_opaque_box(&mut self) -> bool;
    fn visit_closure_ptr(&mut self, ck: uint) -> bool;
}

#[abi = "rust-intrinsic"]
extern "rust-intrinsic" {

    /// Atomic compare and exchange, sequentially consistent.
    pub fn atomic_cxchg(dst: &mut int, old: int, src: int) -> int;
    /// Atomic compare and exchange, acquire ordering.
    pub fn atomic_cxchg_acq(dst: &mut int, old: int, src: int) -> int;
    /// Atomic compare and exchange, release ordering.
    pub fn atomic_cxchg_rel(dst: &mut int, old: int, src: int) -> int;

    pub fn atomic_cxchg_acqrel(dst: &mut int, old: int, src: int) -> int;
    pub fn atomic_cxchg_relaxed(dst: &mut int, old: int, src: int) -> int;


    /// Atomic load, sequentially consistent.
    pub fn atomic_load(src: &int) -> int;
    /// Atomic load, acquire ordering.
    pub fn atomic_load_acq(src: &int) -> int;

    pub fn atomic_load_relaxed(src: &int) -> int;

    /// Atomic store, sequentially consistent.
    pub fn atomic_store(dst: &mut int, val: int);
    /// Atomic store, release ordering.
    pub fn atomic_store_rel(dst: &mut int, val: int);

    pub fn atomic_store_relaxed(dst: &mut int, val: int);

    /// Atomic exchange, sequentially consistent.
    pub fn atomic_xchg(dst: &mut int, src: int) -> int;
    /// Atomic exchange, acquire ordering.
    pub fn atomic_xchg_acq(dst: &mut int, src: int) -> int;
    /// Atomic exchange, release ordering.
    pub fn atomic_xchg_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xchg_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xchg_relaxed(dst: &mut int, src: int) -> int;

    /// Atomic addition, sequentially consistent.
    pub fn atomic_xadd(dst: &mut int, src: int) -> int;
    /// Atomic addition, acquire ordering.
    pub fn atomic_xadd_acq(dst: &mut int, src: int) -> int;
    /// Atomic addition, release ordering.
    pub fn atomic_xadd_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xadd_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xadd_relaxed(dst: &mut int, src: int) -> int;

    /// Atomic subtraction, sequentially consistent.
    pub fn atomic_xsub(dst: &mut int, src: int) -> int;
    /// Atomic subtraction, acquire ordering.
    pub fn atomic_xsub_acq(dst: &mut int, src: int) -> int;
    /// Atomic subtraction, release ordering.
    pub fn atomic_xsub_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xsub_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xsub_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_and(dst: &mut int, src: int) -> int;
    pub fn atomic_and_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_and_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_and_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_and_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_nand(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_nand_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_or(dst: &mut int, src: int) -> int;
    pub fn atomic_or_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_or_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_or_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_or_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_xor(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_xor_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_max(dst: &mut int, src: int) -> int;
    pub fn atomic_max_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_max_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_max_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_max_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_min(dst: &mut int, src: int) -> int;
    pub fn atomic_min_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_min_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_min_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_min_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_umin(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_umin_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_umax(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_acq(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_rel(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_acqrel(dst: &mut int, src: int) -> int;
    pub fn atomic_umax_relaxed(dst: &mut int, src: int) -> int;

    pub fn atomic_fence();
    pub fn atomic_fence_acq();
    pub fn atomic_fence_rel();
    pub fn atomic_fence_acqrel();

    /// The size of a type in bytes.
    ///
    /// This is the exact number of bytes in memory taken up by a
    /// value of the given type. In other words, a memset of this size
    /// would *exactly* overwrite a value. When laid out in vectors
    /// and structures there may be additional padding between
    /// elements.
    pub fn size_of<T>() -> uint;

    /// Move a value to a memory location containing a value.
    ///
    /// Drop glue is run on the destination, which must contain a
    /// valid Rust value.
    pub fn move_val<T>(dst: &mut T, src: T);

    /// Move a value to an uninitialized memory location.
    ///
    /// Drop glue is not run on the destination.
    pub fn move_val_init<T>(dst: &mut T, src: T);

    pub fn min_align_of<T>() -> uint;
    pub fn pref_align_of<T>() -> uint;

    /// Get a static pointer to a type descriptor.
    pub fn get_tydesc<T>() -> *TyDesc;

    /// Create a value initialized to zero.
    ///
    /// `init` is unsafe because it returns a zeroed-out datum,
    /// which is unsafe unless T is POD. We don't have a POD
    /// kind yet. (See #4074).
    pub fn init<T>() -> T;

    /// Create an uninitialized value.
    pub fn uninit<T>() -> T;

    /// Move a value out of scope without running drop glue.
    ///
    /// `forget` is unsafe because the caller is responsible for
    /// ensuring the argument is deallocated already.
    pub fn forget<T>(_: T) -> ();
    pub fn transmute<T,U>(e: T) -> U;

    /// Returns `true` if a type requires drop glue.
    pub fn needs_drop<T>() -> bool;

    /// Returns `true` if a type is managed (will be allocated on the local heap)
    pub fn contains_managed<T>() -> bool;

    pub fn visit_tydesc(td: *TyDesc, tv: &mut TyVisitor);

    pub fn frame_address(f: &once fn(*u8));

    /// Get the address of the `__morestack` stack growth function.
    pub fn morestack_addr() -> *();

    /// Calculates the offset from a pointer. The offset *must* be in-bounds of
    /// the object, or one-byte-past-the-end. An arithmetic overflow is also
    /// undefined behaviour.
    ///
    /// This is implemented as an intrinsic to avoid converting to and from an
    /// integer, since the conversion would throw away aliasing information.
    pub fn offset<T>(dst: *T, offset: int) -> *T;

    /// Equivalent to the `llvm.memcpy.p0i8.0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
    /// Equivalent to the `llvm.memcpy.p0i8.0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);

    /// Equivalent to the `llvm.memmove.p0i8.0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memmove32<T>(dst: *mut T, src: *T, count: u32);
    /// Equivalent to the `llvm.memmove.p0i8.0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memmove64<T>(dst: *mut T, src: *T, count: u64);

    /// Equivalent to the `llvm.memset.p0i8.i32` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memset32<T>(dst: *mut T, val: u8, count: u32);
    /// Equivalent to the `llvm.memset.p0i8.i64` intrinsic, with a size of
    /// `count` * `size_of::<T>()` and an alignment of `min_align_of::<T>()`
    pub fn memset64<T>(dst: *mut T, val: u8, count: u64);
}

#[cfg(stage1)]
#[cfg(stage2)]
#[cfg(stage3)]
mod workaround {
    llvm!(extern {
        ir "
        declare i8 @llvm.ctpop.i8(i8)
        declare i16 @llvm.ctpop.i16(i16)
        declare i32 @llvm.ctpop.i32(i32)
        declare i64 @llvm.ctpop.i64(i64)

        declare i8 @llvm.ctlz.i8(i8, i1)
        declare i16 @llvm.ctlz.i16(i16, i1)
        declare i32 @llvm.ctlz.i32(i32, i1)
        declare i64 @llvm.ctlz.i64(i64, i1)

        declare i8 @llvm.cttz.i8(i8, i1)
        declare i16 @llvm.cttz.i16(i16, i1)
        declare i32 @llvm.cttz.i32(i32, i1)
        declare i64 @llvm.cttz.i64(i64, i1)

        declare i16 @llvm.bswap.i16(i16)
        declare i32 @llvm.bswap.i32(i32)
        declare i64 @llvm.bswap.i64(i64)

        declare float @llvm.sqrt.f32(float)
        declare double @llvm.sqrt.f64(double)

        declare float @llvm.powi.f32(float, i32)
        declare double @llvm.powi.f64(double, i32)

        declare float @llvm.sin.f32(float)
        declare double @llvm.sin.f64(double)

        declare float @llvm.cos.f32(float)
        declare double @llvm.cos.f64(double)

        declare float @llvm.pow.f32(float, float)
        declare double @llvm.pow.f64(double, double)

        declare float @llvm.exp.f32(float)
        declare double @llvm.exp.f64(double)

        declare float @llvm.exp2.f32(float)
        declare double @llvm.exp2.f64(double)

        declare float @llvm.log.f32(float)
        declare double @llvm.log.f64(double)

        declare float @llvm.log10.f32(float)
        declare double @llvm.log10.f64(double)

        declare float @llvm.log2.f32(float)
        declare double @llvm.log2.f64(double)

        declare float @llvm.fma.f32(float, float, float)
        declare double @llvm.fma.f64(double, double, double)

        declare float @llvm.fabs.f32(float)
        declare double @llvm.fabs.f64(double)

        declare float @llvm.floor.f32(float)
        declare double @llvm.floor.f64(double)

        declare float @llvm.ceil.f32(float)
        declare double @llvm.ceil.f64(double)

        declare float @llvm.trunc.f32(float)
        declare double @llvm.trunc.f64(double)

        declare { i8,  i1 } @llvm.sadd.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.sadd.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.sadd.with.overflow.i64(i64, i64)

        declare { i8,  i1 } @llvm.uadd.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.uadd.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.uadd.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64)

        declare { i8,  i1 } @llvm.ssub.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.ssub.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.ssub.with.overflow.i64(i64, i64)

        declare { i8,  i1 } @llvm.usub.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.usub.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.usub.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.usub.with.overflow.i64(i64, i64)

        declare { i8,  i1 } @llvm.smul.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.smul.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.smul.with.overflow.i64(i64, i64)

        declare { i8,  i1 } @llvm.umul.with.overflow.i8(i8, i8)
        declare { i16, i1 } @llvm.umul.with.overflow.i16(i16, i16)
        declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32)
        declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64)
        ";
    })
}

llvm!(extern {
    fn llvm_ctpop8(x: i8) -> i8 {
        "%r = call i8 @llvm.ctpop.i8(i8 %arg0)
         ret i8 %r"
    }

    fn llvm_ctpop16(x: i16) -> i16 {
        "%r = call i16 @llvm.ctpop.i16(i16 %arg0)
         ret i16 %r"
    }

    fn llvm_ctpop32(x: i32) -> i32 {
        "%r = call i32 @llvm.ctpop.i32(i32 %arg0)
         ret i32 %r"
    }

    fn llvm_ctpop64(x: i64) -> i64 {
        "%r = call i64 @llvm.ctpop.i64(i64 %arg0)
         ret i64 %r"
    }

    fn llvm_ctlz8(x: i8) -> i8 {
        "%r = call i8 @llvm.ctlz.i8(i8 %arg0, i1 0)
         ret i8 %r"
    }

    fn llvm_ctlz16(x: i16) -> i16 {
        "%r = call i16 @llvm.ctlz.i16(i16 %arg0, i1 0)
         ret i16 %r"
    }

    fn llvm_ctlz32(x: i32) -> i32 {
        "%r = call i32 @llvm.ctlz.i32(i32 %arg0, i1 0)
         ret i32 %r"
    }

    fn llvm_ctlz64(x: i64) -> i64 {
        "%r = call i64 @llvm.ctlz.i64(i64 %arg0, i1 0)
         ret i64 %r"
    }

    fn llvm_cttz8(x: i8) -> i8 {
        "%r = call i8 @llvm.cttz.i8(i8 %arg0, i1 0)
         ret i8 %r"
    }

    fn llvm_cttz16(x: i16) -> i16 {
        "%r = call i16 @llvm.cttz.i16(i16 %arg0, i1 0)
         ret i16 %r"
    }

    fn llvm_cttz32(x: i32) -> i32 {
        "%r = call i32 @llvm.cttz.i32(i32 %arg0, i1 0)
         ret i32 %r"
    }

    fn llvm_cttz64(x: i64) -> i64 {
        "%r = call i64 @llvm.cttz.i64(i64 %arg0, i1 0)
         ret i64 %r"
    }

    fn llvm_bswap16(x: i16) -> i16 {
        "%r = call i16 @llvm.bswap.i16(i16 %arg0)
         ret i16 %r"
    }

    fn llvm_bswap32(x: i32) -> i32 {
        "%r = call i32 @llvm.bswap.i32(i32 %arg0)
         ret i32 %r"
    }

    fn llvm_bswap64(x: i64) -> i64 {
        "%r = call i64 @llvm.bswap.i64(i64 %arg0)
         ret i64 %r"
    }

    fn llvm_sqrtf32(x: f32) -> f32 {
        "%r = call float @llvm.sqrt.f32(float %arg0)
         ret float %r"
    }

    fn llvm_sqrtf64(x: f64) -> f64 {
        "%r = call double @llvm.sqrt.f64(double %arg0)
         ret double %r"
    }

    fn llvm_powif32(a: f32, x: i32) -> f32 {
        "%r = call float @llvm.powi.f32(float %arg0, i32 %arg1)
         ret float %r"
    }

    fn llvm_powif64(a: f64, x: i32) -> f64 {
        "%r = call double @llvm.powi.f64(double %arg0, i32 %arg1)
         ret double %r"
    }

    fn llvm_sinf32(x: f32) -> f32 {
        "%r = call float @llvm.sin.f32(float %arg0)
         ret float %r"
    }

    fn llvm_sinf64(x: f64) -> f64 {
        "%r = call double @llvm.sin.f64(double %arg0)
         ret double %r"
    }

    fn llvm_cosf32(x: f32) -> f32 {
        "%r = call float @llvm.cos.f32(float %arg0)
         ret float %r"
    }

    fn llvm_cosf64(x: f64) -> f64 {
        "%r = call double @llvm.cos.f64(double %arg0)
         ret double %r"
    }

    fn llvm_powf32(a: f32, x: f32) -> f32 {
        "%r = call float @llvm.pow.f32(float %arg0, float %arg1)
         ret float %r"
    }

    fn llvm_powf64(a: f64, x: f64) -> f64 {
        "%r = call double @llvm.pow.f64(double %arg0, double %arg1)
         ret double %r"
    }

    fn llvm_expf32(x: f32) -> f32 {
        "%r = call float @llvm.exp.f32(float %arg0)
         ret float %r"
    }

    fn llvm_expf64(x: f64) -> f64 {
        "%r = call double @llvm.exp.f64(double %arg0)
         ret double %r"
    }

    fn llvm_exp2f32(x: f32) -> f32 {
        "%r = call float @llvm.exp2.f32(float %arg0)
         ret float %r"
    }

    fn llvm_exp2f64(x: f64) -> f64 {
        "%r = call double @llvm.exp2.f64(double %arg0)
         ret double %r"
    }

    fn llvm_logf32(x: f32) -> f32 {
        "%r = call float @llvm.log.f32(float %arg0)
         ret float %r"
    }

    fn llvm_logf64(x: f64) -> f64 {
        "%r = call double @llvm.log.f64(double %arg0)
         ret double %r"
    }

    fn llvm_log10f32(x: f32) -> f32 {
        "%r = call float @llvm.log10.f32(float %arg0)
         ret float %r"
    }

    fn llvm_log10f64(x: f64) -> f64 {
        "%r = call double @llvm.log10.f64(double %arg0)
         ret double %r"
    }

    fn llvm_log2f32(x: f32) -> f32 {
        "%r = call float @llvm.log2.f32(float %arg0)
         ret float %r"
    }

    fn llvm_log2f64(x: f64) -> f64 {
        "%r = call double @llvm.log2.f64(double %arg0)
         ret double %r"
    }

    fn llvm_fmaf32(a: f32, b: f32, c: f32) -> f32 {
        "%r = call float @llvm.fma.f32(float %arg0, float %arg1, float %arg2)
         ret float %r"
    }

    fn llvm_fmaf64(a: f64, b: f64, c: f64) -> f64 {
        "%r = call double @llvm.fma.f64(double %arg0, double %arg1, double %arg2)
         ret double %r"
    }

    fn llvm_fabsf32(x: f32) -> f32 {
        "%r = call float @llvm.fabs.f32(float %arg0)
         ret float %r"
    }

    fn llvm_fabsf64(x: f64) -> f64 {
        "%r = call double @llvm.fabs.f64(double %arg0)
         ret double %r"
    }

    fn llvm_floorf32(x: f32) -> f32 {
        "%r = call float @llvm.floor.f32(float %arg0)
         ret float %r"
    }

    fn llvm_floorf64(x: f64) -> f64 {
        "%r = call double @llvm.floor.f64(double %arg0)
         ret double %r"
    }

    fn llvm_ceilf32(x: f32) -> f32 {
        "%r = call float @llvm.ceil.f32(float %arg0)
         ret float %r"
    }

    fn llvm_ceilf64(x: f64) -> f64 {
        "%r = call double @llvm.ceil.f64(double %arg0)
         ret double %r"
    }

    fn llvm_truncf32(x: f32) -> f32 {
        "%r = call float @llvm.trunc.f32(float %arg0)
         ret float %r"
    }

    fn llvm_truncf64(x: f64) -> f64 {
        "%r = call double @llvm.trunc.f64(double %arg0)
         ret double %r"
    }

    fn llvm_i8_add_with_overflow(r: *mut (i8, bool), x: i8, y: i8) {
        "%s = call { i8, i1 } @llvm.sadd.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_i16_add_with_overflow(r: *mut (i16, bool), x: i16, y: i16) {
        "%s = call { i16, i1 } @llvm.sadd.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_i32_add_with_overflow(r: *mut (i32, bool), x: i32, y: i32) {
        "%s = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_i64_add_with_overflow(r: *mut (i64, bool), x: i64, y: i64) {
        "%s = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }

    fn llvm_u8_add_with_overflow(r: *mut (u8, bool), x: u8, y: u8) {
        "%s = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_u16_add_with_overflow(r: *mut (u16, bool), x: u16, y: u16) {
        "%s = call { i16, i1 } @llvm.uadd.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_u32_add_with_overflow(r: *mut (u32, bool), x: u32, y: u32) {
        "%s = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_u64_add_with_overflow(r: *mut (u64, bool), x: u64, y: u64) {
        "%s = call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }

    fn llvm_i8_sub_with_overflow(r: *mut (i8, bool), x: i8, y: i8) {
        "%s = call { i8, i1 } @llvm.ssub.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_i16_sub_with_overflow(r: *mut (i16, bool), x: i16, y: i16) {
        "%s = call { i16, i1 } @llvm.ssub.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_i32_sub_with_overflow(r: *mut (i32, bool), x: i32, y: i32) {
        "%s = call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_i64_sub_with_overflow(r: *mut (i64, bool), x: i64, y: i64) {
        "%s = call { i64, i1 } @llvm.ssub.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }

    fn llvm_u8_sub_with_overflow(r: *mut (u8, bool), x: u8, y: u8) {
        "%s = call { i8, i1 } @llvm.usub.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_u16_sub_with_overflow(r: *mut (u16, bool), x: u16, y: u16) {
        "%s = call { i16, i1 } @llvm.usub.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_u32_sub_with_overflow(r: *mut (u32, bool), x: u32, y: u32) {
        "%s = call { i32, i1 } @llvm.usub.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_u64_sub_with_overflow(r: *mut (u64, bool), x: u64, y: u64) {
        "%s = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }

    fn llvm_i8_mul_with_overflow(r: *mut (i8, bool), x: i8, y: i8) {
        "%s = call { i8, i1 } @llvm.smul.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_i16_mul_with_overflow(r: *mut (i16, bool), x: i16, y: i16) {
        "%s = call { i16, i1 } @llvm.smul.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_i32_mul_with_overflow(r: *mut (i32, bool), x: i32, y: i32) {
        "%s = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_i64_mul_with_overflow(r: *mut (i64, bool), x: i64, y: i64) {
        "%s = call { i64, i1 } @llvm.smul.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }

    fn llvm_u8_mul_with_overflow(r: *mut (u8, bool), x: u8, y: u8) {
        "%s = call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %arg1, i8 %arg2)
         %m = bitcast { i8, i8 }* %arg0 to { i8, i1 }*
         store { i8, i1 } %s, { i8, i1 }* %m
         ret void"
    }

    fn llvm_u16_mul_with_overflow(r: *mut (u16, bool), x: u16, y: u16) {
        "%s = call { i16, i1 } @llvm.umul.with.overflow.i16(i16 %arg1, i16 %arg2)
         %m = bitcast { i16, i8 }* %arg0 to { i16, i1 }*
         store { i16, i1 } %s, { i16, i1 }* %m
         ret void"
    }

    fn llvm_u32_mul_with_overflow(r: *mut (u32, bool), x: u32, y: u32) {
        "%s = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %arg1, i32 %arg2)
         %m = bitcast { i32, i8 }* %arg0 to { i32, i1 }*
         store { i32, i1 } %s, { i32, i1 }* %m
         ret void"
    }

    fn llvm_u64_mul_with_overflow(r: *mut (u64, bool), x: u64, y: u64) {
        "%s = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %arg1, i64 %arg2)
         %m = bitcast { i64, i8 }* %arg0 to { i64, i1 }*
         store { i64, i1 } %s, { i64, i1 }* %m
         ret void"
    }
})

#[fixed_stack_segment] pub fn ctpop8(x: i8) -> i8 { unsafe { llvm_ctpop8(x) } }
#[fixed_stack_segment] pub fn ctpop16(x: i16) -> i16 { unsafe { llvm_ctpop16(x) } }
#[fixed_stack_segment] pub fn ctpop32(x: i32) -> i32 { unsafe { llvm_ctpop32(x) } }
#[fixed_stack_segment] pub fn ctpop64(x: i64) -> i64 { unsafe { llvm_ctpop64(x) } }

#[fixed_stack_segment] pub fn ctlz8(x: i8) -> i8 { unsafe { llvm_ctlz8(x) } }
#[fixed_stack_segment] pub fn ctlz16(x: i16) -> i16 { unsafe { llvm_ctlz16(x) } }
#[fixed_stack_segment] pub fn ctlz32(x: i32) -> i32 { unsafe { llvm_ctlz32(x) } }
#[fixed_stack_segment] pub fn ctlz64(x: i64) -> i64 { unsafe { llvm_ctlz64(x) } }

#[fixed_stack_segment] pub fn cttz8(x: i8) -> i8 { unsafe { llvm_cttz8(x) } }
#[fixed_stack_segment] pub fn cttz16(x: i16) -> i16 { unsafe { llvm_cttz16(x) } }
#[fixed_stack_segment] pub fn cttz32(x: i32) -> i32 { unsafe { llvm_cttz32(x) } }
#[fixed_stack_segment] pub fn cttz64(x: i64) -> i64 { unsafe { llvm_cttz64(x) } }

#[fixed_stack_segment] pub fn bswap16(x: i16) -> i16 { unsafe { llvm_bswap16(x) } }
#[fixed_stack_segment] pub fn bswap32(x: i32) -> i32 { unsafe { llvm_bswap32(x) } }
#[fixed_stack_segment] pub fn bswap64(x: i64) -> i64 { unsafe { llvm_bswap64(x) } }

#[fixed_stack_segment] pub fn sqrtf32(x: f32) -> f32 { unsafe { llvm_sqrtf32(x) } }
#[fixed_stack_segment] pub fn sqrtf64(x: f64) -> f64 { unsafe { llvm_sqrtf64(x) } }

#[fixed_stack_segment] pub fn powif32(a: f32, x: i32) -> f32 { unsafe { llvm_powif32(a, x) } }
#[fixed_stack_segment] pub fn powif64(a: f64, x: i32) -> f64 { unsafe { llvm_powif64(a, x) } }

#[fixed_stack_segment] pub fn sinf32(x: f32) -> f32 { unsafe { llvm_sinf32(x) } }
#[fixed_stack_segment] pub fn sinf64(x: f64) -> f64 { unsafe { llvm_sinf64(x) } }

#[fixed_stack_segment] pub fn cosf32(x: f32) -> f32 { unsafe { llvm_cosf32(x) } }
#[fixed_stack_segment] pub fn cosf64(x: f64) -> f64 { unsafe { llvm_cosf64(x) } }

#[fixed_stack_segment] pub fn powf32(a: f32, x: f32) -> f32 { unsafe { llvm_powf32(a, x) } }
#[fixed_stack_segment] pub fn powf64(a: f64, x: f64) -> f64 { unsafe { llvm_powf64(a, x) } }

#[fixed_stack_segment] pub fn expf32(x: f32) -> f32 { unsafe { llvm_expf32(x) } }
#[fixed_stack_segment] pub fn expf64(x: f64) -> f64 { unsafe { llvm_expf64(x) } }

#[fixed_stack_segment] pub fn exp2f32(x: f32) -> f32 { unsafe { llvm_exp2f32(x) } }
#[fixed_stack_segment] pub fn exp2f64(x: f64) -> f64 { unsafe { llvm_exp2f64(x) } }

#[fixed_stack_segment] pub fn logf32(x: f32) -> f32 { unsafe { llvm_logf32(x) } }
#[fixed_stack_segment] pub fn logf64(x: f64) -> f64 { unsafe { llvm_logf64(x) } }

#[fixed_stack_segment] pub fn log10f32(x: f32) -> f32 { unsafe { llvm_log10f32(x) } }
#[fixed_stack_segment] pub fn log10f64(x: f64) -> f64 { unsafe { llvm_log10f64(x) } }

#[fixed_stack_segment] pub fn log2f32(x: f32) -> f32 { unsafe { llvm_log2f32(x) } }
#[fixed_stack_segment] pub fn log2f64(x: f64) -> f64 { unsafe { llvm_log2f64(x) } }

#[fixed_stack_segment] pub fn fmaf32(a: f32, b: f32, c: f32) -> f32 { unsafe {
    llvm_fmaf32(a, b, c)
}}
#[fixed_stack_segment] pub fn fmaf64(a: f64, b: f64, c: f64) -> f64 { unsafe {
    llvm_fmaf64(a, b, c)
}}

#[fixed_stack_segment] pub fn fabsf32(x: f32) -> f32 { unsafe { llvm_fabsf32(x) } }
#[fixed_stack_segment] pub fn fabsf64(x: f64) -> f64 { unsafe { llvm_fabsf64(x) } }

#[fixed_stack_segment] pub fn floorf32(x: f32) -> f32 { unsafe { llvm_floorf32(x) } }
#[fixed_stack_segment] pub fn floorf64(x: f64) -> f64 { unsafe { llvm_floorf64(x) } }

#[fixed_stack_segment] pub fn ceilf32(x: f32) -> f32 { unsafe { llvm_ceilf32(x) } }
#[fixed_stack_segment] pub fn ceilf64(x: f64) -> f64 { unsafe { llvm_ceilf64(x) } }

#[fixed_stack_segment] pub fn truncf32(x: f32) -> f32 { unsafe { llvm_truncf32(x) } }
#[fixed_stack_segment] pub fn truncf64(x: f64) -> f64 { unsafe { llvm_truncf64(x) } }

#[fixed_stack_segment] pub fn i8_add_with_overflow(x: i8, y: i8) -> (i8, bool) { unsafe {
    let mut r = (0i8, true); llvm_i8_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i16_add_with_overflow(x: i16, y: i16) -> (i16, bool) { unsafe {
    let mut r = (0i16, true); llvm_i16_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i32_add_with_overflow(x: i32, y: i32) -> (i32, bool) { unsafe {
    let mut r = (0i32, true); llvm_i32_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i64_add_with_overflow(x: i64, y: i64) -> (i64, bool) { unsafe {
    let mut r = (0i64, true); llvm_i64_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u8_add_with_overflow(x: u8, y: u8) -> (u8, bool) { unsafe {
    let mut r = (0u8, true); llvm_u8_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u16_add_with_overflow(x: u16, y: u16) -> (u16, bool) { unsafe {
    let mut r = (0u16, true); llvm_u16_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u32_add_with_overflow(x: u32, y: u32) -> (u32, bool) { unsafe {
    let mut r = (0u32, true); llvm_u32_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u64_add_with_overflow(x: u64, y: u64) -> (u64, bool) { unsafe {
    let mut r = (0u64, true); llvm_u64_add_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}

#[fixed_stack_segment] pub fn i8_sub_with_overflow(x: i8, y: i8) -> (i8, bool) { unsafe {
    let mut r = (0i8, true); llvm_i8_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i16_sub_with_overflow(x: i16, y: i16) -> (i16, bool) { unsafe {
    let mut r = (0i16, true); llvm_i16_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i32_sub_with_overflow(x: i32, y: i32) -> (i32, bool) { unsafe {
    let mut r = (0i32, true); llvm_i32_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i64_sub_with_overflow(x: i64, y: i64) -> (i64, bool) { unsafe {
    let mut r = (0i64, true); llvm_i64_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u8_sub_with_overflow(x: u8, y: u8) -> (u8, bool) { unsafe {
    let mut r = (0u8, true); llvm_u8_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u16_sub_with_overflow(x: u16, y: u16) -> (u16, bool) { unsafe {
    let mut r = (0u16, true); llvm_u16_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool) { unsafe {
    let mut r = (0u32, true); llvm_u32_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u64_sub_with_overflow(x: u64, y: u64) -> (u64, bool) { unsafe {
    let mut r = (0u64, true); llvm_u64_sub_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}

#[fixed_stack_segment] pub fn i8_mul_with_overflow(x: i8, y: i8) -> (i8, bool) { unsafe {
    let mut r = (0i8, true); llvm_i8_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i16_mul_with_overflow(x: i16, y: i16) -> (i16, bool) { unsafe {
    let mut r = (0i16, true); llvm_i16_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i32_mul_with_overflow(x: i32, y: i32) -> (i32, bool) { unsafe {
    let mut r = (0i32, true); llvm_i32_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn i64_mul_with_overflow(x: i64, y: i64) -> (i64, bool) { unsafe {
    let mut r = (0i64, true); llvm_i64_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u8_mul_with_overflow(x: u8, y: u8) -> (u8, bool) { unsafe {
    let mut r = (0u8, true); llvm_u8_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u16_mul_with_overflow(x: u16, y: u16) -> (u16, bool) { unsafe {
    let mut r = (0u16, true); llvm_u16_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u32_mul_with_overflow(x: u32, y: u32) -> (u32, bool) { unsafe {
    let mut r = (0u32, true); llvm_u32_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}
#[fixed_stack_segment] pub fn u64_mul_with_overflow(x: u64, y: u64) -> (u64, bool) { unsafe {
    let mut r = (0u64, true); llvm_u64_mul_with_overflow(ptr::to_mut_unsafe_ptr(&mut r), x, y); r
}}

#[cfg(target_endian = "little")] pub fn to_le16(x: i16) -> i16 { x }
#[cfg(target_endian = "big")]    pub fn to_le16(x: i16) -> i16 { bswap16(x) }
#[cfg(target_endian = "little")] pub fn to_le32(x: i32) -> i32 { x }
#[cfg(target_endian = "big")]    pub fn to_le32(x: i32) -> i32 { bswap32(x) }
#[cfg(target_endian = "little")] pub fn to_le64(x: i64) -> i64 { x }
#[cfg(target_endian = "big")]    pub fn to_le64(x: i64) -> i64 { bswap64(x) }

#[cfg(target_endian = "little")] pub fn to_be16(x: i16) -> i16 { bswap16(x) }
#[cfg(target_endian = "big")]    pub fn to_be16(x: i16) -> i16 { x }
#[cfg(target_endian = "little")] pub fn to_be32(x: i32) -> i32 { bswap32(x) }
#[cfg(target_endian = "big")]    pub fn to_be32(x: i32) -> i32 { x }
#[cfg(target_endian = "little")] pub fn to_be64(x: i64) -> i64 { bswap64(x) }
#[cfg(target_endian = "big")]    pub fn to_be64(x: i64) -> i64 { x }

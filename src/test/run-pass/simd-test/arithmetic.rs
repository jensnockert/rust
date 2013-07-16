simd!(i64x2: i64 * 2)
simd!(i32x4: i32 * 4)
simd!(i16x8: i16 * 8)
simd!(i8x16: i8 * 16)

simd!(u64x2: u64 * 2)
simd!(u32x4: u32 * 4)
simd!(u16x8: u16 * 8)
simd!(u8x16: u8 * 16)

simd!(f64x2: f64 * 2)
simd!(f32x4: f32 * 4)

fn main() {
    let a = [1.0f32, 2.0f32, 3.0f32, 4.0f32] as f32x4;
    let b = [5.0f32, 1.0f32, 2.0f32, 2.0f32] as f32x4;
    let c = a + b;
    assert_eq!(c.s0, 6.0);
    assert_eq!(c.s1, 3.0);
    assert_eq!(c.s2, 5.0);
    assert_eq!(c.s3, 6.0);

    let d = a.even + a.odd;
    assert_eq!(d.s0, 3.0);
    assert_eq!(d.s1, 7.0);
}
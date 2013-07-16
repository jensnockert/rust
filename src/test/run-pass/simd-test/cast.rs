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
    let a = 1.0f32 as f32x4;
    assert_eq!(a.s0, 1.0);
    assert_eq!(a.s1, 1.0);
    assert_eq!(a.s2, 1.0);
    assert_eq!(a.s3, 1.0);

    let b = [1.0f32, 2.0f32, 3.0f32, 4.0f32] as f32x4;
    assert_eq!(b.s0, 1.0);
    assert_eq!(b.s1, 2.0);
    assert_eq!(b.s2, 3.0);
    assert_eq!(b.s3, 4.0);

    let c = (0xFFFFFFFFu32 as u32x4) as i32x4;
    assert_eq!(c.s0, -1);
    assert_eq!(c.s1, -1);
    assert_eq!(c.s2, -1);
    assert_eq!(c.s3, -1);
}
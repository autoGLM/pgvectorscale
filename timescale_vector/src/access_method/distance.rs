/* we use the avx2 version of x86 functions. This verifies that's kosher */
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(not(target_feature = "avx2"))]
compile_error!(
    "On x86, the AVX2 feature must be enabled. Set RUSTFLAGS=\"-C target-feature=+avx2,+fma\""
);

#[inline]
pub fn distance_l2(a: &[f32], b: &[f32]) -> f32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    //note safety is guraranteed by compile_error above
    unsafe {
        return super::distance_x86::distance_l2_x86_avx2(a, b);
    }

    #[allow(unreachable_code)]
    {
        return distance_l2_unoptimized(a, b);
    }
}

#[inline(always)]
pub fn distance_l2_unoptimized(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let norm: f32 = a
        .iter()
        .zip(b.iter())
        .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
        .sum();
    assert!(norm >= 0.);
    //don't sqrt for performance. These are only used for ordering so sqrt not needed
    norm
}

/* PQ computes distances on subsegments that have few dimensions (e.g. 6). This function optimizes that.
* We optimize by telling the compiler exactly how long the slices are. This allows the compiler to figure
* out SIMD optimizations. Look at the benchmark results. */
#[inline]
pub fn distance_l2_optimized_for_few_dimensions(a: &[f32], b: &[f32]) -> f32 {
    let norm: f32 = match a.len() {
        0 => 0.,
        1 => a[..1]
            .iter()
            .zip(b[..1].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        2 => a[..2]
            .iter()
            .zip(b[..2].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        3 => a[..3]
            .iter()
            .zip(b[..3].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        4 => a[..4]
            .iter()
            .zip(b[..4].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        5 => a[..5]
            .iter()
            .zip(b[..5].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        6 => a[..6]
            .iter()
            .zip(b[..6].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        7 => a[..7]
            .iter()
            .zip(b[..7].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        8 => a[..8]
            .iter()
            .zip(b[..8].iter())
            .map(|t| (*t.0 as f32 - *t.1 as f32) * (*t.0 as f32 - *t.1 as f32))
            .sum(),
        _ => distance_l2(a, b),
    };
    assert!(norm >= 0.);
    //don't sqrt for performance. These are only used for ordering so sqrt not needed
    norm
}

#[inline]
pub fn distance_cosine(a: &[f32], b: &[f32]) -> f32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    //note safety is guraranteed by compile_error above
    unsafe {
        return super::distance_x86::distance_cosine_x86_avx2(a, b);
    }

    #[allow(unreachable_code)]
    {
        return distance_cosine_unoptimized(a, b);
    }
}

#[inline(always)]
pub fn distance_cosine_unoptimized(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    debug_assert!(preprocess_cosine_get_norm(a).is_none());
    debug_assert!(preprocess_cosine_get_norm(b).is_none());
    let res: f32 = a.iter().zip(b).map(|(a, b)| *a * *b).sum();
    (1.0 - res).max(0.0)
}

pub fn preprocess_cosine_get_norm(a: &[f32]) -> Option<f32> {
    let norm = a.iter().map(|v| v * v).sum::<f32>();
    //adjust the epsilon to the length of the vector
    let adj_epsilon = f32::EPSILON * a.len() as f32;

    /* this mainly handles the zero-vector case */
    if norm < f32::EPSILON {
        return None;
    }
    /* no need to renormalize if norm around 1.0 */
    if norm >= 1.0 - adj_epsilon && norm <= 1.0 + adj_epsilon {
        return None;
    }
    return Some(norm.sqrt());
}

pub fn preprocess_cosine(a: &mut [f32]) {
    let norm = preprocess_cosine_get_norm(a);
    match norm {
        None => (),
        Some(norm) => {
            a.iter_mut().for_each(|v| *v /= norm);
            debug_assert!(
                preprocess_cosine_get_norm(a).is_none(),
                "preprocess_cosine isn't idempotent",
            );
        }
    }
}
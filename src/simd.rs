///
/// Imports
///
use std::arch::x86_64;

///
/// Public Functions
///

/// Performs vector addition using idiomatic Rust code. The Rust compiler is smart
/// enough to generate auto-vectorized code that is highly performant.
pub fn vector_add(slice1: &[i32], slice2: &[i32], result_slice: &mut [i32]) {
    let iterator = slice1.iter().zip(slice2).zip(result_slice);
    for ((ptr1, ptr2), result_ptr) in iterator {
        *result_ptr = *ptr1 + *ptr2;
    }
}

/// Performs vector addition using 128-bit SIMD vectors and SSE2 features.
pub unsafe fn vector_add_sse2(slice1: &[i32], slice2: &[i32], result_slice: &mut [i32]) {
    let slice1_ptr = &slice1[0] as *const _ as *const x86_64::__m128i;
    let slice2_ptr = &slice2[0] as *const _ as *const x86_64::__m128i;
    let result_slice_ptr = &mut result_slice[0] as *mut _ as *mut x86_64::__m128i;

    let n = slice1.len() / 4;
    for i in 0..n {
        // Build a 128-bit vector from four 32-bit integers in `slice1`.
        let vec1 = x86_64::_mm_load_si128(slice1_ptr.add(i));

        // Build a 128-bit vector from four 32-bit integers in `slice2`.
        let vec2 = x86_64::_mm_load_si128(slice2_ptr.add(i));

        // Add the two 128-bit vectors, but 32-bits at a time.
        let result_vec = x86_64::_mm_add_epi32(vec1, vec2);

        // Store the 128-bit vector as four 32-bit integers in `result_slice`;
        x86_64::_mm_store_si128(result_slice_ptr.add(i), result_vec);
    }
}

pub const DATA32_LENGTH: usize = 1 << 10;

// A 256-bit aligned array is needed for using 256-bit SIMD instructions.
#[repr(align(32))]
pub struct Data32(pub [i32; DATA32_LENGTH]);

impl Data32 {
    pub fn new() -> Self {
        Data32([0; DATA32_LENGTH])
    }
}

/// Performs vector addition using 256-bit SIMD vectors and AVX2 features.
pub unsafe fn vector_add_avx2(slice1: &[i32], slice2: &[i32], result_slice: &mut [i32]) {
    let slice1_ptr = &slice1[0] as *const _ as *const x86_64::__m256i;
    let slice2_ptr = &slice2[0] as *const _ as *const x86_64::__m256i;
    let result_slice_ptr = &mut result_slice[0] as *mut _ as *mut x86_64::__m256i;

    let n = slice1.len() as isize / 8;
    for i in 0..n {
        // Build a 256-bit vector from eight 32-bit integers in `slice1`.
        let vec1 = x86_64::_mm256_load_si256(slice1_ptr.offset(i));

        // Build a 256-bit vector from eight 32-bit integers in `slice2`.
        let vec2 = x86_64::_mm256_load_si256(slice2_ptr.offset(i));

        // Add the two 256-bit vectors, but 32-bits at a time.
        let result_vec = x86_64::_mm256_add_epi32(vec1, vec2);

        // Store the 128-bit vector as four 32-bit integers in `result_slice`;
        x86_64::_mm256_store_si256(result_slice_ptr.offset(i), result_vec);
    }
}

#[cfg(test)]
mod tests {
    ///
    /// Imports
    ///
    use super::*;
    use rand::Rng;

    ///
    /// Constants
    ///
    const VECTOR_LENGTH: usize = 16;
    const MIN: i32 = i32::MIN / 3;
    const MAX: i32 = i32::MAX / 3;

    ///
    /// Test Helpers
    ///

    /// Returns a Vector<i32> with random values.
    fn gen_random_vector() -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut v = vec![0; VECTOR_LENGTH];
        for i in 0..VECTOR_LENGTH {
            v[i] = rng.gen_range(MIN, MAX);
        }
        v
    }

    /// Returns a Box<Data32> with random values.
    fn gen_random_data32() -> Box<Data32> {
        let mut rng = rand::thread_rng();
        let mut v = Data32::new();
        for i in 0..DATA32_LENGTH {
            v.0[i] = rng.gen_range(MIN, MAX);
        }
        Box::new(v)
    }

    /// Returns true if the two vectors contain exactly the same values.
    /// Otherwise, false.
    fn vectors_are_equal(v1: &[i32], v2: &[i32]) -> bool {
        for (x1, x2) in v1.iter().zip(v2) {
            if *x1 != *x2 {
                return false;
            }
        }
        true
    }

    ///
    /// Tests for vector_add
    ///

    #[test]
    fn vector_add_has_identity_zero() {
        let id = vec![0; VECTOR_LENGTH];
        let v1 = gen_random_vector();
        let mut result = vec![0; VECTOR_LENGTH];
        vector_add(&id, &v1, &mut result);

        assert!(vectors_are_equal(&v1, &result));
    }

    #[test]
    fn vector_add_is_commutative() {
        let v1 = gen_random_vector();
        let v2 = gen_random_vector();
        let mut result1 = vec![0; VECTOR_LENGTH];
        let mut result2 = vec![0; VECTOR_LENGTH];
        vector_add(&v1, &v2, &mut result1);
        vector_add(&v2, &v1, &mut result2);

        assert!(vectors_are_equal(&result1, &result2));
    }

    #[test]
    fn vector_add_is_associative() {
        let v1 = gen_random_vector();
        let v2 = gen_random_vector();
        let v3 = gen_random_vector();
        let mut interim_result = vec![0; VECTOR_LENGTH];
        let mut result1 = vec![0; VECTOR_LENGTH];
        let mut result2 = vec![0; VECTOR_LENGTH];

        vector_add(&v1, &v2, &mut interim_result);
        vector_add(&interim_result, &v3, &mut result1);

        vector_add(&v2, &v3, &mut interim_result);
        vector_add(&v1, &interim_result, &mut result2);

        assert!(vectors_are_equal(&result1, &result2));
    }

    ///
    /// Tests for vector_add_sse2
    ///

    #[test]
    fn vector_add_sse2_has_identity_zero() {
        let id = vec![0; VECTOR_LENGTH];
        let v1 = gen_random_vector();
        let mut result = vec![0; VECTOR_LENGTH];

        unsafe {
            vector_add_sse2(&id, &v1, &mut result);
        }

        assert!(vectors_are_equal(&v1, &result));
    }

    #[test]
    fn vector_add_sse2_is_commutative() {
        let v1 = gen_random_vector();
        let v2 = gen_random_vector();
        let mut result1 = vec![0; VECTOR_LENGTH];
        let mut result2 = vec![0; VECTOR_LENGTH];

        unsafe {
            vector_add_sse2(&v1, &v2, &mut result1);
            vector_add_sse2(&v2, &v1, &mut result2);
        }

        assert!(vectors_are_equal(&result1, &result2));
    }

    #[test]
    fn vector_add_sse2_is_associative() {
        let v1 = gen_random_vector();
        let v2 = gen_random_vector();
        let v3 = gen_random_vector();
        let mut interim_result = vec![0; VECTOR_LENGTH];
        let mut result1 = vec![0; VECTOR_LENGTH];
        let mut result2 = vec![0; VECTOR_LENGTH];

        unsafe {
            vector_add_sse2(&v1, &v2, &mut interim_result);
            vector_add_sse2(&interim_result, &v3, &mut result1);

            vector_add_sse2(&v2, &v3, &mut interim_result);
            vector_add_sse2(&v1, &interim_result, &mut result2);
        }

        assert!(vectors_are_equal(&result1, &result2));
    }

    ///
    /// Tests for vector_add_avx2
    ///

    #[test]
    fn vector_add_avx2_has_identity_zero() -> Result<(), &'static str> {
        let id = Data32::new();
        let v1 = gen_random_data32();
        let mut result = Data32::new();

        unsafe {
            vector_add_avx2(&id.0, &v1.0, &mut result.0);
        }

        assert!(vectors_are_equal(&v1.0, &result.0));
        Ok(())
    }

    #[test]
    fn vector_add_avx2_is_commutative() -> Result<(), &'static str> {
        let v1 = gen_random_data32();
        let v2 = gen_random_data32();
        let mut result1 = Data32::new();
        let mut result2 = Data32::new();

        unsafe {
            vector_add_avx2(&v1.0, &v2.0, &mut result1.0);
            vector_add_avx2(&v2.0, &v1.0, &mut result2.0);
        }

        assert!(vectors_are_equal(&result1.0, &result2.0));
        Ok(())
    }

    #[test]
    fn vector_add_avx2_is_associative() -> Result<(), &'static str> {
        let v1 = gen_random_data32();
        let v2 = gen_random_data32();
        let v3 = gen_random_data32();
        let mut interim_result = Data32::new();
        let mut result1 = Data32::new();
        let mut result2 = Data32::new();

        unsafe {
            vector_add_avx2(&v1.0, &v2.0, &mut interim_result.0);
            vector_add_avx2(&interim_result.0, &v3.0, &mut result1.0);

            vector_add_avx2(&v2.0, &v3.0, &mut interim_result.0);
            vector_add_avx2(&v1.0, &interim_result.0, &mut result2.0);
        }

        assert!(vectors_are_equal(&result1.0, &result2.0));
        Ok(())
    }
}

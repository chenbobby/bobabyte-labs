use std::arch::x86_64;

pub fn hello() {
    // TODO: Perhaps change these to vectors.
    let data1: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let data2: Vec<i32> = vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 2147483647];
    let mut results: Vec<i32> = vec![0; 16];
    unsafe { vector_add_sse2(&data1, &data2, &mut results); }
    println!("{:?}", results);
}

/// Although `add` does not use SIMD intrinsics, the Rust compiler may auto-vectorize
/// the loop based on the compilation target. You can disable auto-vectorization by
/// using the flag `no-vectorize-loops`. See https://doc.rust-lang.org/cargo/reference/profiles.html
/// for more details.
fn vector_add(slice1: &[i32], slice2: &[i32], result_slice: &mut [i32]) {
    let iterator = slice1.iter().zip(slice2).zip(result_slice);
    for ((ptr1, ptr2), result_ptr) in iterator {
        *result_ptr = *ptr1 + *ptr2;
    }
}

unsafe fn vector_add_sse2(slice1: &[i32], slice2: &[i32], result_slice: &mut [i32]) {
    let iterator = slice1.iter().zip(slice2).zip(result_slice).step_by(4);
    for ((ptr1, ptr2), result_ptr) in iterator {
        // Build a 128-bit vector from four 32-bit integers in `slice1`.
        let vec1_ptr = ptr1 as *const _ as *const x86_64::__m128i;
        let vec1 = x86_64::_mm_load_si128(vec1_ptr);

        // Build a 128-bit vector from four 32-bit integers in `slice2`.
        let vec2_ptr = ptr2 as *const i32 as *const x86_64::__m128i;
        let vec2 = x86_64::_mm_load_si128(vec2_ptr);

        // Add the two 128-bit vectors, but 32-bits at a time.
        let result_vec = x86_64::_mm_add_epi32(vec1, vec2);

        // Store the 128-bit vector as four 32-bit integers in `result_slice`;
        let result_vec_ptr = result_ptr as *mut i32 as *mut x86_64::__m128i;
        x86_64::_mm_store_si128(result_vec_ptr, result_vec);
    }
}

#[cfg(test)]
mod tests {
    ///
    /// Imports
    ///
    use rand::Rng;
    use super::*;

    /// 
    /// Constants
    ///
    const VECTOR_LENGTH: usize = 16;
    const MIN: i32 = i32::MIN/2;
    const MAX: i32 = i32::MAX/2;

    ///
    /// Test Helpers
    ///

    /// Returns a vector of i32 elements with random values.
    fn gen_random_vector() -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut v = vec![0; VECTOR_LENGTH];
        for i in 0..VECTOR_LENGTH {
            v[i] = rng.gen_range(MIN, MAX);
        }
        v
    }

    /// Returns true if the two vectors contain exactly the same values.
    fn vectors_are_equal(v1: &[i32], v2: &[i32]) -> bool {
        for (x1, x2) in v1.iter().zip(v2) {
            if *x1 != *x2 {
                return false;
            }
        }
        return true;
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

        unsafe{ vector_add_sse2(&id, &v1, &mut result); }

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
}

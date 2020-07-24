pub fn vector_increment_fissioned(slice1: &mut [i32], slice2: &mut [i32]) {
    for x in slice1.iter_mut() {
        *x += 1;
    }

    for x in slice2.iter_mut() {
        *x += 1;
    }
}

pub fn vector_increment_unfissioned(slice1: &mut [i32], slice2: &mut [i32]) {
    for (x, y) in slice1.iter_mut().zip(slice2.iter_mut()) {
        *x += 1;
        *y += 1;
    }
}

pub fn iter_sums_u_subs ( v: &[u128] ) -> u128 {
    let mut i = v.iter();
    let first = i.next().unwrap();
    let mut prev = first;
    let mut r = 0;

    for e in i {
        r += u_substract(*prev, *e);
        prev = e;
    }

    return r + u_substract(*prev, *first);
    
}
pub fn u_substract ( a: u128, b: u128 ) -> u128 {
    if a < b {
        b - a
    } else {
        a - b
    }
}


use std::mem;

#[link(name = "merkletree_cuda")]
extern "C" {
    fn fill_digests(leaves: *mut u64, leaves_len: u32, cap_height: u32) -> *mut u64;
}

#[allow(unused_mut)]
pub fn fill_digests_cuda<F, H: std::clone::Clone>(
    leaves: Vec<Vec<F>>,
    cap_height: usize,
) -> (Vec<H>, Vec<H>) {
    let leaves_len = leaves.len();
    let mut digests_cap: &[H];
    let mut digests: Vec<H>;
    let mut cap: Vec<H>;
    let mut flattened_leaves = leaves.into_iter().flatten().collect::<Vec<F>>();

    unsafe {
        let ptr0 = flattened_leaves.as_mut_ptr();
        let ptr1 = mem::transmute::<*mut F, *mut u64>(ptr0);
        let tmp = mem::transmute::<*mut u64, *mut H>(fill_digests(
            ptr1,
            leaves_len as u32,
            cap_height as u32,
        ));
        digests_cap = std::slice::from_raw_parts(tmp, leaves_len * 2);
        digests = digests_cap[..(leaves_len * 2 - 2)].to_vec();
        cap = digests_cap[(leaves_len * 2 - 2)..(leaves_len * 2 - 1)].to_vec();
    }

    (digests, cap)
}

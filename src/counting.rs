use crate::match_finder::Match;
use crate::mdma::MdmaIndex;

pub fn count(m: &mut Match, mdma_index: &MdmaIndex) -> (i32, usize) {
    match m.self_ref {
        false => count_fast(m, mdma_index),
        true => count_slow(m, mdma_index)
    }
}

fn count_fast(m: &mut Match, mdma_index: &MdmaIndex) -> (i32, usize) {
    let mut count = 0;
    let effective_len = m.len as i32 - 1;

    let last_match = mdma_index.sa[m.sa_index as usize] as usize;
    let range = m.get_range();

    // TODO: Try unroll?
    for loc in &mdma_index.sa[range] {
        if mdma_index.offsets[*loc as usize] >= effective_len { count += 1; }
    }

    (count, last_match)
}

fn count_slow(m: &mut Match, mdma_index: &MdmaIndex) -> (i32, usize) {
    let range = m.get_range();
    let mut locations = vec![0; range.len()];
    locations.copy_from_slice(&mdma_index.sa[range]);
    locations.sort_unstable();

    let effective_len = m.len as i32 - 1;
    let mut count = 0;
    let mut flag = false;
    let mut last_match = - (m.len as i32);

    for loc in locations {
        // TODO: Optimize branching? -> there're no branches in the loop,
        // but the compiler can't (won't) unroll because of the dependency on last_match
        // It's not clear how to unroll either, there's a bunch of ways matches may intertwine
        // if the outer ends of 2 matches are far apart enough to fit 2 matches we can consider 2 matches
        // but perhaps the branch predictor is fine as it is and can even speculatively prefetch offsets[loc]
        if loc <= last_match + effective_len { flag = true; continue; }

        if mdma_index.offsets[loc as usize] >= effective_len {
            count += 1;
            last_match = loc;
        }
    }

    m.self_ref = flag;
    (count, last_match as usize)
}
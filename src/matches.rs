pub const NUM_FILE_CATEGORIES: usize = 6;

// matches, are stored in a vector of Vec<String> where index from the root represents a file with different properties:
// 0 -> not hidden / file
// 1 -> not hidden / symlink
// 2 -> not hidden / directory
// 3 -> hidden / file
// 4 -> hidden / symlink
// 5 -> hidden / directory

// initialise_matches_capacities, initialises the vector capacities based on the sample directory used in `BEMCHARKS.md`
// TODO: Occurence ratios below could be more generalised
pub fn initialise_matches_capacities(fd_limit: usize) -> [Vec<String>; NUM_FILE_CATEGORIES] {
    let mut matches: [Vec<String>; NUM_FILE_CATEGORIES] = [const { Vec::new() }; NUM_FILE_CATEGORIES];
    let mut left = fd_limit;
    let type_mults = vec![92.5/100.0, 1.0/100.0, 6.5/100.0];
    let hidden_mults = vec![99.9/100.0, 0.1/100.0];
    for i in 0..NUM_FILE_CATEGORIES {
        let file_type_idx = i % 3;
        let is_hidden_idx = (i >= 3) as usize;
        let cap = ((fd_limit as f64) * type_mults[file_type_idx] * hidden_mults[is_hidden_idx]).floor() as usize;
        
        matches[i] = Vec::with_capacity(cap);
        left -= cap;

    }
    let leftover_idx: usize = 2;
    matches[leftover_idx] = Vec::with_capacity(matches[leftover_idx].capacity() + left);
    return matches;
}

// insert_entry_in_matches, inserts an entry in `matches` at the index that corresponds to its properties according to the following formula:
//  (is_hidden * 3) + (IS_FILE ? 0 : (IS_SYMLINK ? 1 : 2))
pub fn insert_entry_in_matches(matches: &mut [Vec<String>; NUM_FILE_CATEGORIES], ent: String, hidden: bool, file: bool, symlink: bool) {
    let hidden_type_multi = hidden as usize;
    let entry_type_offset = (symlink as usize) + ((!file as usize) * 2);
    let idx = (hidden_type_multi * 3) + entry_type_offset;
    matches[idx].push(ent);
}
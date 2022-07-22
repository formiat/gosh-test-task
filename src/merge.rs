use std::cmp::min;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum MergeDirection {
    Direct,
    Reverse,
}

impl MergeDirection {
    pub fn check_for_mergeability(substr_1: &[char], substr_2: &[char]) -> (Self, usize) {
        let best_direct_shrink = Self::check_one_direction(substr_1, substr_2);
        let best_reverse_shrink = Self::check_one_direction(substr_2, substr_1);

        if best_direct_shrink > best_reverse_shrink {
            (Self::Direct, best_direct_shrink)
        } else {
            (Self::Reverse, best_reverse_shrink)
        }
    }

    fn check_one_direction(substr_1: &[char], substr_2: &[char]) -> usize {
        let mut best_shrink = 0;

        let max_n = min(substr_1.len(), substr_2.len());

        for n in 1..max_n {
            let slice_1 = &substr_1[substr_1.len() - n..];
            let slice_2 = &substr_2[..n];

            if slice_1 == slice_2 {
                best_shrink = n;
            }
        }

        best_shrink
    }

    fn merge_two_substrings(
        &self,
        substr_1: &[char],
        substr_2: &[char],
        shrink: usize,
    ) -> Vec<char> {
        let (substr_1, substr_2) = match self {
            Self::Direct => (substr_1, substr_2),
            Self::Reverse => (substr_2, substr_1),
        };

        let mut substr_1: Vec<char> = substr_1[..substr_1.len() - shrink].to_vec();
        substr_1.append(&mut substr_2.to_vec());

        substr_1
    }
}

fn join_vec<T>(mut vec: Vec<Vec<T>>) -> Vec<T> {
    let mut new_vec = Vec::new();

    for item in &mut vec {
        new_vec.append(item);
    }

    new_vec
}

pub fn compressive_merge(mut substr_list: Vec<String>) -> String {
    remove_substrings(&mut substr_list);

    let mut substr_list = substr_list
        .into_iter()
        .map(|v| v.chars().collect())
        .collect();

    // While changes are made
    while merge_iteration(&mut substr_list) {}

    join_vec(substr_list).into_iter().collect()
}

fn clean<T>(substr_list: &mut Vec<T>, ids_to_remove: HashSet<usize>) {
    let mut ids_to_remove: Vec<_> = ids_to_remove.into_iter().collect();
    ids_to_remove.sort_by(|a, b| b.cmp(a));

    if !ids_to_remove.is_empty() {
        for id in ids_to_remove {
            substr_list.swap_remove(id);
        }
    }
}

fn remove_substrings(substr_list: &mut Vec<String>) {
    let mut ids_to_remove = HashSet::new();

    for (i, substr_1) in substr_list.iter().enumerate() {
        for (j, substr_2) in substr_list.iter().enumerate().skip(i + 1) {
            if substr_1.contains(substr_2) {
                ids_to_remove.insert(j);
            } else if substr_2.contains(substr_1) {
                ids_to_remove.insert(i);
            }
        }
    }

    clean(substr_list, ids_to_remove);
}

fn merge_one_substring(
    substr_list: &[Vec<char>],
    i: usize,
    substr_1: &[char],
) -> Option<(usize, Vec<char>)> {
    let mut best_merge_index = None;
    let mut best_merge_direction = None;
    let mut best_merge_shrink = 0;

    for (j, substr_2) in substr_list.iter().enumerate().skip(i + 1) {
        let (merge_direction, shrink) = MergeDirection::check_for_mergeability(substr_1, substr_2);

        if shrink > best_merge_shrink {
            best_merge_index = Some(j);
            best_merge_direction = Some(merge_direction);
            best_merge_shrink = shrink;
        }
    }

    if let Some(best_merge_index) = best_merge_index {
        let best_merge_result = best_merge_direction.unwrap().merge_two_substrings(
            &substr_list[i],
            &substr_list[best_merge_index],
            best_merge_shrink,
        );

        Some((best_merge_index, best_merge_result))
    } else {
        None
    }
}

/// Return: `changes have been made`
fn merge_iteration(substr_list: &mut Vec<Vec<char>>) -> bool {
    let mut ids_to_remove = HashSet::new();
    let mut substr_list_new = HashMap::new();

    for (i, substr_1) in substr_list.iter().enumerate() {
        if let Some((j, substr_new)) = merge_one_substring(substr_list.as_slice(), i, substr_1) {
            substr_list_new.insert(j, substr_new);
            ids_to_remove.insert(i);

            break;
        }
    }
    for (i, substr_new) in substr_list_new {
        substr_list[i] = substr_new;
    }

    let changes_are_made = !ids_to_remove.is_empty();
    clean(substr_list, ids_to_remove);

    changes_are_made
}

#![allow(unused)]

use std::collections::HashMap;

fn main() {}

// #560
fn subarray_sum(nums: Vec<i32>, target: i32) -> i32 {
    let mut count = 0;
    let mut sum = 0;
    let mut map = HashMap::new();
    map.insert(0, 1);
    for i in 0..nums.len() {
        sum += nums[i];
        count += map.get(&(sum-target)).unwrap_or(&0);
        map.insert(sum, *map.get(&sum).unwrap_or(&0) + 1);
    }
    return count;
}

#[test]
fn test_subarray_sum() {
    let numss = [
        vec![1,-1,6,2,3,1,-3],
        vec![1,2,3,2,5,4,-2,2,1,-1,1,4,-2],
        vec![3,2,-1,4,2,1,-4,2],
        vec![-1,3,2,4,5,3,2],
        vec![6,-2,1,1,3,4,2]
    ];
    let targets = vec![
        6,3,5,8,7
    ];
    let expected = vec![
        3,7,4,2,2
    ];
    for i in 0..expected.len() {
        assert_eq!(subarray_sum(numss[i].clone(), targets[i]), expected[i]);
    }
}

// #74
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if target < matrix[0][0] || target > *matrix.last().unwrap().last().unwrap() {
        return false;
    }
    let last_elem_idx = matrix[0].len() - 1;
    let (mut low, mut high) = (0, matrix.len() - 1);
    let mut mid = (low + high) / 2;

    while low != high {
        if target <= matrix[mid][last_elem_idx] {
            high = mid;
        } else {
            low = mid + 1;
        }
        mid = (low + high) / 2;
    }

    let row_idx = mid;
    low = 0;
    high = last_elem_idx;
    mid = (low + high) / 2;

    while low != high {
        if target <= matrix[row_idx][mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
        mid = (low + high) / 2;
    }

    if target == matrix[row_idx][mid] {
        return true
    }
    return false
}

#[test]
fn test_search_matrix() {
    let matrix = vec![
        vec![ 1, 3, 5, 7],//0
        vec![10,11,16,20],//1
        vec![23,30,34,60],//2
        vec![64,67,71,72],//3
        vec![73,75,79,80],//4
        vec![84,85,87,89],//5
        //   0  1  2  3
    ];
    let targets = vec![
        -35,-4,1,5,8,9,10,11,14,19,20,
        22,23,24,25,30,31,34,60,68,93
    ];
    let expected = vec![
        false,false,true,true,false,false,true,true,false,false,true,//20
        false,true,false,false,true,false,true,true,false,false
    ];
    for i in 0..targets.len() {
        println!("test #{}",i);
        assert_eq!(search_matrix(matrix.clone(), targets[i]), expected[i])
    }
}

// #46
fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    return permute_dfs(nums, vec![]);
}

fn permute_dfs(candidates: Vec<i32>, result: Vec<i32>) -> Vec<Vec<i32>> {
    if candidates.is_empty() {
        return vec![result];
    }
    let mut final_result = Vec::with_capacity((1..=candidates.len()).product());
    for i in 0..candidates.len() {
        let mut removed_candidates = candidates.clone();
        let mut added_result = result.clone();
        added_result.push(removed_candidates.remove(i));
        final_result.append(&mut permute_dfs(removed_candidates, added_result));
    }
    return final_result
}

// #567
fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }
    let mut map = [0; 26];
    for c in s1.chars() {
        map[c as usize - 'a' as usize] += 1;
    }
    for (i, c) in s2.chars().enumerate() {
        map[c as usize - 'a' as usize] -= 1;
        if i >= s1.len() {
            let idx = i - s1.len();
            let last_char = &s2[idx..idx+1];
            map[last_char.chars().next().unwrap() as usize - 'a' as usize] += 1;
        }
        if map.iter().all(|&v| v == 0) {
            return true;
        }
    }
    return false;
}
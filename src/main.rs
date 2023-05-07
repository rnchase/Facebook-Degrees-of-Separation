use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
mod my_module;
use crate::my_module::average;
use crate::my_module::median;
use crate::my_module::six_degrees;
use crate::my_module::max;


fn main() {
    let my_list = read_file("facebook_combined.csv");
    let mut n = 4039;
    let mut adj_list : Vec<Vec<usize>> = vec![vec![]; n];
    for (v,w) in my_list.iter() {
        let v_us = usize::try_from(*v).unwrap();
        let w_us = usize::try_from(*w).unwrap();
        adj_list[v_us].push(w_us);
    };

    let mut distances = Vec::new();
    let mut start_end_vec: Vec<(usize,usize,usize)> = Vec::new();
    get_distances(&mut distances, &mut start_end_vec, &adj_list);
    

   
    
    let mut remove_zero_distances = remove_zero(&mut distances);
    average(&remove_zero_distances);
    median(remove_zero_distances.to_vec());
    six_degrees(&distances);
    max(&distances, start_end_vec);
}   

fn get_distances(distances: &mut Vec<usize>, start_end_vec: &mut Vec<(usize,usize,usize)>, adj_list: &Vec<Vec<usize>>) {
    for start in 0..adj_list.len() {
        for end in start..adj_list.len() {
            if let Some(x) = degrees(&adj_list, start, end) {
                let current_distance = degrees(&adj_list, start, end).unwrap();
                let mut start_end = vec![current_distance,start,end];
                start_end_vec.push((current_distance, start, end));
                distances.push(current_distance);
                //println!("The distance between {} and {} is {}", start, end, current_distance);
            }
            else {
                //println!("There is no path from {} to {} ", start, end);
                start_end_vec.push((0, start, end));
            }
        

        }
    }
}

fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut entries: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        if v.len() == 1{    
                continue;
        }
        let x = v[0].parse::<usize>().unwrap();
        let y = v[1].parse::<usize>().unwrap();
        entries.push((x, y));
        
    }
    return entries;
}

fn degrees(adj_list: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<usize> { 
    let mut queue = VecDeque::new();
    let mut visits = vec![false; adj_list.len()]; // creates vector of visited points 

    //setting the start point to be visited and in queue
    visits[start] = true;
    queue.push_back((start, 0));

    while let Some((node, distance)) = queue.pop_front() { 
        if node == end {
            return Some(distance);
        }
        for &neighbor in &adj_list[node]{
            if !visits[neighbor] {
                visits[neighbor] = true;
                queue.push_back((neighbor, distance+1));
            } 

            
        }
        
    }
    return None;

}

fn remove_zero(distances: &mut Vec<usize>) -> &Vec<usize> {
    distances.retain(|&x| x != 0);
    return distances;
}

#[test] 
fn test_zero() {
    let test_adj = vec![vec![0], vec![2], vec![1]];
    assert_eq!(degrees(&test_adj,0,0), Some(0));
}

#[test] 
fn test_one() {
    let test_adj = vec![vec![0], vec![2], vec![1]];
    assert_eq!(degrees(&test_adj,1,2), Some(1));
}

#[test] 
fn test_None() {
    let test_adj = vec![vec![0], vec![2], vec![1]];
    assert_eq!(degrees(&test_adj, 0,2), None);
}












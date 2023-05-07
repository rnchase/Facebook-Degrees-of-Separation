
pub fn average(remove_zero_distances: &Vec<usize>) { 
    let mut total = 0;
    for x in 0..remove_zero_distances.len() {
        total += remove_zero_distances[x];
    }
    let avg = total/remove_zero_distances.len();
    println!("The average distance is {}", avg);
}

pub fn median(mut remove_zero_distances:Vec<usize>) {  
    remove_zero_distances.sort();
   let middle = remove_zero_distances.len()/2;
   let median_vec = remove_zero_distances[middle];
   println!("The median is {}", median_vec);
        
}

pub fn six_degrees(distances: &Vec<usize>) { 
    for x in 0..distances.len() { 
        if distances[x] > 6 {
            println!("There is a distance that is greater than 6, therefore failing the 6 degrees of separation");
            break;
        }
    
    }
}

pub fn max(distances: &Vec<usize>, start_end_vec: Vec<(usize,usize,usize)>) { 
    let mut index = 0;
    let mut highest = 0;
    for x in 0..distances.len() {
        if distances[x] > highest{
            index = x;
            highest = distances[x];
        }
    }
    println!("The maximum distance is {} and the start point is {} with the end point {}", highest, start_end_vec[index].1, start_end_vec[index].2);
}
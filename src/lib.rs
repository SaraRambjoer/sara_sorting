use core::f64;
use std::iter;

use numeric_equivalent::NumericEquivalent;
use rand::{thread_rng, Rng, RngCore};

mod numeric_equivalent;




// this kind of sorts a little for each pass. each pass is essentially a shittier version of one pass bubble sort.
pub fn trampoline_sort<T: NumericEquivalent>(mut to_sort: Vec<T>) -> Vec<T> {
    let mut min = f64::MAX;
    let mut max: f64 = f64::MIN;

    for x in &to_sort {
        let val = x.get_numeric();
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    let sort_length = to_sort.len();
    let mut rng = thread_rng();

    let mut i0 = 0;
    while i0 < sort_length - 1 {
        if i0 > 1 && to_sort.get(i0).expect("").get_numeric() < to_sort.get(i0-1).expect("").get_numeric() {
            to_sort.swap(i0, i0-1);

            if i0 == 1 {
                break;
            }
         
            // boing
            let randswap = rng.gen_range(0..(i0-1));
            to_sort.swap(i0-1, randswap)

        } 
        else if to_sort.get(i0).expect("").get_numeric() > to_sort.get(i0+1).expect("").get_numeric() {
            to_sort.swap(i0, i0+1);

            if i0 == sort_length-1 {
                break;
            }
         
            // boing
            let randswap = rng.gen_range(i0+1..sort_length);
            to_sort.swap(i0+1, randswap)
        }
        else {
            i0 += 1;
        }
    }

    to_sort
}

pub fn approximate_box_sort<T: NumericEquivalent>(to_sort: Vec<T>, bins: i64)  -> Vec<T> {
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut numeric_vecs: Vec<f64> = Vec::new();

    for var in &to_sort {
        let val = var.get_numeric();
        numeric_vecs.push(val);
        if val < min {
            min = val;
        }
        
        if val > max {
            max = val;
        }
    }

    let mut sort_into_bins: Vec<Vec<T>> = Vec::new();
    let interval = (max+1.0-min)/(bins as f64);

    for _ in 0..bins {
        sort_into_bins.push(Vec::new());
    }

    let mut i0 = 0;
    for x in to_sort {
        let val = numeric_vecs.get(i0).expect("sort error");
        let i1: usize = ((val - min)/interval).floor() as usize;
        let bin: &mut Vec<T> = sort_into_bins.get_mut(i1).expect("Bin error");
        bin.push(x);
        i0 += 1;
    }

    sort_into_bins.into_iter().flatten().collect()
}

// Approx. k-sort with k = total/bins
pub fn approximate_gaussian_sort<T: NumericEquivalent>(to_sort: Vec<T>, bins: i64)  -> Vec<T> {
    let mut sum = 0.0;
    let mut sum_squared = 0.0;
    let count = to_sort.len() as f64;

    for x in &to_sort {
        let val = x.get_numeric();
        sum += val;
        sum_squared += val.powi(2);
    }
    
    let mean = sum / count;
    let std_dev = (sum_squared / count - mean.powi(2)).sqrt();

    let prob_intervals = (1..bins).map(|x| (x as f64)/(bins as f64));
    let splits: Vec<f64> = prob_intervals.map(|x| distrs::Normal::ppf(x, mean, std_dev)).collect();
    let mut sort_into_bins: Vec<Vec<T>> = Vec::new();
    for _ in 0..bins {
        sort_into_bins.push(Vec::new());
    }

    for x in to_sort {
        let val = x.get_numeric();
        let i0 = binary_search(&splits, val);
        let bin: &mut Vec<T> = sort_into_bins.get_mut(i0).expect("Bin error");
        bin.push(x);
    }

    sort_into_bins.into_iter().flatten().collect()
}

fn binary_search(splits: &[f64], val: f64) -> usize {
    let mut start = 0;
    let mut end = splits.len();
    
    while start < end {
        let mid = (start + end) / 2;
        if val < splits[mid] {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    
    start
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use rand::Rng;

    use super::*;

    #[derive(Debug)]
    struct TestFloat {
        val: i64,
    }

    impl NumericEquivalent for TestFloat {
        fn get_numeric(&self) -> f64 {
            self.val as f64
        }
    }

    #[test]
    fn test_gauss() {
        //let mut rng = rand::thread_rng();
        //let to_sort: Vec<TestFloat> = (0..10000).into_iter().map(|_| TestFloat {val: rng.gen_range(0..50000)}).collect();
        //let mut to_sort_as_floats: Vec<i64> = to_sort.iter().map(|x| x.val).collect();
        //let gaussian_sort_timer = Instant::now();
        //let sorted = approximate_gaussian_sort(to_sort, 10000);
        //let gaussian_sort_time = gaussian_sort_timer.elapsed();
        //let standard_sort_timer = Instant::now();
        //to_sort_as_floats.sort();
        //let standard_sort_time = standard_sort_timer.elapsed();
        //println!("{:?}", sorted.iter().map(|x| x.val).collect::<Vec<i64>>());
        //println!("Standard sort time: {:?}, Gaussian sort time: {:?}", standard_sort_time, gaussian_sort_time);
    }

    #[test]
    fn test_box() {
        //let mut rng = rand::thread_rng();
        //let to_sort: Vec<TestFloat> = (0..10000).into_iter().map(|_| TestFloat {val: rng.gen_range(0..50000)}).collect();
        //let mut to_sort_as_floats: Vec<i64> = to_sort.iter().map(|x| x.val).collect();
        //let gaussian_sort_timer = Instant::now();
        //let sorted = approximate_box_sort(to_sort, 100);
        //let gaussian_sort_time = gaussian_sort_timer.elapsed();
        //let standard_sort_timer = Instant::now();
        //to_sort_as_floats.sort();
        //let standard_sort_time = standard_sort_timer.elapsed();
        //println!("{:?}", sorted.iter().map(|x| x.val).collect::<Vec<i64>>());
        //println!("Standard sort time: {:?}, Box sort time: {:?}", standard_sort_time, gaussian_sort_time);
    }

    #[test]
    fn test_trampoline() {
        let mut rng = rand::thread_rng();
        let to_sort: Vec<TestFloat> = (0..10000).into_iter().map(|_| TestFloat {val: rng.gen_range(0..50000)}).collect();
        let mut to_sort_as_floats: Vec<i64> = to_sort.iter().map(|x| x.val).collect();
        let gaussian_sort_timer = Instant::now();
        let sorted = trampoline_sort(to_sort);
        let gaussian_sort_time = gaussian_sort_timer.elapsed();
        let standard_sort_timer = Instant::now();
        to_sort_as_floats.sort();
        let standard_sort_time = standard_sort_timer.elapsed();
        println!("{:?}", sorted.iter().map(|x| x.val).collect::<Vec<i64>>());
        println!("Standard sort time: {:?}, Trampoline sort time: {:?}", standard_sort_time, gaussian_sort_time);
    }
}

fn main() {
    let meetings_sec_a: Vec<Vec<i32>> = vec![vec![13, 15], vec![15, 16], vec![7, 9]];
    let meetings_sec_b: Vec<Vec<i32>> = vec![vec![14, 15], vec![5, 10]];

    let intersection = overlapping_meetings(meetings_sec_a, meetings_sec_b);
    println!("The overlapping timings are {:?}", intersection);
}
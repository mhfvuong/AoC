use std::cmp::min;
use std::collections::HashMap;

pub fn distress_signal(data_string: String) {
    let mut pairs = Vec::new();
    let mut idx = 0;
    let mut sum_idx = 0;
    let lines = data_string.lines();
    for line in lines {
        if !line.is_empty() {pairs.push(line);}
    }
    for p in 0..(pairs.len() - 1) {
        if p % 2 == 0 {
            idx += 1;
            let p1 = pairs[p];
            let p2 = pairs[p + 1];
            println!("pair");
            println!("{p1}");
            println!("{p2}");
            if right_order(p1, p2) {
                println!("left is smaller than right: correct order (idx = {}) \n", &idx);
                sum_idx += idx;
            }
            else {println!("left is larger than right: incorrect order \n");}
        }
    }
    assert_ne!(sum_idx, 7298);
    println!("answer: {}", sum_idx);
}

//
fn right_order(left: &str, right: &str) -> bool {
    let mut min_length = left.as_bytes().len();
    if left.as_bytes().len() > right.as_bytes().len() {
        min_length = right.as_bytes().len();
    }
    let mut left_items = Vec::new();
    let mut right_items = Vec::new();
    for i in left.chars() {

    }
//     let mut left_list = 0;
//     let mut right_list = 0;
//     let mut shorter = &right_len;
//     if left_len < right_len {shorter = &left_len;}
//     let mut l_idx = 0;
//     let mut r_idx = 0;
//     for _ in 0..*min_length {
//         let left_value = match_packet(left, l_idx, &mut left_list);
//         let right_value = match_packet(right, r_idx, &mut right_list);
//         // println!("compare {} vs {}", &left_value, &right_value);
//         if left_value != -1 && right_value != -1 {
//             if left_value < right_value { return true; }
//             else if left_value > right_value { return false; }
//         }
//         else if left_value == -1 && right_value == -1 {
//             if left_list < right_list { return true; }
//             else if left_list > right_list { return false; }
//         }
//         else if (left_value != -1 && right_value == -1) || (left_value == -1 && right_value != -1) {
//             if left_value != -1 {
//                 l_idx -= 1;
//             }
//             if right_value != -1 {
//                 r_idx -= 1;
//             }
//         }
//         if left_list == 0 && right_list != 0 { return true; }
//         else if right_list == 0 && left_list != 0 { return false; }
//         l_idx += 1;
//         r_idx += 1;
//     }
    return false;
}
//
// fn match_packet(packet: &str, idx: usize, packet_list: &mut i32) -> i32{
//     match packet.chars().nth(idx).unwrap() {
//         '[' => *packet_list += 1,
//         ']' => *packet_list -= 1,
//         ',' => (),
//         number => return number.to_string().parse::<i32>().unwrap()
//     }
//     -1
// }
// // 7298 is incorrect
// pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
//     let mut result = Vec::new();

//     // Create ascending part (top half)
//     for level in 0..i {
//         // Create spaces
//         let spaces = " ".repeat((level + 1) as usize);
//         // Create repeated characters
//         let chars =v.repeat((level + 1) as usize);
//         // Combine spaces and characters
//         result.push(spaces + &chars);
//     }


//     // Create descending part (bottom half)
//     for level in (0..i-1).rev() {
//         // Create spaces
//         let spaces = " ".repeat((level + 1) as usize);
//         // Create repeated characters
//         let chars =v.repeat((level + 1) as usize);
//         // Combine spaces and characters
//         result.push(spaces + &chars);
//     }
//     result
// }

pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();

    // Create ascending part (top half)
    for level in 0..i
}
pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut pyramid = Vec::new();

    for j in 1..i {
        pyramid.push(format!("{}{}", " ".repeat(j as usize), v.repeat(j as usize)));
    }

    for j in (1..=i).rev() {
        pyramid.push(format!("{}{}", " ".repeat (j as usize), v.repeat(j as usize)));
    }
    pyramid
}
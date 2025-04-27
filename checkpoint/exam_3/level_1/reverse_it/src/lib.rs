pub fn reverse_it(i_nbr: i32) -> String {
    let original = i_nbr.to_string();
    let reversed: String = original
    .chars()
    .filter(|&c| c != '-') // to ignore '-' if its negative
    .rev()
    .collect();

    if i_nbr < 0 {
        format!("-{}{}", reversed, original.trim_start_matches('-'))
    } else {
        format!("{}{}", reversed, original)
    }
}
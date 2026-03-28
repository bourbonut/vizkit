pub(crate) fn split_colors(string: &str) -> Vec<&str> {
    let n = string.len() / 6;
    (0..n).map(|i| &string[i * 6..(i + 1) * 6]).collect()
}

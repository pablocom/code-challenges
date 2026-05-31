//! Length of the last whitespace-delimited word, ignoring trailing spaces.

pub fn solve(s: &str) -> usize {
    s.split_whitespace().next_back().map_or(0, str::len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve("Hello World"), 5);
        assert_eq!(solve("   fly me   to   the moon  "), 4);
        assert_eq!(solve("luffy is still joyboy"), 6);
        assert_eq!(solve("a"), 1);
        assert_eq!(solve("today"), 5);
        assert_eq!(solve("hello world  "), 5);
    }
}

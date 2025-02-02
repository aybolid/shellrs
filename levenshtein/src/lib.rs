pub struct Levenshtein;

impl Levenshtein {
    /// Compute the Levenshtein distance between two strings.
    pub fn distance(s: &str, t: &str) -> usize {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let s_len = s_chars.len();
        let t_len = t_chars.len();

        if s_len == 0 {
            return t_len;
        }
        if t_len == 0 {
            return s_len;
        }

        // dp[i] represents the cost of converting s[0..i] to an empty string.
        let mut dp: Vec<usize> = (0..=s_len).collect();

        for j in 1..=t_len {
            let mut prev = dp[0];
            dp[0] = j;

            for i in 1..=s_len {
                let temp = dp[i];

                if s_chars[i - 1] == t_chars[j - 1] {
                    dp[i] = prev;
                } else {
                    dp[i] = 1 + std::cmp::min(prev, std::cmp::min(dp[i - 1], dp[i]));
                }

                prev = temp;
            }
        }

        dp[s_len]
    }

    /// Given a string `s` and a slice of candidate strings `vars`,
    /// return the candidate with the smallest Levenshtein distance to `s`.
    /// Returns a reference to the candidate string.
    pub fn get_closest<'a>(s: &str, vars: &'a [String]) -> Option<&'a str> {
        let mut min_distance = usize::MAX;
        let mut closest: Option<&'a str> = None;

        for candidate in vars {
            let distance = Self::distance(s, candidate);
            if distance < min_distance {
                min_distance = distance;
                closest = Some(candidate);
            }
        }

        closest
    }

    /// Given a string `s`, a slice of candidate strings `vars`, and a maximum acceptable distance `threshold`,
    /// return the candidate with the smallest Levenshtein distance to `s` if that distance is less than or equal
    /// to `threshold`. Returns a reference to the candidate string.
    pub fn get_closest_with_threshold<'a>(
        s: &str,
        vars: &'a [String],
        threshold: usize,
    ) -> Option<&'a str> {
        let mut min_distance = usize::MAX;
        let mut closest: Option<&'a str> = None;

        for candidate in vars {
            let distance = Self::distance(s, candidate);
            if distance < min_distance {
                min_distance = distance;
                closest = Some(candidate);
            }
        }

        if min_distance <= threshold {
            closest
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Levenshtein;

    #[test]
    fn test_distance_empty_strings() {
        // Both strings empty.
        assert_eq!(Levenshtein::distance("", ""), 0);
    }

    #[test]
    fn test_distance_one_empty() {
        // One string is empty.
        assert_eq!(Levenshtein::distance("", "abc"), 3);
        assert_eq!(Levenshtein::distance("abc", ""), 3);
    }

    #[test]
    fn test_distance_identical_strings() {
        // Identical strings should have zero distance.
        let s = "rust";
        assert_eq!(Levenshtein::distance(s, s), 0);
    }

    #[test]
    fn test_distance_general() {
        // Test some known distances.
        // Example: "kitten" -> "sitting" has a distance of 3.
        assert_eq!(Levenshtein::distance("kitten", "sitting"), 3);
        // Another example: "flaw" -> "lawn" has a distance of 2.
        assert_eq!(Levenshtein::distance("flaw", "lawn"), 2);
    }

    #[test]
    fn test_distance_case_sensitive() {
        // The implementation is case sensitive.
        assert_eq!(Levenshtein::distance("Rust", "rust"), 1);
    }

    #[test]
    fn test_get_closest_empty_candidates() {
        // When candidate list is empty, should return None.
        let source = "hello";
        let candidates: Vec<String> = Vec::new();
        assert_eq!(Levenshtein::get_closest(source, &candidates), None);
    }

    #[test]
    fn test_get_closest_single_candidate() {
        // Single candidate should be returned.
        let source = "hello";
        let candidates = vec!["hallo".to_string()];
        assert_eq!(Levenshtein::get_closest(source, &candidates), Some("hallo"));
    }

    #[test]
    fn test_get_closest_multiple_candidates() {
        // Multiple candidates, check that the one with the smallest distance is returned.
        let source = "kitten";
        let candidates = vec![
            "sitting".to_string(), // distance 3
            "bitten".to_string(),  // distance 1
            "kitchen".to_string(), // distance 2
        ];
        // "bitten" is the closest.
        assert_eq!(
            Levenshtein::get_closest(source, &candidates),
            Some("bitten")
        );
    }

    #[test]
    fn test_get_closest_tie_breaker() {
        // In case of a tie, the first encountered candidate with the minimum distance is chosen.
        let source = "abc";
        let candidates = vec![
            "abd".to_string(), // distance 1
            "abx".to_string(), // distance 1
        ];
        assert_eq!(Levenshtein::get_closest(source, &candidates), Some("abd"));
    }

    #[test]
    fn test_get_closest_with_threshold_success() {
        // Test get_closest_with_threshold where a candidate is within the threshold.
        let source = "kitten";
        let candidates = vec![
            "sitting".to_string(), // distance 3
            "bitten".to_string(),  // distance 1
            "kitchen".to_string(), // distance 2
        ];
        // With threshold 2, "bitten" (distance 1) qualifies.
        assert_eq!(
            Levenshtein::get_closest_with_threshold(source, &candidates, 2),
            Some("bitten")
        );
    }

    #[test]
    fn test_get_closest_with_threshold_fail() {
        // Test get_closest_with_threshold where no candidate is within the threshold.
        let source = "kitten";
        let candidates = vec![
            "sitting".to_string(), // distance 3
            "kitchen".to_string(), // distance 2
        ];
        // With threshold 1, no candidate qualifies.
        assert_eq!(
            Levenshtein::get_closest_with_threshold(source, &candidates, 1),
            None
        );
    }
}

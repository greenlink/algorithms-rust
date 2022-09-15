fn linear_search(haystack: Vec<u8>, needle: u8) -> bool {
    let _i = 0;
    for item in haystack {
        if item == needle {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod linear_search_tests {
    use super::*;

    #[test]
    fn test_if_find_value() {
        let haystack:Vec<u8> = (0..10).collect();
        let needle:u8 = 5;
        let expected = true;

        let result = linear_search(haystack, needle);

        assert_eq!(result, expected)
    }

    #[test]
    fn test_if_dont_find_value() {
        let haystack:Vec<u8> = (0..10).collect();
        let needle:u8 = 11;
        let expected = false;

        let result = linear_search(haystack, needle);

        assert_eq!(result, expected)
    }
}

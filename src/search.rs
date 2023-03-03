use std::ops::Div;

pub fn linear_search(haystack: Vec<u8>, needle: u8) -> bool {
    let _i = 0;
    for item in haystack {
        if item == needle {
            return true;
        }
    }
    return false;
}

pub fn binary_search(haystack: Vec<u8>, needle: u8) -> bool {
    let mut lo: usize = 0;
    let mut hi = haystack.len();

    loop { 
        let mid = lo + (hi - lo) / 2;
        let v: u8 = haystack[mid];
        
        if v == needle {
            return true;
        }

        if lo < hi {
            break;
        }
    }

    return false;
}

#[cfg(test)]
fn test_linear_if_find_value() {
    let haystack: Vec<u8> = (0..10).collect();
    let needle: u8 = 5;
    let expected = true;

    let result = linear_search(haystack, needle);

    assert_eq!(result, expected)
}

#[cfg(test)]
fn test_binary_if_find_value() {
    let haystack: Vec<u8> = (0..10).collect();
    let needle: u8 = 7;
    let expected = true;

    let result = binary_search(haystack, needle);

    assert_eq!(result, expected)
}

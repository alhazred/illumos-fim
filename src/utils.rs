// Copyright (C) 2021, Achiefs.
// Copyright 2022 Tintri by DDN, Inc. All rights reserved.

// To manage unique event identifier
use uuid::Uuid;
// To get Operating system
use std::env;
use std::path::Path;
// To manage cmp order
use std::cmp::Ordering;

// ----------------------------------------------------------------------------

// Function to pop last char of a given String
pub fn pop(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next_back();
    chars.as_str()
}
// ----------------------------------------------------------------------------

pub fn get_uuid() -> String {
    format!("{}", Uuid::new_v4())
}

// ----------------------------------------------------------------------------


use std::os::unix::fs::PermissionsExt;
use std::fs::metadata;

pub fn get_path(path: &Path) -> String {
    format!("{}", path.display().to_string())
}

pub fn get_perms(path: &Path) -> String {
    format!("{:o}", metadata(path).unwrap().permissions().mode())
}

pub fn get_size(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().len())
}
use std::os::unix::fs::MetadataExt;

pub fn get_uid(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().uid())
}

pub fn get_gid(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().gid())
}

pub fn get_mtime(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().mtime())
}
pub fn get_atime(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().atime())
}
pub fn get_ctime(path: &Path) -> String {
    format!("{}", metadata(path).unwrap().ctime())
}

// ----------------------------------------------------------------------------

pub fn get_os() -> String {
    env::consts::OS.to_string()
}

// ----------------------------------------------------------------------------



// Function to clean trailing slash of a path
pub fn clean_path(path: &str) -> String {
    String::from(if path.ends_with('/') || path.ends_with('\\'){ pop(path) }else{ path })
}

// ----------------------------------------------------------------------------

// Returns if raw_path contains compare_path
pub fn match_path(raw_path: &str, compare_path: &str) -> bool {
    let pattern = "/";
    let mut raw_tokens: Vec<&str> = raw_path.split(pattern).collect();
    let mut compare_tokens: Vec<&str> = compare_path.split(pattern).collect();

    match raw_tokens.len().cmp(&compare_tokens.len()) {
        Ordering::Equal => {
            raw_tokens.iter().zip(compare_tokens.iter()).all(|(r,c)|
                clean_path(r) == clean_path(c))
        },
        Ordering::Greater => {
            // Removing file name from bottom
            raw_tokens.pop();
            raw_tokens.iter().zip(compare_tokens.iter()).all(|(r,c)|
                clean_path(r) == clean_path(c))
        },
        _ => {
            // Removing file name from bottom
            compare_tokens.pop();
            raw_tokens.iter().zip(compare_tokens.iter()).all(|(r,c)|
                clean_path(r) == clean_path(c))
        }
    }
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop() {
        assert_eq!(pop("test-"), "test");
        assert_eq!(pop("dir/"), "dir");
        assert_eq!(pop("dir@"), "dir");
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_uuid() {
        // 9bd52d8c-e162-4f4d-ab35-32206d6d1445
        let uuid = get_uuid();
        let uuid_vec: Vec<&str> = uuid.split("-").collect();
        assert_eq!(uuid.len(), 36);
        assert_eq!(uuid_vec.len(), 5);
        assert_eq!(uuid_vec[0].len(), 8);
        assert_eq!(uuid_vec[1].len(), 4);
        assert_eq!(uuid_vec[2].len(), 4);
        assert_eq!(uuid_vec[3].len(), 4);
        assert_eq!(uuid_vec[4].len(), 12);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_os() {
        assert_eq!(get_os(), env::consts::OS.to_string());
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_match_path() {
        if get_os() == "illumos" {
            assert!(match_path("/", "/"));
            assert!(match_path("/test", "/test"));
            assert!(match_path("/test/", "/test"));
            assert!(match_path("/test/tmp", "/test"));
            assert!(!match_path("/tmp/test", "/test"));
            assert!(!match_path("/tmp", "/test"));
        }
    }

}

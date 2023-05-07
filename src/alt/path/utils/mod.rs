pub fn cleanse_path(path: &str) -> String {
    let s = path.to_string();
    if s.len() > 1 && s[0..2].to_string() == "./" {
        s[2..].to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::cleanse_path;

    #[test]
    fn cleanse_path_returns_path_with_dot_slash_prefix_stripped() {
        assert_eq!("hoopty/doopty.thing", cleanse_path("./hoopty/doopty.thing"));
    }

    #[test]
    fn cleanse_path_does_not_effect_non_dot_slash_prefixes() {
        assert_eq!(
            "foo/hoopty/doopty.thing",
            cleanse_path("foo/hoopty/doopty.thing")
        );
    }
}

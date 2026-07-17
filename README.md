# serde_versions
Небольшая библиотека для хранения и (де-)сериализации структуры описывающей версии в формат serde.

## Как использовать
```cargo add --git https://github.com/egevtech/serde_versions```

## Пример кода
```rust
    use serde_versions::Version;
    use serde::Deserialize;
    use serde_json;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        version: Version,
    }

    fn main() {
        let json_str = r#"
            {
                "version": {
                    "major": 1,
                    "minor": 0,
                    "patch": 0
                }
            }
            "#;

        let dt: Data = match serde_json::from_str(json_str) {
            Ok(dt) => dt,
            Err(err) => panic!("Failed to unwrap json: {}", err),
        };

        assert_eq!(
            dt,
            Data {
                version: Version::new(1, 0, 0)
            }
        )
    }
```
```rust
let ver1 = ver!(3.5.1);
let ver2 = ver!(6.5.6);

assert!(ver1 < ver2);
```
```rust
let ver1 = ver!(1.5.6);
let ver2 = ver!(2.5.3);
let ver3 = ver!(1.5.6);

assert_eq!(ver1, ver3);
assert_ne!(ver1, ver2);
```
```rust 
let ver = ver!(7.6.5);
let str = "7.6.5";

assert_eq!(Version::from_str(str), ver);
assert_eq!(&ver.to_string(), str);
```

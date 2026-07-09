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

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use std::{collections::BTreeMap, str::FromStr};

    use envoke::{Envoke, Fill};
    use secrecy::Secret;

    #[test]
    fn test_partial_initialization() {
        #[derive(Fill)]
        struct Test {
            #[fill(env)]
            field1: String,

            #[fill(env, default)]
            field2: i32,
        }

        temp_env::with_vars([("field1", Some("value")), ("field2", Some("10"))], || {
            let test = Test {
                field2: 20,
                ..Envoke::envoke()
            };

            assert_eq!(test.field1, "value".to_string());
            assert_eq!(test.field2, 20);
        });
    }

    #[test]
    fn test_load_env_field_name() {
        #[derive(Fill)]
        struct Test {
            #[fill(env)]
            field: String,
        }

        temp_env::with_var("field", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, "value".to_string())
        });
    }

    #[test]
    fn test_load_env_specified_name() {
        #[derive(Fill)]
        struct Test {
            #[fill(env = "TEST_ENV")]
            field: String,
        }

        temp_env::with_var("TEST_ENV", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, "value".to_string())
        });
    }

    #[test]
    fn test_load_env_multiple_names() {
        #[derive(Fill)]
        struct Test {
            #[fill(env, env = "ENV1", env = "ENV2", default = "test")]
            field: Option<String>,
        }

        temp_env::with_var("ENV1", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, Some("value".to_string()))
        });
    }

    #[test]
    fn test_load_env_default_fallback() {
        #[derive(Fill)]
        struct Test {
            #[fill(env, default)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 0); // i32 default
    }

    #[test]
    fn test_load_env_default_t_fallback() {
        #[derive(Debug, PartialEq, strum::EnumString)]
        enum Tes {
            Enum1,
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env, default = Tes::Enum1)]
            field: Tes,
        }

        let test = Test::envoke();
        assert_eq!(test.field, Tes::Enum1);
    }

    #[test]
    fn test_load_env_default_fn_fallback() {
        fn default_i32() -> i32 {
            10
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env, default = default_i32())]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 10);
    }

    #[test]
    fn test_load_only_default() {
        #[derive(Fill)]
        struct Test {
            #[fill(env, default)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 0); // i32 default
    }

    #[test]
    fn test_load_only_default_t() {
        #[derive(Fill)]
        struct Test {
            #[fill(env, default = 10)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 10);
    }

    #[test]
    fn test_load_only_default_fn() {
        fn default_i32(add: i32) -> i32 {
            10 + add
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env, default = default_i32(10))]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 20);
    }

    #[test]
    fn test_load_env_not_found() {
        #[derive(Fill)]
        struct Test {
            #[fill(env)]
            field: i32,
        }

        let test = Test::try_envoke();
        assert!(test.is_err());
        assert!(test.err().is_some_and(|e| e.is_retrieve_error()))
    }

    #[test]
    fn test_load_env_and_parse() {
        use std::time::Duration;

        fn to_time(sec: u64) -> Duration {
            Duration::from_secs(sec)
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env = "TEST_ENV", parse_fn = to_time, arg_type = u64)]
            field: Duration,
        }

        temp_env::with_var("TEST_ENV", Some("10"), || {
            let test = Test::envoke();
            assert_eq!(test.field, Duration::from_secs(10));
        });
    }

    #[test]
    fn test_load_env_and_validate_before() {
        use std::time::Duration;

        fn above_zero(secs: &u64) -> std::result::Result<(), String> {
            match *secs > 0 {
                true => Ok(()),
                false => Err("duration cannot be 0".to_string()),
            }
        }

        fn to_time(secs: u64) -> Duration {
            Duration::from_secs(secs)
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env = "TEST_ENV", parse_fn = to_time, arg_type = u64, validate_fn(before = above_zero))]
            field: Duration,
        }

        temp_env::with_var("TEST_ENV", Some("10"), || {
            let test = Test::envoke();
            assert_eq!(test.field, Duration::from_secs(10));
        });
    }

    #[test]
    fn test_load_env_and_validate_after() {
        use envoke::{Envoke, Fill};

        fn more_than_ten_opt(amount: &Option<u64>) -> std::result::Result<(), String> {
            if let Some(amount) = amount {
                if *amount < 10 {
                    return Err("amount should be more than 10".to_string());
                }
            }

            Ok(())
        }

        fn more_than_ten(amount: &u64) -> std::result::Result<(), String> {
            match *amount > 10 {
                true => Ok(()),
                false => Err("amount should be more than 10".to_string()),
            }
        }

        fn add_ten_opt(amount: Option<u64>) -> Option<u64> {
            amount.and_then(|x| Some(x + 10))
        }

        fn add_ten(amount: u64) -> u64 {
            amount + 10
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env = "TEST_ENV", parse_fn = add_ten_opt, arg_type = Option<u64>, validate_fn(after = more_than_ten_opt))]
            field1: Option<u64>,

            #[fill(env = "TEST_ENV", parse_fn = add_ten, arg_type = u64, validate_fn = more_than_ten)]
            field2: u64,
        }

        temp_env::with_var("TEST_ENV", Some("5"), || {
            let test = Test::envoke();
            assert_eq!(test.field1, Some(15));
            assert_eq!(test.field2, 15);
        });
    }

    #[test]
    fn test_load_env_and_validate_before_and_after() {
        fn less_than_ten(amount: &u64) -> std::result::Result<(), String> {
            match *amount < 10 {
                true => Ok(()),
                false => Err("amount should be less than 10".to_string()),
            }
        }

        fn more_than_ten(amount: &u64) -> std::result::Result<(), String> {
            match *amount > 10 {
                true => Ok(()),
                false => Err("amount should be more than 10".to_string()),
            }
        }

        fn add_ten(amount: u64) -> u64 {
            amount + 10
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env = "TEST_ENV", parse_fn = add_ten, arg_type = u64, validate_fn(before = less_than_ten, after = more_than_ten))]
            field: u64,
        }

        temp_env::with_var("TEST_ENV", Some("5"), || {
            let test = Test::envoke();
            assert_eq!(test.field, 15);
        });
    }

    #[test]
    fn test_load_env_with_prefix_and_suffix() {
        #[derive(Fill)]
        #[fill(prefix = "PREFIX", suffix = "SUFFIX", delimiter = "_")]
        struct Test {
            #[fill(env = "TEST_ENV")]
            field: String,
        }

        temp_env::with_var("PREFIX_TEST_ENV_SUFFIX", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, "value".to_string())
        });
    }

    #[test]
    fn test_load_env_override_prefix_and_suffix() {
        #[derive(Fill)]
        #[fill(prefix = "PREFIX", suffix = "SUFFIX", delimiter = "_")]
        struct Test {
            #[fill(env = "TEST_ENV", no_prefix, no_suffix)]
            field: String,
        }

        temp_env::with_var("TEST_ENV", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, "value".to_string())
        });
    }

    #[test]
    fn test_load_env_nested_structs() {
        #[derive(Fill)]
        struct TestInnerInner {
            #[fill(env = "TEST_ENV", no_prefix, no_suffix)]
            field: String,
        }

        #[derive(Fill)]
        struct TestInner {
            #[fill(nested)]
            inner: TestInnerInner,
        }

        #[derive(Fill)]
        struct Test {
            #[fill(nested)]
            inner: TestInner,
        }

        temp_env::with_var("TEST_ENV", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.inner.inner.field, "value".to_string())
        });
    }

    #[test]
    fn test_load_env_map_and_set() {
        use std::{
            collections::{BTreeSet, HashMap, HashSet},
            time::Duration,
        };

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::EnumString)]
        #[strum(serialize_all = "lowercase")]
        enum TestEnum {
            Enum1,
            Enum2,
            Enum3,
        }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        enum Value {
            Number(i64),
            String(String),
        }

        impl FromStr for Value {
            type Err = envoke::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Ok(num) = s.parse::<i64>() {
                    Ok(Value::Number(num))
                } else {
                    Ok(Value::String(s.to_string()))
                }
            }
        }

        fn to_time(secs: Vec<u64>) -> Vec<Duration> {
            secs.into_iter().map(Duration::from_secs).collect()
        }

        #[derive(Debug, Fill)]
        struct Test {
            // Test HashMap with default delimiter (,)
            #[fill(env = "TEST_HMAP_1")]
            hmap1: HashMap<String, String>,

            // Test HashMap with custom delimiter (;)
            #[fill(env = "TEST_HMAP_2", delimiter = ";")]
            hmap2: HashMap<String, i32>,

            // Test BTreeMap with default delimiter (,)
            #[fill(env = "TEST_BMAP_1")]
            bmap1: BTreeMap<String, String>,

            // Test BTreeMap with custom delimiter (&) and enum parsing
            #[fill(env = "TEST_BMAP_2", delimiter = "&")]
            bmap2: BTreeMap<String, TestEnum>,

            // Test HashSet with default delimiter (,)
            #[fill(env = "TEST_HSET_1", default = HashSet::from([1, 2, 3]))]
            hset1: HashSet<i32>,

            // Test HashSet with custom delimiter (|)
            #[fill(env = "TEST_HSET_2", delimiter = "|")]
            hset2: HashSet<String>,

            // Test BTreeSet with default delimiter (,)
            #[fill(env = "TEST_BSET_1")]
            bset1: BTreeSet<TestEnum>,

            // Test BTreeSet with custom delimiter (!)
            #[fill(env = "TEST_BSET_2", delimiter = "!")]
            bset2: BTreeSet<Value>,

            // Test Vec with default delimiter (,)
            #[fill(env = "TEST_VEC_1")]
            vec1: Vec<bool>,

            // Test Vec with custom delimiter (-) and custom parse_fn
            #[fill(env = "TEST_VEC_2", delimiter = "-", parse_fn = to_time, arg_type = Vec<u64>)]
            vec2: Vec<Duration>,
        }

        // Test loading of HashMap, HashSet, and Vec from environment variables
        temp_env::with_vars(
            [
                ("TEST_HMAP_1", Some("key1=value1,key2=value2")),
                ("TEST_HMAP_2", Some("key1=1;key2=2;key3=3")),
                ("TEST_BMAP_1", Some("key1=value1,key2=value2")),
                ("TEST_BMAP_2", Some("key1=enum1&key2=enum2")),
                ("TEST_HSET_2", Some("value1|value2|value3")),
                ("TEST_BSET_1", Some("enum2,enum1")),
                ("TEST_BSET_2", Some("1!2!foo!4!bar")),
                ("TEST_VEC_1", Some("true,false,true")),
                ("TEST_VEC_2", Some("1-2-3")),
            ],
            || {
                let test = Test::envoke();
                println!("{test:#?}");

                assert_eq!(test.hmap1.len(), 2);
                assert_eq!(
                    test.hmap1,
                    HashMap::from([
                        ("key1".to_string(), "value1".to_string()),
                        ("key2".to_string(), "value2".to_string())
                    ])
                );

                assert_eq!(test.hmap1.len(), 2);
                assert_eq!(
                    test.hmap1,
                    HashMap::from([
                        ("key1".to_string(), "value1".to_string()),
                        ("key2".to_string(), "value2".to_string())
                    ])
                );

                assert_eq!(test.bmap1.len(), 2);
                assert_eq!(
                    test.bmap1,
                    BTreeMap::from([
                        ("key1".to_string(), "value1".to_string()),
                        ("key2".to_string(), "value2".to_string())
                    ])
                );

                assert_eq!(test.bmap2.len(), 2);
                assert_eq!(
                    test.bmap2,
                    BTreeMap::from([
                        ("key1".to_string(), TestEnum::Enum1),
                        ("key2".to_string(), TestEnum::Enum2)
                    ])
                );

                assert_eq!(test.hset1.len(), 3);
                assert_eq!(test.hset1, HashSet::from([1, 2, 3]));

                assert_eq!(test.hset2.len(), 3);
                assert_eq!(
                    test.hset2,
                    HashSet::from([
                        "value1".to_string(),
                        "value2".to_string(),
                        "value3".to_string()
                    ])
                );

                assert_eq!(test.bset1.len(), 2);
                assert_eq!(
                    test.bset1,
                    BTreeSet::from([TestEnum::Enum1, TestEnum::Enum2])
                );

                assert_eq!(test.bset2.len(), 5);

                let expected = BTreeSet::from([
                    Value::Number(1),
                    Value::Number(2),
                    Value::String("foo".to_string()),
                    Value::Number(4),
                    Value::String("bar".to_string()),
                ]);
                assert!(expected.iter().all(|e| test.bset2.contains(e)));

                assert_eq!(test.vec1.len(), 3);
                assert_eq!(test.vec1, vec![true, false, true]);

                assert_eq!(test.vec2.len(), 3);
                assert_eq!(
                    test.vec2,
                    vec![
                        Duration::from_secs(1),
                        Duration::from_secs(2),
                        Duration::from_secs(3)
                    ]
                );
            },
        );
    }

    #[test]
    fn test_load_env_rename_env() {
        #[derive(Fill)]
        #[fill(rename_all = "SCREAMING_SNAKE_CASE")]
        struct Test {
            #[fill(env)]
            field1: i32,
        }

        temp_env::with_var("FIELD_1", Some("42"), || {
            let test = Test::envoke();
            assert_eq!(test.field1, 42)
        });
    }

    #[test]
    fn test_load_env_correct_order() {
        #[derive(Fill)]
        #[fill(rename_all = "UPPERCASE")]
        struct Test {
            #[fill(env, env = "ENV1", env = "ENV2")]
            field: String,
        }

        temp_env::with_vars(
            [
                ("ENV1", Some("value2")),
                ("ENV2", Some("value3")),
                ("FIELD", Some("value1")),
            ],
            || {
                let test = Test::envoke();
                assert_eq!(test.field, "value1".to_string())
            },
        );

        temp_env::with_vars([("ENV1", Some("value2")), ("ENV2", Some("value3"))], || {
            let test = Test::envoke();
            assert_eq!(test.field, "value2".to_string())
        });
    }

    #[test]
    fn test_secret_wrapper() {
        #[derive(Fill)]
        struct Test {
            #[fill(env, env = "ENV1", env = "ENV2")]
            field: Secret<String>,
        }
    }
}

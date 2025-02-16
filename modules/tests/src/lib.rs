#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use envoke::{Envoke, Fill};

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
            #[fill(env, env = "ENV1", env = "ENV2")]
            field: String,
        }

        temp_env::with_var("ENV1", Some("value"), || {
            let test = Test::envoke();
            assert_eq!(test.field, "value".to_string())
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
        #[derive(Fill)]
        struct Test {
            #[fill(env, default_t = 10)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 10);
    }

    #[test]
    fn test_load_env_default_fn_fallback() {
        fn default_i32() -> i32 {
            10
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env, default_fn = default_i32)]
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
            #[fill(env, default_t = 10)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 10);
    }

    #[test]
    fn test_load_only_default_fn() {
        fn default_i32() -> i32 {
            10
        }

        #[derive(Fill)]
        struct Test {
            #[fill(env, default_fn = default_i32)]
            field: i32,
        }

        let test = Test::envoke();
        assert_eq!(test.field, 10);
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
            String(String),
            Number(i64),
        }

        // A custom parsing function to convert a vector of u64 into a vector of
        // Duration
        fn to_time(secs: Vec<u64>) -> Vec<Duration> {
            secs.into_iter().map(Duration::from_secs).collect()
        }

        // Struct that will be filled with environment variables
        #[derive(Debug, Fill)]
        struct Test {
            // Test HashMap with default delimiter (,)
            #[fill(env = "TEST_HMAP_1")]
            hmap1: HashMap<String, String>,

            // Test HashMap with custom delimiter (;)
            #[fill(env = "TEST_HMAP_2", delimiter = ";")]
            hmap2: HashMap<String, i32>,

            // Test HashMap with default delimiter (,)
            #[fill(env = "TEST_BMAP_1")]
            bmap1: HashMap<String, String>,

            // Test HashMap with custom delimiter (&) and enum parsing
            #[fill(env = "TEST_BMAP_2", delimiter = "&")]
            bmap2: HashMap<String, TestEnum>,

            // Test HashSet with default delimiter (,)
            #[fill(env = "TEST_HSET_1")]
            hset1: HashSet<i32>,

            // Test HashSet with custom delimiter (|)
            #[fill(env = "TEST_HSET_2", delimiter = "|")]
            hset2: HashSet<String>,

            // Test HashSet with default delimiter (,)
            #[fill(env = "TEST_BSET_1")]
            bset1: BTreeSet<TestEnum>,

            // Test HashSet with custom delimiter (!)
            #[fill(env = "TEST_BSET_2", delimiter = "!")]
            bset2: BTreeSet<String>,

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
                ("TEST_HSET_1", Some("1,2,3")),
                ("TEST_HSET_2", Some("apple|banana|cherry")),
                ("TEST_BSET_1", Some("enum2,enum1")),
                ("TEST_BSET_2", Some("1!2!foo!4!bar")),
                ("TEST_VEC_1", Some("true,false,true")),
                ("TEST_VEC_2", Some("1-2-3")),
            ],
            || {
                let test = Test::envoke();

                println!("{test:#?}");

                // Test HashMap loading and assertions
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
}

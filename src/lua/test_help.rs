/// The `test_lua!()` macro should be invoked at the start of a
/// #[test] function to setup the Lua state and create the following
/// Lua-specific assertions macros:
///
/// * `assert_lua_bool!("true", true);`
/// * `assert_lua_string!("'Yes'", "Yes");`
/// * `assert_lua!(usize, "123", 123);`
///
/// It also adds two macros for setting and getting globals:
///
/// * `let var: TYPE = global!("key");`
/// * `set_global!("key", "value");`
///
/// `test_lua!()` can optionally be called with "key" => value pairs
/// as a convenience to set globals right away.
macro_rules! test_lua {
    // test_lua!(key => value);
    ($($key:literal => $val:expr),+) => {
        test_lua!();
        $( set_global!($key, $val); )+
    };

    // Allow trailing comma.
    ($($key:literal => $val:expr,)+) => { test_lua!($($key => $val),+) };

    () => {
        let state = mlua::Lua::new();

        #[allow(unused_macros)]
        macro_rules! run_lua {
            ($lua_code:literal) => {
                state.load($lua_code).call::<_,()>(()).unwrap();
            };
        }

        #[allow(unused_macros)]
        macro_rules! assert_lua {
            ($return_type:ty, $lua_code:literal, $expect:expr) => {
                assert_eq!(
                    state.load(concat!("return ", $lua_code)).call::<_, $return_type>(()).unwrap(),
                    $expect
                );
            };
        }

        #[allow(unused_macros)]
        macro_rules! assert_lua_bool {
            ($lua_code:literal, $expect:expr) => {
                assert_lua!(bool, $lua_code, $expect)
            };
        }

        #[allow(unused_macros)]
        macro_rules! assert_lua_string {
            ($lua_code:literal, $expect:expr) => {
                assert_lua!(String, $lua_code, $expect)
            };
        }

        #[allow(unused_macros)]
        macro_rules! global {
            ($key:literal) => {
                state.globals().get($key).unwrap()
            };
        }

        #[allow(unused_macros)]
        macro_rules! set_global {
            ($key:literal, $val:expr) => {
                    state.globals().set($key, $val).unwrap();
            };
        }

        #[allow(unused_macros)]
        macro_rules! register {
            ($key:literal) => {
                state.named_registry_value($key).unwrap();
            };
        }

        #[allow(unused_macros)]
        macro_rules! set_register {
            ($key:literal, $val:expr) => {
                state.set_named_registry_value($key, $val).unwrap();
            };
        }
    };
}

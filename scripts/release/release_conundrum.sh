set -e

cargo publish -p conundrum_lang -p conundrum_cli -p conundrum_config -p conundrum_db -p conundrum_fs -p conundrum_macro_traits -p conundrum_macros -p conundrum_swift -p conundrum_server_rs -p conundrum_tools -p conundrum_wasm -p fluster_core_utilities --dry-run

cd $FLUSTER_IOS_ROOT/packages/rust/conundrum_server_rs/

cargo add rmcp

cargo add rig --features rmcp

cargo fetch

cargo build

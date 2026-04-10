cd $FLUSTER_IOS_ROOT/packages/rust/conundrum
cargo add tsify
cargo add tsify --features js
cargo doc -p tsify --open

// Doing this as a macro for two reasons, one is clearly more important than
// the other.
//
// 1. To enable a more reliable wasm flag later when I have access to wifi and
//    can google it.
// 2. Because I _finally_ had some time to dedicate to learning macros, and I'm
//    pumped about it.
// macro_rules! wasm_only {
//     ($($code:tt)*) => {
//         #[cfg(not(target_arch = "wasm32"))]
//         $($code)*
//     };
// }

// wasm_only! {
//     let y = Vec::new();
// }

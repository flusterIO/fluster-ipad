pub mod wasm_not_wasm_macro {
    macro_rules! wasm_not_wasm {
    (
        wasm { $($wasm_code:tt)* }
        native { $($native_code:tt)* }
    ) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $($wasm_code)*
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            $($native_code)*
        }
    }};
}

    pub(crate) use wasm_not_wasm; // <-- the trick
}

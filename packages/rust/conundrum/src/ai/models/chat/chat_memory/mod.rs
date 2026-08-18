use rig_memory::ConversationMemory;

pub struct ChatMemory {}


impl ConversationMemory for ChatMemory {
    fn load<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, Result<Vec<rig::prelude::Message>, rig_memory::MemoryError>> {
        todo!()
    }

    fn append<'a>(
        &'a self,
        conversation_id: &'a str,
        messages: Vec<rig::prelude::Message>,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, Result<(), rig_memory::MemoryError>> {
        todo!()
    }

    fn clear<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> rig::wasm_compat::WasmBoxedFuture<'a, Result<(), rig_memory::MemoryError>> {
        todo!()
    }
}

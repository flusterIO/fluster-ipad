use crate::output::html::dom::dom_id::DOMId;

#[derive(Debug, Default, Clone)]
pub struct DomData {
    pub id_count: u64,
}

impl DomData {
    pub fn new_id(&mut self) -> DOMId {
        let id = format!("cdrm-{}", self.id_count);
        self.id_count += 1;
        DOMId::new(id)
    }
}

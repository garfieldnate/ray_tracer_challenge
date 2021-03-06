use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

// Shapes are globally unique. We use IDs to simplify their comparison.
static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn get_next_unique_shape_id() -> usize {
    OBJECT_COUNTER.fetch_add(1, Ordering::SeqCst)
}
#[derive(PartialEq)]
pub struct ObjectId {
    id: usize,
}
impl Default for ObjectId {
    fn default() -> Self {
        Self {
            id: get_next_unique_shape_id(),
        }
    }
}
impl Debug for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}
// Cloned ID's are also unique. The Clone trait is implemented to make it easier to auto-derive
// Clone in other structs that contain ObjectId's.
impl Clone for ObjectId {
    fn clone(&self) -> Self {
        Default::default()
    }
}
impl ObjectId {
    pub fn get_id(&self) -> usize {
        self.id
    }
}

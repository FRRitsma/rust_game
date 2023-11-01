// TODO: Implement shoots, hits, gets hit

pub trait SpawnEntity {
    fn spawn<T>(&self, entity_vec: &mut Vec<T>);
}

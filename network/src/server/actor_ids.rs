use std::num::NonZeroI16;

pub type ActorId = NonZeroI16;
pub type ActorIndex = usize;

pub struct ActorIds {
    ids: Vec<Option<ActorId>>,
    id_generator: i16,
    capacity: i16,
}

impl ActorIds {
    pub fn new(capacity: i16) -> ActorIds {
        ActorIds {
            ids: Vec::<Option<ActorId>>::with_capacity(capacity as usize),
            id_generator: 0,
            capacity,
        }
    }

    pub fn find_index(&self, id: ActorId) -> Option<ActorIndex> {
        self.ids.iter().position(|&value| value == Some(id))
    }

    pub fn add(&mut self) -> Option<(ActorId, ActorIndex)> {
        if self.ids.len() == self.capacity as usize {
            return None;
        }

        while {
            self.id_generator += 1;
            NonZeroI16::new(self.id_generator)
        }
        .is_none()
        {}

        let new_id = NonZeroI16::new(self.id_generator);
        self.ids.push(new_id);
        let index = self.ids.len() - 1;
        Some((new_id.unwrap(), index))
    }

    pub fn remove(&mut self, id: ActorId) -> Option<ActorIndex> {
        if let Some(index) = self.find_index(id) {
            self.ids.swap_remove(index);
            return Some(index);
        }
        None
    }
    pub fn count(&self) -> i16 {
        self.ids.len() as i16
    }
}

#[cfg(test)]
mod tests {
    use super::ActorIds;

    #[test]
    fn ids() {
        let mut list = ActorIds::new(4);
        assert_eq!(list.count(), 0);
        let id = list.add();
        assert!(id.is_some());
        assert_eq!(list.count(), 1);
        list.remove(id.unwrap().0);
        assert_eq!(list.count(), 0);

        for i in 0..4 {
            let id = list.add();
            assert!(id.is_some());
            let id = id.unwrap();
            let index = list.find_index(id.0);
            assert!(index.is_some());
            assert_eq!(id.1, index.unwrap());
            assert_eq!(list.count(), i + 1)
        }

        let id = list.add();
        assert!(id.is_none());

        assert_eq!(list.count(), 4);
    }
}

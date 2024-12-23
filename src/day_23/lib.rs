use std::collections::HashSet;

use std::hash::{Hash,Hasher};

type Computer = String;
pub struct NetworkSet(pub HashSet<Computer>);

impl PartialEq for NetworkSet {
  fn eq(&self, other: &NetworkSet) -> bool {
    self.0.is_subset(&other.0) && other.0.is_subset(&self.0) 
  }
}

impl Eq for NetworkSet {}

impl Hash for NetworkSet {
  fn hash<H>(&self, computer: &mut H) where H: Hasher {
    let mut a: Vec<&Computer> = self.0.iter().collect();
    a.sort();
    for s in a.iter() {
      s.hash(computer);
    }
  }
}

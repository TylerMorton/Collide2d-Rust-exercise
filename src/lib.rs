pub mod collision_physics;
pub mod person;

pub mod core {
    pub use crate::person::Person;
    use std::collections::HashMap;
    use crate::collision_physics;

    // returns indexes of reference people from vector
    pub fn naive_have_collided(crowd: &Vec<Person>) -> Vec<usize> {
        let mut collisions: HashMap<u32, usize> = HashMap::new();
        for (idx, i) in crowd.iter().enumerate() {
            for j in crowd {
                if j == i {
                    continue;
                }
                if i.peer_collision(j) {
                    collisions.entry(i.id()).or_insert(idx);
                }
            }
        }
        collisions.into_values().collect()
    }

    pub fn collide(crowd: &mut Vec<&mut Person>) -> Vec<[u32;2]> {
        let mut collisions: HashMap<Vec<usize>, usize> = HashMap::new();
        for (idx1, i) in crowd.iter().enumerate() {
            for (idx2, j) in crowd.iter().enumerate() {
                if j == i {
                    continue;
                }
                if i.peer_collision(j) {
                    let mut vec = vec![idx1, idx2];
                    vec.sort();
                    collisions.entry(vec).or_insert(0);
                }
            }
        }
        let values: Vec<Vec<usize>> = collisions.into_keys().collect();
        for val in values.iter() {
            let p1: &Person = &crowd[val[0]];
            let p2: &Person = &crowd[val[1]];
            let v1 = p2.velocity();
            let v2 = p1.velocity();
            let x1 = p1.position();
            let x2 = p2.position();

            let a = collision_physics::colission_update2(
                v1, p1.mass(), x1,
                v2, p2.mass(), x2
            );

            crowd[val[0]].change_velocity(a[0]);
            crowd[val[1]].change_velocity(a[1]);
        }
        values.iter().map(|x| [crowd[x[0]].id(), crowd[x[1]].id()]).collect()
    }

    pub fn are_colliding(crowd: &mut Vec<&mut Person>) -> Vec<usize> {
        let mut collisions: HashMap<usize, usize> = HashMap::new();
        for (idx1, i) in crowd.iter().enumerate() {
            for j in crowd.iter() {
                if j == i {
                    continue;
                }
                if i.peer_collision(j) {
                    collisions.entry(idx1).or_insert(0);
                }
            }
        }
        collisions.into_keys().collect()
    }
    pub fn are_colliding_pairs(crowd: &mut Vec<&mut Person>) -> Vec<Vec<u32>> {
        let mut collisions: HashMap<usize, Vec<u32>> = HashMap::new();
        for (idx1, i) in crowd.iter().enumerate() {
            for j in crowd.iter() {
                if j == i {
                    continue;
                }
                if i.peer_collision(j) {
                    let mut a = vec![i.id(), j.id()];
                    a.sort();
                    collisions.entry(idx1).or_insert(a);
                }
            }
        }
        collisions.into_values().collect()
    }
}

#[cfg(test)]
mod tests {

    use crate::person::Person;
    use crate::collision_physics::distance2;

    #[test]
    fn build_person() {
        let p = Person::new(4.0, [0.0, 0.0], [0.0, 0.0]);
        assert_eq!(p.radius(), 4.0);
        assert_eq!(p.diameter(), 8.0);
        assert_eq!(p.position(), [0.0, 0.0]);
    }

    #[test]
    fn distance() {
        let p1 = [4.0, 7.0];
        let p2 = [8.0, 10.0];
        assert_eq!(distance2(p1, p2), 5.0);
    }

    #[test]
    fn peer_collision() {
        let p = Person::new(10.0, [1.0, 4.0], [0.0, 0.0]);
        let q = Person::new(7.0, [10.0, 7.0], [0.0, 0.0]);
        assert_eq!(p.peer_collision(&q), true);
    }

    #[test]
    fn no_peer_collision() {
        let p = Person::new(4.0, [4.0, 7.0], [0.0, 0.0]);
        let q = Person::new(4.0, [10.0, 11.0], [0.0, 0.0]);
        assert_eq!(p.peer_collision(&q), false);
    }


    // #[test]

    // fn single_collision() {
    //   let p = Person::new(10.0, [1.0, 4.0, 2.0], [1.0, 0.0, 0.0]);
    //   let q = Person::new(7.0, [10.0, 7.0, 15.0], [1.0, 0.0, 0.0]);
    //   let r = Person::new(2.0, [20.0, 11.0, 13.0], [1.0, 0.0, 0.0] );
    //   let crowd = vec![&p, &q, &r];
    //   assert_eq!(naive_have_collided(&crowd).sort(), vec![&p, &q].sort())
    // }

    // #[test]
    // fn multiple_collisions() {
    //   let p = Person::new(10.0, [1.0, 4.0, 2.0]);
    //   let q = Person::new(7.0, [10.0, 7.0, 15.0]);
    //   let r = Person::new(4.0, [20.0, 11.0, 13.0]);
    //   let crowd = vec![&p, &q, &r];
    //   assert_eq!(naive_have_collided(crowd).sort(), vec![&p, &q].sort())
    // }

    // #[test]
    // fn no_collisions() {
    //   let p = Person::new(9.0, [1.0, 4.0, 2.0]);
    //   let q = Person::new(7.0, [10.0, 7.0, 15.0]);
    //   let r = Person::new(2.0, [20.0, 11.0, 13.0]);
    //   let crowd = vec![&p, &q, &r];
    //   assert_eq!(naive_have_collided(crowd), vec![] as Vec<&Person>)
    // }
}

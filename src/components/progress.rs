use std::collections::{ HashMap, HashSet };
use rand::{ Rng };
use components::item::{ ItemInstance };
use components::space::{ Level };

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    HasAccess(AreaInstance),
    HasItem(ItemInstance),
    GameWon,
}


// an abstract representation of a quest
pub struct Milestone {
    pub pre: Vec<Event>,
    pub post: Vec<Event>,
}


pub struct Area {
    pub min_level: Level,
    pub max_level: Level,
}

impl Area {
    pub fn new(min: Level, max: Level) -> Area {
        Area { min_level: min, max_level: max }
    }

    pub fn from_instance(instance: AreaInstance) -> Area {
        use self::AreaInstance::*;
        use self::Level::*;
        match instance {
            FuyoPenthouse => Area::new(Tower(10), Tower(10)),
            MainFrame => Area::new(Tower(5), Tower(8)),
            ObservationDeck => Area::new(Tower(9), Tower(9)),
            Entrance => Area::new(Tower(0), Tower(0)),

            NichireiRestaurant => Area::new(Tower(7), Tower(8)),
            NichireiLabs => Area::new(Tower(5), Tower(8)),

            YasudaLife => Area::new(Tower(2), Tower(3)),
            PrimeSecHQ => Area::new(Tower(4), Tower(7)),
            KayabaIndustries => Area::new(Tower(4), Tower(7)),
            KayabaRoboticsLabs => Area::new(Tower(6), Tower(7)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MilestoneInstance {
    TheHeist,
    AAA,
    KillKayabaCeo,
    KillYasudaCeo,
    KillPrimeSeqCeo,
    KillNichireiCeo,

    MilestoneCloseHack,
}

impl MilestoneInstance {
    pub fn values() -> [Self; 7] {
        use self::MilestoneInstance::*;
        [TheHeist, AAA, KillNichireiCeo, KillPrimeSeqCeo, KillYasudaCeo, KillKayabaCeo,
         MilestoneCloseHack]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AreaInstance {
    FuyoPenthouse,
    MainFrame,
    ObservationDeck,
    Entrance,

    NichireiRestaurant,
    NichireiLabs,
    YasudaLife,
    KayabaIndustries,
    KayabaRoboticsLabs,
    PrimeSecHQ,
}

impl AreaInstance {
    pub fn values() -> [Self; 10] {
        use self::AreaInstance::*;
        [FuyoPenthouse, MainFrame, ObservationDeck, Entrance, NichireiRestaurant, NichireiLabs,
        YasudaLife, KayabaIndustries, KayabaRoboticsLabs, PrimeSecHQ]
    }
}

impl Milestone {
    pub fn new(instance: MilestoneInstance) -> Milestone {
        use self::MilestoneInstance::*;
        use self::Event::*;
        use self::AreaInstance::*;
        match instance {
            TheHeist => Milestone {
                pre: vec![HasAccess(FuyoPenthouse)],
                post: vec![GameWon],
            },
            AAA => Milestone {
                pre: vec![HasAccess(MainFrame)],
                post: vec![HasAccess(FuyoPenthouse)],
            },
            KillKayabaCeo => Milestone {
                pre: vec![HasAccess(KayabaRoboticsLabs)],
                post: vec![HasAccess(MainFrame)],
            },
            KillYasudaCeo => Milestone {
                pre: vec![HasAccess(YasudaLife)],
                post: vec![HasAccess(MainFrame)],
            },
            KillPrimeSeqCeo => Milestone {
                pre: vec![HasAccess(PrimeSecHQ)],
                post: vec![HasAccess(MainFrame)],
            },
            KillNichireiCeo => Milestone {
                pre: vec![HasAccess(NichireiLabs)],
                post: vec![HasAccess(MainFrame)],
            },
            MilestoneCloseHack => Milestone {
                pre: vec![],
                post: vec![
                    HasAccess(NichireiLabs),
                    HasAccess(KayabaRoboticsLabs),
                    HasAccess(YasudaLife),
                    HasAccess(PrimeSecHQ),
                ],
            },
        }
    }

    pub fn areas(&self) -> Vec<AreaInstance> {
        use self::Event::*;
        self.pre.iter()
            .filter_map(|e| match *e {
                HasAccess(a) => Some(a),
                _ => None,
            })
            .collect()
    }

    fn create_instance_pool() -> HashMap<MilestoneInstance, Milestone> {
        MilestoneInstance::values()
            .iter()
            .cloned()
            .map(|i| (i, Milestone::new(i)))
            .collect()
    }


    pub fn generate_random_roadmap<R: Rng>(root: MilestoneInstance, prune: f32, rng: &mut R) -> Option<HashMap<MilestoneInstance, Milestone>> {
        let mut pool = Milestone::create_instance_pool();
        let mut result = HashMap::new();
        let mut open = vec![root];

        let mut fulfilled: HashSet<Event> = HashSet::new();
        while let Some(curr) = open.pop() {
            let to_fulfill = Milestone::discover_milestone(&mut pool, curr, &mut result, &mut fulfilled);
            if to_fulfill.len() > 0 {
                let chosen = Milestone::collect_fulfilling(&pool, &to_fulfill);

                // could not fulfill
                if chosen.len() == 0 {
                    return None;
                }

                let mut currently_fulfilled = Milestone::calculate_fulfillment(&pool, &chosen);
                Milestone::collect_and_prune(&pool, &mut currently_fulfilled, &chosen,
                                            &mut open, prune, rng);
            }
        }
        Some(result)
    }

    pub fn generate_roadmap(root: MilestoneInstance) -> Option<HashMap<MilestoneInstance, Milestone>> {
        let mut pool = Milestone::create_instance_pool();
        let mut result = HashMap::new();
        let mut open = vec![root];

        let mut fulfilled: HashSet<Event> = HashSet::new();
        while let Some(curr) = open.pop() {
            let to_fulfill = Milestone::discover_milestone(&mut pool, curr, &mut result, &mut fulfilled);
            if to_fulfill.len() > 0 {
                let chosen = Milestone::collect_fulfilling(&pool, &to_fulfill);

                // could not fulfill
                if chosen.len() == 0 {
                    return None;
                }

                // collect and prune.
                for i in chosen {
                    open.push(i);
                }
            }
        }
        Some(result)
    }

    fn discover_milestone(pool: &mut HashMap<MilestoneInstance, Milestone>,
                          curr: MilestoneInstance,
                          result: &mut HashMap<MilestoneInstance, Milestone>,
                          fulfilled: &mut HashSet<Event>) -> HashSet<Event> {
        // remove from pool and find out new events to be fulfilled
        let mut to_fulfill: HashSet<Event> = HashSet::new();
        match pool.remove(&curr) {
            Some(milestone) => {
                for post in milestone.post.clone() {
                    fulfilled.insert(post);
                };

                for pre in milestone.pre.clone() {
                    if !fulfilled.contains(&pre) {
                        to_fulfill.insert(pre);
                    }
                };
                result.insert(curr, milestone);
            },
            None => (),
        }
        to_fulfill
    }

    fn collect_fulfilling(pool: &HashMap<MilestoneInstance, Milestone>,
                          to_fulfill: &HashSet<Event>) -> Vec<MilestoneInstance> {
        let mut chosen = vec![];
        for (i, milestone) in pool {
            for post in milestone.post.clone() {
                if to_fulfill.contains(&post) {
                    chosen.push(*i);
                    break;
                }
            }
        }
        chosen
    }

    fn calculate_fulfillment(pool: &HashMap<MilestoneInstance, Milestone>,
                             chosen: &Vec<MilestoneInstance>) -> HashMap<Event, i32> {
        let mut currently_fulfilled: HashMap<Event, i32> = HashMap::new();
        for i in chosen {
            for post in pool.get(&i).unwrap().post.clone() {
                let occurrences = currently_fulfilled.entry(post).or_insert(0);
                *occurrences += 1;
            }
        }
        currently_fulfilled
    }

    fn collect_and_prune<R: Rng>(pool: &HashMap<MilestoneInstance, Milestone>,
                                 currently_fulfilled: &mut HashMap<Event, i32>,
                                 chosen: &Vec<MilestoneInstance>,
                                 open: &mut Vec<MilestoneInstance>,
                                 prune: f32, rng: &mut R) {
        // collect and prune.
        for i in chosen {
            let mut take = true;
            if prune > 0.0 && prune > rng.next_f32() {
                take = pool.get(&i).unwrap().post
                    .iter()
                    .any(|p| currently_fulfilled.get(p).unwrap_or(&0) < &2);
            }
            if take {
                open.push(*i);
            } else {
                for post in pool.get(&i).unwrap().post.clone() {
                    let occurrences = currently_fulfilled.entry(post).or_insert(0);
                    *occurrences -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rand::thread_rng;
    use components::progress::{ Milestone, MilestoneInstance };

    fn gen_map(instance: MilestoneInstance) -> Option<HashMap<MilestoneInstance, Milestone>> {
        Milestone::generate_roadmap(instance)
    }

    fn gen_rand_map(instance: MilestoneInstance, prune: f32) -> Option<HashMap<MilestoneInstance, Milestone>> {
        Milestone::generate_random_roadmap(instance, prune, &mut thread_rng())
    }

    #[test]
    fn trivial_roadmap() {
        use self::MilestoneInstance::*;
        let single_node = gen_map(MilestoneCloseHack).unwrap();
        assert!(single_node.len() == 1);
        assert!(single_node.get(&MilestoneCloseHack).is_some());
    }

    #[test]
    fn simple_roadmap() {
        use self::MilestoneInstance::*;
        let single_node = gen_map(KillNichireiCeo).unwrap();
        assert!(single_node.len() == 2);
        assert!(single_node.get(&KillNichireiCeo).is_some());
        assert!(single_node.get(&MilestoneCloseHack).is_some());
    }

    #[test]
    fn full_roadmap() {
        use self::MilestoneInstance::*;
        let single_node = gen_map(TheHeist).unwrap();
        assert!(single_node.len() == 7, "should be 7 but was {}", single_node.len());
        assert!(single_node.get(&TheHeist).is_some());
        assert!(single_node.get(&AAA).is_some());
        assert!(single_node.get(&KillPrimeSeqCeo).is_some());
        assert!(single_node.get(&KillNichireiCeo).is_some());
        assert!(single_node.get(&KillKayabaCeo).is_some());
        assert!(single_node.get(&KillYasudaCeo).is_some());
        assert!(single_node.get(&MilestoneCloseHack).is_some());
    }

    #[test]
    fn fully_pruned_roadmap() {
        use self::MilestoneInstance::*;
        let single_node = gen_rand_map(TheHeist, 1.0).unwrap();
        assert!(single_node.len() == 4, "should be 4 but was {}", single_node.len());
        assert!(single_node.get(&TheHeist).is_some());
        assert!(single_node.get(&AAA).is_some());
        assert!(single_node.get(&KillPrimeSeqCeo)
                .or(single_node.get(&KillNichireiCeo))
                .or(single_node.get(&KillKayabaCeo))
                .or(single_node.get(&KillYasudaCeo)).is_some());
        assert!(single_node.get(&MilestoneCloseHack).is_some());
    }

    #[test]
    fn fully_unpruned_roadmap() {
        use self::MilestoneInstance::*;
        let single_node = gen_rand_map(TheHeist, 0.0).unwrap();
        assert!(single_node.len() == 7, "should be 7 but was {}", single_node.len());
        assert!(single_node.get(&TheHeist).is_some());
        assert!(single_node.get(&AAA).is_some());
        assert!(single_node.get(&KillPrimeSeqCeo).is_some());
        assert!(single_node.get(&KillNichireiCeo).is_some());
        assert!(single_node.get(&KillKayabaCeo).is_some());
        assert!(single_node.get(&KillYasudaCeo).is_some());
        assert!(single_node.get(&MilestoneCloseHack).is_some());
    }
}

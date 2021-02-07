use bit_set;
use dogs::searchspace::{SearchSpace, SearchTree, TotalChildrenExpansion, PartialChildrenExpansion, GuidedSpace, PrefixEquivalenceTree};
// use std::cmp::Ordering;
use ordered_float::OrderedFloat;

use super::sop;
use crate::sop::CityId;

#[derive(Debug, Clone)]
pub struct ForwardNode {
    prefix: Vec<u32>,
    added: bit_set::BitSet,
    cost: u32,
    /// PartialChildrenExpansion pointer
    pce_pointer: usize,
}


#[derive(Debug)]
pub struct ForwardSearch {
    inst: sop::Instance,
    sorted_possible_successors: Vec<Vec<CityId>>,
}

impl SearchSpace<ForwardNode, Vec<CityId>> for ForwardSearch {
    fn solution(&mut self, node: &ForwardNode) -> Vec<CityId> {
        debug_assert!(self.goal(node));
        return node.prefix.clone();
    }
}

impl GuidedSpace<ForwardNode, OrderedFloat<f64>> for ForwardSearch {
    fn guide(&mut self, node: &ForwardNode) -> OrderedFloat<f64> {
        return OrderedFloat(node.cost as f64);
    }
}

impl TotalChildrenExpansion<ForwardNode> for ForwardSearch {
    fn children(&mut self, node: &mut ForwardNode) -> Vec<ForwardNode> {
        let mut res = Vec::new();
        // for each city
        for i in self
            .inst
            .possible_successors(&ForwardSearch::get_last_city(node))
            .iter()
        {
            // if already added, skip
            if node.added.contains(*i as usize) {
                continue;
            }
            // possibly define in the preprocessing, a predecesor bitset then use an inter operation (should be faster)
            // if exists a pred not added, skip
            let mut to_add = true;
            for e in self.inst.predecessors(i).iter() {
                if !node.added.contains(*e as usize) {
                    to_add = false;
                    break;
                }
            }
            if to_add {
                // else add to children current node
                res.push(self.add_city(node, *i));
            }
        }
        return res;
    }
}


impl PartialChildrenExpansion<ForwardNode> for ForwardSearch {
    fn get_next_child(&mut self, n:&mut ForwardNode) -> Option<ForwardNode> {
        let last_city:u32 = ForwardSearch::get_last_city(n);
        // for each possible next city
        let possible_successors:&Vec<CityId> = &self.sorted_possible_successors[last_city as usize];
        while n.pce_pointer < possible_successors.len() {
            let successor = possible_successors[n.pce_pointer];
            n.pce_pointer += 1;
            // if already added, skip
            if n.added.contains(successor as usize) {
                continue;
            }
            // check that successor is not a precedence of a not added vertex
            let mut to_add = true;
            for e in self.inst.predecessors(&successor).iter() {
                if !n.added.contains(*e as usize) {
                    to_add = false;
                    break;
                }
            }
            if !to_add {
                continue;
            }
            let c = self.add_city(n, successor);
            return Some(c);
        }
        return None;
    }
}


impl SearchTree<ForwardNode, u32> for ForwardSearch {
    fn root(&mut self) -> ForwardNode {
        let mut res = ForwardNode {
            prefix: Vec::new(),
            added: bit_set::BitSet::new(),
            cost: 0,
            pce_pointer: 0,
        };
        res.prefix.push(0);
        res.added.insert(0); // add root
        return res;
    }

    fn bound(&mut self, node: &ForwardNode) -> u32 {
        return node.cost;
    }


    fn goal(&mut self, node: &ForwardNode) -> bool {
        return node.prefix.len() as u32 == self.inst.nb_cities();
    }

}

#[derive(Eq, Hash)]
pub struct ForwardNodePE {
    last: CityId,
    set: bit_set::BitSet,
}

impl PartialEq for ForwardNodePE {
    fn eq(&self, other: &Self) -> bool {
        self.last == other.last && self.set == other.set
    }
}

impl PrefixEquivalenceTree<ForwardNode, u32, ForwardNodePE> for ForwardSearch {
    fn get_pe(&self, s: &ForwardNode) -> ForwardNodePE {
        return ForwardNodePE {
            last: ForwardSearch::get_last_city(s),
            set: s.added.clone()
        };
    }

    fn prefix_bound(&self, s: &ForwardNode) -> u32 {
        return s.cost;
    }
}

impl ForwardSearch {
    pub fn new(filename: &str) -> ForwardSearch {
        let inst = sop::Instance::new(&filename).unwrap();
        let mut possible_successors:Vec<Vec<CityId>> = Vec::new();
        for i in 0..inst.nb_cities() {
            let mut tmp = inst.possible_successors(&i).clone();
            tmp.sort_by_key(|a| { inst.cost_arc(i, *a) });
            possible_successors.push(tmp);
        }
        ForwardSearch {
            inst: inst,
            sorted_possible_successors: possible_successors,
        }
    }

    fn add_city(&self, node: &ForwardNode, i: u32) -> ForwardNode {
        let mut res = node.clone();
        res.pce_pointer = 0;
        let last = ForwardSearch::get_last_city(node);
        res.cost += self.inst.cost_arc(last, i) as u32;
        res.prefix.push(i);
        res.added.insert(i as usize);
        return res;
    }

    fn get_last_city(node: &ForwardNode) -> u32 {
        return node.prefix[node.prefix.len() - 1];
    }
}

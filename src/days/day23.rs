use crate::aoc::{read_lines, Answers, Solution};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Keys;
use std::error::Error;
use std::hash::Hash;

#[derive(Debug)]
pub struct Day23 {
    network: Graph<String>,
}

impl Day23 {
    pub fn new() -> Day23 {
        Day23 {
            network: Graph::new(),
        }
    }
}

impl Solution for Day23 {
    fn parse_input(&mut self) -> Result<(), Box<dyn Error>> {
        let filename = "./data/day23.txt";
        let lines = read_lines(filename)?;
        for line in lines.flatten() {
            let mut parts = line.split("-").map(|s| s.to_owned());
            self.network.link(
                parts.next().ok_or("missing first part")?,
                parts.next().ok_or("missing second part")?,
            );
        }
        Ok(())
    }

    fn solve(&mut self) -> Result<Answers, Box<dyn Error>> {
        let cliques = self.network.cliques(3);
        let part1 = cliques
            .iter()
            .filter(|&c| c.iter().any(|s| s.starts_with("t")))
            .count();
        let part2 = self.network.largest_clique().join(",");
        Ok(Answers::both(part1, part2))
    }
}

#[derive(Debug)]
struct Graph<T: Hash + Eq + Clone + Ord> {
    nodes: FxHashMap<T, FxHashSet<T>>,
}

#[allow(dead_code)]
impl<T: Hash + Eq + Clone + Ord> Graph<T> {
    fn new() -> Graph<T> {
        Graph {
            nodes: FxHashMap::default(),
        }
    }

    fn link(&mut self, a: T, b: T) {
        self.nodes
            .entry(a.clone())
            .or_insert_with(FxHashSet::default)
            .insert(b.clone());
        self.nodes
            .entry(b)
            .or_insert_with(FxHashSet::default)
            .insert(a);
    }

    fn add(&mut self, node: T) {
        self.nodes.entry(node).or_insert_with(FxHashSet::default);
    }

    fn contains_node(&self, node: &T) -> bool {
        self.nodes.contains_key(node)
    }

    fn contains_edge(&self, a: &T, b: &T) -> bool {
        self.nodes.get(a).map(|s| s.contains(b)).unwrap_or(false)
    }

    fn nodes(&self) -> Keys<T, FxHashSet<T>> {
        self.nodes.keys()
    }

    fn neighbors(&self, node: &T) -> FxHashSet<T> {
        self.nodes
            .get(node)
            .cloned()
            .unwrap_or(FxHashSet::default())
    }

    fn cliques(&self, k: usize) -> Vec<Vec<T>> {
        let mut cliques = Vec::new();
        let mut nodes: Vec<T> = self.nodes().cloned().collect();
        nodes.sort();
        for (i, node) in nodes.iter().enumerate() {
            let mut current = vec![node.clone()];
            let mut candidates: Vec<T> = nodes[i + 1..]
                .iter()
                .filter(|&v| self.contains_edge(node, v))
                .cloned()
                .collect();
            self.extend_clique(&mut current, &mut candidates, k, &mut cliques);
        }
        cliques
    }

    fn extend_clique(
        &self,
        current: &mut Vec<T>,
        candidates: &mut Vec<T>,
        k: usize,
        cliques: &mut Vec<Vec<T>>,
    ) {
        if current.len() == k {
            cliques.push(current.clone());
            return;
        }
        while let Some(v) = candidates.pop() {
            let mut new_candidates: Vec<T> = candidates
                .iter()
                .filter(|&u| self.contains_edge(&v, u))
                .cloned()
                .collect();
            current.push(v.clone());
            self.extend_clique(current, &mut new_candidates, k, cliques);
            current.pop();
        }
    }

    fn largest_clique(&self) -> Vec<T> {
        let mut cliques = Vec::new();
        let mut nodes: Vec<T> = self.nodes().cloned().collect();
        nodes.sort();
        for (i, node) in nodes.iter().enumerate() {
            let mut current = vec![node.clone()];
            let mut candidates: Vec<T> = nodes[i + 1..]
                .iter()
                .filter(|&v| self.contains_edge(node, v))
                .cloned()
                .collect();
            self.extend_largest_clique(&mut current, &mut candidates, &mut cliques);
        }
        cliques.sort_by(|a, b| b.len().cmp(&a.len()));
        cliques[0].sort();
        cliques[0].clone()
    }

    fn extend_largest_clique(
        &self,
        current: &mut Vec<T>,
        candidates: &mut Vec<T>,
        cliques: &mut Vec<Vec<T>>,
    ) {
        if candidates.len() == 0 {
            cliques.push(current.clone());
            return;
        }
        while let Some(v) = candidates.pop() {
            let mut new_candidates: Vec<T> = candidates
                .iter()
                .filter(|&u| self.contains_edge(&v, u))
                .cloned()
                .collect();
            current.push(v.clone());
            self.extend_largest_clique(current, &mut new_candidates, cliques);
            current.pop();
        }
    }
}

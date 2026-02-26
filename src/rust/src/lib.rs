use extendr_api::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Struktura pomocnicza dla algorytmu Dijkstry
#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Funkcja obliczająca IDW na grafie.
#[extendr]
pub fn graph_idw_rust(
    from_nodes: Vec<i32>,
    values: Vec<f64>,
    to_nodes: Vec<i32>,
    adj_from: Vec<i32>,
    adj_to: Vec<i32>,
    adj_weight: Vec<f64>,
    p: f64,
    max_dist: f64
) -> Vec<f64> {
    let mut graph: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
    for i in 0..adj_from.len() {
        let u = adj_from[i] as usize;
        let v = adj_to[i] as usize;
        let w = adj_weight[i];
        graph.entry(u).or_insert_with(|| Vec::with_capacity(4)).push((v, w));
    }

    let mut source_data: HashMap<usize, f64> = HashMap::new();
    for i in 0..from_nodes.len() {
        source_data.insert(from_nodes[i] as usize, values[i]);
    }

    to_nodes.iter().map(|&target| {
        let target_idx = target as usize;
        if let Some(&val) = source_data.get(&target_idx) {
            return val;
        }

        let mut dists = HashMap::new();
        let mut heap = BinaryHeap::new();

        dists.insert(target_idx, 0.0);
        heap.push(State { cost: 0.0, position: target_idx });

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        while let Some(State { cost, position }) = heap.pop() {
            if cost > max_dist { break; }
            if cost > *dists.get(&position).unwrap_or(&f64::INFINITY) { continue; }

            if let Some(&val) = source_data.get(&position) {
                if cost > 0.0 {
                    let w = 1.0 / cost.powf(p);
                    numerator += w * val;
                    denominator += w;
                } else {
                    return val;
                }
            }

            if let Some(neighbors) = graph.get(&position) {
                for (next_node, weight) in neighbors {
                    let next_cost = cost + weight;
                    if next_cost <= max_dist && next_cost < *dists.get(next_node).unwrap_or(&f64::INFINITY) {
                        dists.insert(*next_node, next_cost);
                        heap.push(State { cost: next_cost, position: *next_node });
                    }
                }
            }
        }

        if denominator > 0.0 { numerator / denominator } else { f64::NAN }
    }).collect()
}

// Nazwa modułu musi być zgodna z nazwą pakietu i pisana małymi literami
extendr_module! {
    mod graphidw;
    fn graph_idw_rust;
}

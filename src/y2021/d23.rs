use std::collections::{BinaryHeap, BTreeMap, HashMap};

// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below
// Warning: disgusting code below

#[aoc_generator(day23)]
fn generator(input: &str) -> State {
    let bytes = input.as_bytes();
    let mut a = (15, 15);
    let mut b = (15, 15);
    let mut c = (15, 15);
    let mut d = (15, 15);
    let amphipods = [bytes[31], bytes[33], bytes[35], bytes[37], bytes[45], bytes[47], bytes[49], bytes[51]];
    for (i, amphipod) in amphipods.into_iter().enumerate().map(|(i, p)| (i as u8, p)) {
        match amphipod {
            b'A' => {
                if a.0 == 15 {
                    a.0 = i + 7;
                }
                else {
                    a.1 = i + 7;
                }
            }
            b'B' => {
                if b.0 == 15 {
                    b.0 = i + 7;
                }
                else {
                    b.1 = i + 7;
                }
            }
            b'C' => {
                if c.0 == 15 {
                    c.0 = i + 7;
                }
                else {
                    c.1 = i + 7;
                }
            }
            b'D' => {
                if d.0 == 15 {
                    d.0 = i + 7;
                }
                else {
                    d.1 = i + 7;
                }
            }
            _ => unreachable!()
        }
    }
    State {
        cost: 0,
        data: Data {
            pods: [a.0, a.1, b.0, b.1, c.0, c.1, d.0, d.1]
        }
    }
}

type Cost = usize;
type Index = usize;

#[derive(Debug, Clone, Copy, Eq)]
struct Data {
    pods: [u8; 8]
}

// pairs are unordered
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        merge(self.pods[0], self.pods[1]) == merge(other.pods[0], other.pods[1]) &&
        merge(self.pods[2], self.pods[3]) == merge(other.pods[2], other.pods[3]) &&
        merge(self.pods[4], self.pods[5]) == merge(other.pods[4], other.pods[5]) &&
        merge(self.pods[6], self.pods[7]) == merge(other.pods[6], other.pods[7])
    }
}

fn merge(x: u8, y: u8) -> usize {
    (x.min(y) as usize) << 4 | x.max(y) as usize
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Cell {
    Empty = 0,
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}


impl Data {
    fn index(self) -> Index {
        merge(self.pods[0], self.pods[1]) << 24 | 
        merge(self.pods[2], self.pods[3]) << 16 | 
        merge(self.pods[4], self.pods[5]) << 8  | 
        merge(self.pods[6], self.pods[7])
    }


    fn adjacents(self) -> Vec<(Data, Cost)> {
        let mut out = vec![];
        out.extend(self.valid_moves(0));
        out.extend(self.valid_moves(1));
        out.extend(self.valid_moves(2));
        out.extend(self.valid_moves(3));
        out.extend(self.valid_moves(4));
        out.extend(self.valid_moves(5));
        out.extend(self.valid_moves(6));
        out.extend(self.valid_moves(7));
        out
    }

    fn value(self, position: u8) -> Cell {
        let mut value = Cell::Empty;
        if self.pods[0] == position { value = Cell::Amber; }
        else if self.pods[1] == position { value = Cell::Amber; }
        else if self.pods[2] == position { value = Cell::Bronze; }
        else if self.pods[3] == position { value = Cell::Bronze; }
        else if self.pods[4] == position { value = Cell::Copper; }
        else if self.pods[5] == position { value = Cell::Copper; }
        else if self.pods[6] == position { value = Cell::Desert; }
        else if self.pods[7] == position { value = Cell::Desert; }
        value
    }

    fn is_empty(self, position: u8) -> bool {
        self.value(position) == Cell::Empty
    }

    fn path_is_empty(self, from: u8, to: u8) -> bool {
        // target is blocked
        if !self.is_empty(to) {
            return false
        }
        // bidirectional path checks below
        let corridoor = from.min(to);
        let entrance = from.max(to);
        // valid for all colors
        if entrance - 6 == corridoor || entrance - 5 == corridoor {
            return true
        }
        if entrance - 7 == corridoor && self.is_empty(entrance - 6) {
            return true
        }
        if entrance - 4 == corridoor && self.is_empty(entrance - 5) {
            return true
        }
        // BCD
        if entrance >= 8 {
            if entrance - 8 == corridoor && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // CD
        if entrance >= 9 {
            if entrance - 9 == corridoor && self.is_empty(entrance - 8) && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // D
        if entrance >= 10 {
            if entrance - 10 == corridoor && self.is_empty(entrance - 9) && self.is_empty(entrance - 8) && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // ABC
        if entrance <= 9 {
            if entrance - 3 == corridoor && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // AB
        if entrance <= 8 {
            if entrance - 2 == corridoor && self.is_empty(entrance - 3) && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // A
        if entrance <= 7 {
            if entrance - 1 == corridoor && self.is_empty(entrance - 2) && self.is_empty(entrance - 3) && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // no
        false
    }

    fn distance(self, from: u8, to: u8) -> usize {
        let corridoor = from.min(to);
        let entrance = from.max(to);
        (match (corridoor, entrance) {
            (0, e) => 2 * (e - 7) + 3,
            (6, e) => 2 * (10 - e) + 3,
            (c, e) if e - 6 >= c => 2 * (e - c - 5),
            (c, e) if e - 5 <= c => 2 * (6 + c - e),
            _ => unreachable!()
        }) as usize
    }

    fn valid_moves(self, idx: usize) -> Vec<(Data, Cost)> {
        let from = self.pods[idx];
        let pod = self.value(from);
        let (goal_far, goal_close) = match pod {
            Cell::Amber => (11, 7),
            Cell::Bronze => (12, 8),
            Cell::Copper => (13, 9),
            Cell::Desert => (14, 10),
            _ => unreachable!()
        };
        // Get rid of the obvious cases applicable for all colors first

        // comfy
        if from == goal_far {
            return vec![]
        }
        // comfy
        if from == goal_close && self.value(goal_far) == pod {
            return vec![]
        }
        // blocked
        if from >= 11 && !self.is_empty(from - 4) {
            return vec![]
        }
        // blocked
        if from <= 6 && !self.is_empty(goal_close) {
            return vec![]
        }
        // blocked
        if from == 0 && !self.is_empty(1) {
            return vec![]
        }
        // blocked
        if from == 6 && !self.is_empty(5) {
            return vec![]
        }
        // blocked
        if from >= 7 && from <= 10 && !self.is_empty(from - 6) && !self.is_empty(from - 5) {
            return vec![]
        }
        // blocked
        if from >= 11 && !self.is_empty(from - 10) && !self.is_empty(from - 9) {
            return vec![]
        }

        
        // Now there are only two cases: move into room & move out of room
        let mut out = vec![];
        if from >= 7 {
            let (entrance, extra) = if from >= 11 {(from - 4, 1)} else {(from, 0)};
            let targets = [0, 1, 2, 3, 4, 5, 6];
            for target in targets {
                if self.path_is_empty(entrance, target) {
                    let mut new = self;
                    new.pods[idx] = target;
                    let cost = (self.distance(entrance, target) + extra) * pod as usize;
                    out.push((new, cost));
                }
            }
        }
        else {
            let (target, extra) = if self.is_empty(goal_far) {(goal_far, 1)} else {(goal_close, 0)};
            if self.path_is_empty(from, goal_close) {
                let mut new = self;
                new.pods[idx] = target;
                let cost = (self.distance(from, goal_close) + extra) * pod as usize;
                out.push((new, cost));
            }
        }        
        out
    }
    
    // 00 01  02  03  04  05 06
    //      07  08  09  10
    //      11  12  13  14
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: Cost,
    data: Data,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap
        other.cost.cmp(&self.cost)
            // ties
            .then_with(|| self.data.index().cmp(&other.data.index()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day23, part1)]
fn organize_dijkstra(start: &State) -> Cost {
    let mut costs = BTreeMap::new();
    let mut heap = BinaryHeap::new();

    costs.insert(start.data.index(), 0);
    heap.push(*start);
    let end = Data {
        pods: [7, 11, 8, 12, 9, 13, 10, 14]
    };

    // basic dijkstra strucure
    while let Some(State { cost, data }) = heap.pop() {
        if data == end {
            return cost
        }

        if cost > *costs.entry(data.index()).or_insert(usize::MAX) {
            continue
        }

        // the biggest change is in the implementation of adjacent nodes
        for (next_data, extra_cost) in data.adjacents() {
            let node = State { cost: cost + extra_cost, data: next_data };
            if node.cost < *costs.entry(next_data.index()).or_insert(usize::MAX) {
                heap.push(node);
                costs.insert(next_data.index(), node.cost);
            }
        }
    }
    usize::MAX
}

#[derive(Debug, Clone, Copy, Eq)]
struct Data2 {
    pods: [u8; 16]
}

// pairs are unordered
impl PartialEq for Data2 {
    fn eq(&self, other: &Self) -> bool {
        merge2(self.pods[0], self.pods[1], self.pods[2], self.pods[3]) == merge2(other.pods[0], other.pods[1], other.pods[2], other.pods[3]) &&
        merge2(self.pods[4], self.pods[5], self.pods[6], self.pods[7]) == merge2(other.pods[4], other.pods[5], other.pods[6], other.pods[7]) &&
        merge2(self.pods[8], self.pods[9], self.pods[10], self.pods[11]) == merge2(other.pods[8], other.pods[9], other.pods[10], other.pods[11]) &&
        merge2(self.pods[12], self.pods[13], self.pods[14], self.pods[15]) == merge2(other.pods[12], other.pods[13], other.pods[14], other.pods[15])
    }
}
fn merge2(x: u8, y: u8, z: u8, w: u8) -> usize {
    let mut q = [x as usize, y as usize, z as usize, w as usize];
    q.sort_unstable();
    q[0] << 15 | q[1] << 10 | q[2] << 5 | q[3]
}

type Index2 = (usize, usize, usize, usize);

impl Data2 {
    fn index(self) -> Index2 {
        (
            merge2(self.pods[0], self.pods[1], self.pods[2], self.pods[3]),
            merge2(self.pods[4], self.pods[5], self.pods[6], self.pods[7]),
            merge2(self.pods[8], self.pods[9], self.pods[10], self.pods[11]),
            merge2(self.pods[12], self.pods[13], self.pods[14], self.pods[15]),
        )
    }


    fn adjacents(self) -> Vec<(Data2, Cost)> {
        let mut out = vec![];
        out.extend(self.valid_moves(0));
        out.extend(self.valid_moves(1));
        out.extend(self.valid_moves(2));
        out.extend(self.valid_moves(3));
        out.extend(self.valid_moves(4));
        out.extend(self.valid_moves(5));
        out.extend(self.valid_moves(6));
        out.extend(self.valid_moves(7));
        out.extend(self.valid_moves(8));
        out.extend(self.valid_moves(9));
        out.extend(self.valid_moves(10));
        out.extend(self.valid_moves(11));
        out.extend(self.valid_moves(12));
        out.extend(self.valid_moves(13));
        out.extend(self.valid_moves(14));
        out.extend(self.valid_moves(15));
        out
    }

    fn value(self, position: u8) -> Cell {
        let mut value = Cell::Empty;
             if self.pods[0]  == position { value = Cell::Amber;  }
        else if self.pods[1]  == position { value = Cell::Amber;  }
        else if self.pods[2]  == position { value = Cell::Amber;  }
        else if self.pods[3]  == position { value = Cell::Amber;  }
        else if self.pods[4]  == position { value = Cell::Bronze; }
        else if self.pods[5]  == position { value = Cell::Bronze; }
        else if self.pods[6]  == position { value = Cell::Bronze; }
        else if self.pods[7]  == position { value = Cell::Bronze; }
        else if self.pods[8]  == position { value = Cell::Copper; }
        else if self.pods[9]  == position { value = Cell::Copper; }
        else if self.pods[10] == position { value = Cell::Copper; }
        else if self.pods[11] == position { value = Cell::Copper; }
        else if self.pods[12] == position { value = Cell::Desert; }
        else if self.pods[13] == position { value = Cell::Desert; }
        else if self.pods[14] == position { value = Cell::Desert; }
        else if self.pods[15] == position { value = Cell::Desert; }
        value
    }

    fn is_empty(self, position: u8) -> bool {
        self.value(position) == Cell::Empty
    }

    // 00 01  02  03  04  05 06
    //      07  08  09  10
    //      11  12  13  14
    //      15  16  17  18
    //      19  20  21  22


    fn path_is_empty(self, from: u8, to: u8) -> bool {
        // target is blocked
        if !self.is_empty(to) {
            return false
        }
        // bidirectional path checks below
        let corridoor = from.min(to);
        let entrance = from.max(to);
        // valid for all colors
        if entrance - 6 == corridoor || entrance - 5 == corridoor {
            return true
        }
        if entrance - 7 == corridoor && self.is_empty(entrance - 6) {
            return true
        }
        if entrance - 4 == corridoor && self.is_empty(entrance - 5) {
            return true
        }
        // BCD
        if entrance >= 8 {
            if entrance - 8 == corridoor && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // CD
        if entrance >= 9 {
            if entrance - 9 == corridoor && self.is_empty(entrance - 8) && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // D
        if entrance >= 10 {
            if entrance - 10 == corridoor && self.is_empty(entrance - 9) && self.is_empty(entrance - 8) && self.is_empty(entrance - 7) && self.is_empty(entrance - 6) {
                return true
            }
        }
        // ABC
        if entrance <= 9 {
            if entrance - 3 == corridoor && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // AB
        if entrance <= 8 {
            if entrance - 2 == corridoor && self.is_empty(entrance - 3) && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // A
        if entrance <= 7 {
            if entrance - 1 == corridoor && self.is_empty(entrance - 2) && self.is_empty(entrance - 3) && self.is_empty(entrance - 4) && self.is_empty(entrance - 5) {
                return true
            }
        }
        // no
        false
    }

    fn distance(self, from: u8, to: u8) -> usize {
        let corridoor = from.min(to);
        let entrance = from.max(to);
        (match (corridoor, entrance) {
            (0, e) => 2 * (e - 7) + 3,
            (6, e) => 2 * (10 - e) + 3,
            (c, e) if e - 6 >= c => 2 * (e - c - 5),
            (c, e) if e - 5 <= c => 2 * (6 + c - e),
            _ => unreachable!()
        }) as usize
    }

    fn valid_moves(self, idx: usize) -> Vec<(Data2, Cost)> {
        let from = self.pods[idx];
        let pod = self.value(from);
        let (goal_furthest, goal_further, goal_far, goal_close) = match pod {
            Cell::Amber => (19, 15, 11, 7),
            Cell::Bronze => (20, 16, 12, 8),
            Cell::Copper => (21, 17, 13, 9),
            Cell::Desert => (22, 18, 14, 10),
            _ => unreachable!()
        };
        // Get rid of the obvious cases applicable for all colors first

        // comfy
        if from == goal_furthest {
            return vec![]
        }
        // comfy
        if from == goal_further && self.value(goal_furthest) == pod {
            return vec![]
        }
        // comfy
        if from == goal_far && self.value(goal_further) == pod && self.value(goal_furthest) == pod {
            return vec![]
        }
        // comfy
        if from == goal_close && self.value(goal_far) == pod && self.value(goal_further) == pod && self.value(goal_furthest) == pod {
            return vec![]
        }
        // blocked
        if from >= 11 && !self.is_empty(from - 4) {
            return vec![]
        }
        // blocked
        if from >= 15 && !self.is_empty(from - 8) {
            return vec![]
        }
        // blocked
        if from >= 19 && !self.is_empty(from - 12) {
            return vec![]
        }
        // blocked
        if from <= 6 && !self.is_empty(goal_close) {
            return vec![]
        }
        // blocked
        if from <= 6 && ((!self.is_empty(goal_furthest) && self.value(goal_furthest) != pod) || (!self.is_empty(goal_further) && self.value(goal_further) != pod) || (!self.is_empty(goal_far) && self.value(goal_far) != pod)) {
            return vec![]
        }
        // blocked
        if from == 0 && !self.is_empty(1) {
            return vec![]
        }
        // blocked
        if from == 6 && !self.is_empty(5) {
            return vec![]
        }
        // blocked
        if from >= 7 && from <= 10 && !self.is_empty(from - 6) && !self.is_empty(from - 5) {
            return vec![]
        }
        // blocked
        if from >= 11 && from <= 14 && !self.is_empty(from - 10) && !self.is_empty(from - 9) {
            return vec![]
        }
        // blocked
        if from >= 15 && from <= 18 && !self.is_empty(from - 14) && !self.is_empty(from - 13) {
            return vec![]
        }
        // blocked
        if from >= 19 && from <= 22 && !self.is_empty(from - 18) && !self.is_empty(from - 17) {
            return vec![]
        }

        
        // Now there are only two cases: move into room & move out of room
        let mut out = vec![];
        if from >= 7 {
            let (entrance, extra) = if from >= 19 {(from - 12, 3)} else if from >= 15 {(from - 8, 2)} else if from >= 11 {(from - 4, 1)} else {(from, 0)};
            let targets = [0, 1, 2, 3, 4, 5, 6];
            for target in targets {
                if self.path_is_empty(entrance, target) {
                    let mut new = self;
                    new.pods[idx] = target;
                    let cost = (self.distance(entrance, target) + extra) * pod as usize;
                    out.push((new, cost));
                }
            }
        }
        else {
            let (target, extra) = if self.is_empty(goal_furthest) {(goal_furthest, 3)} else if self.is_empty(goal_further) {(goal_further, 2)} else if self.is_empty(goal_far) {(goal_far, 1)} else {(goal_close, 0)};
            if self.path_is_empty(from, goal_close) {
                let mut new = self;
                new.pods[idx] = target;
                let cost = (self.distance(from, goal_close) + extra) * pod as usize;
                out.push((new, cost));
            }
        }        
        out
    }   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State2 {
    cost: Cost,
    data: Data2,
}

fn shift(idx: u8) -> u8 {
    if idx >= 11 {
        idx + 8
    }
    else {
        idx
    }
}

impl State2 {
    fn from_state(state: State) -> Self {
        let a = [shift(state.data.pods[0]), shift(state.data.pods[1]), 14, 17];
        let b = [shift(state.data.pods[2]), shift(state.data.pods[3]), 13, 16];
        let c = [shift(state.data.pods[4]), shift(state.data.pods[5]), 12, 18];
        let d = [shift(state.data.pods[6]), shift(state.data.pods[7]), 11, 15];
        State2 {
            cost: state.cost,
            data: Data2 { 
                pods: [
                    a[0], a[1], a[2], a[3],
                    b[0], b[1], b[2], b[3],
                    c[0], c[1], c[2], c[3],
                    d[0], d[1], d[2], d[3],
                ]
            }
        }
    }
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap
        other.cost.cmp(&self.cost)
            // ties
            .then_with(|| self.data.index().cmp(&other.data.index()))
    }
}

impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[aoc(day23, part2)]
fn tall_dijkstra(start: &State) -> Cost {
    let start = State2::from_state(*start);
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();

    costs.insert(start.data.index(), 0);
    heap.push(start);
    let end = Data2 {
        pods: [7, 11, 15, 19, 8, 12, 16, 20, 9, 13, 17, 21, 10, 14, 18, 22]
    };

    // basic dijkstra strucure
    while let Some(State2 { cost, data }) = heap.pop() {
        if data == end {
            return cost
        }

        if cost > *costs.entry(data.index()).or_insert(usize::MAX) {
            continue
        }

        // the biggest change is in the implementation of adjacent nodes
        for (next_data, extra_cost) in data.adjacents() {
            let node = State2 { cost: cost + extra_cost, data: next_data };
            if node.cost < *costs.entry(next_data.index()).or_insert(usize::MAX) {
                heap.push(node);
                costs.insert(next_data.index(), node.cost);
            }
        }
    }
    usize::MAX
}

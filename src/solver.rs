use crate::board::Board;

fn adjacent(a: Coord, b: Coord) -> bool {
    let (x1, y1) = (a.0 as isize, a.1 as isize);
    let (x2, y2) = (b.0 as isize, b.1 as isize);
    if x1 == x2 && (y1 == y2 + 1 || y1 == y2 - 1) {
        return true;
    }
    if y1 == y2 && (x1 == x2 + 1 || x1 == x2 - 1) {
        return true;
    }
    false
}

type Semiflow = Vec<Coord>;

#[derive(Clone, Debug)]
struct Flow {
    pair: [Semiflow; 2],
    complete: bool,
    color: u8,
}

impl Flow {
    fn tips(&self) -> [Coord; 2] {
        [*self.pair[0].last().unwrap(), *self.pair[1].last().unwrap()]
    }
}
type Coord = (usize, usize); // row, col

// type Move = (Coord, &mut Flow, usize);

#[derive(Clone, Debug)]
pub struct Solver {
    flows: Vec<Flow>,
    pub board: Board,
}

impl Solver {
    pub fn new(b: &Board) -> Solver {
        let mut flows: Vec<Flow> = vec![];

        //Need to go from a board to a sparse board of heads.
        //Go across the entire board. If you find a head, check if there's already a flow of that color.
        // If so, just add that head to the second element of the pair.
        // If not, create a new pair.
        for i in 0..b.len() {
            if b[i].is_head() {
                let pos: Coord = b.inverse_ind(i);
                if let Some(target) = flows.iter_mut().find(|flow| flow.color == b[i].color()) {
                    target.pair[1] = vec![pos];
                } else {
                    flows.push(Flow {
                        pair: [vec![pos], vec![pos]],
                        complete: false,
                        color: b[i].color(),
                    })
                }
            }
        }

        Self {
            flows,
            board: b.clone(),
        }
    }

    fn forced_move(&self) -> Option<(Coord, usize, usize)> {
        // (where_to_move, which_flow, first_flow_in_pair?)
        let board = &self.board;
        for (i, f) in self.flows.iter().enumerate() {
            for lefty in 0..=1 {
                let sf = f.pair[lefty].clone();
                let moves: Vec<Coord> = Self::moves_from(board, *sf.last().unwrap());
                if moves.len() == 1 && !f.complete {
                    return Some((moves[0], i, lefty));
                }
            }
        }
        return None;
    }

    pub fn binary_step(&mut self) -> Option<(Self, Self)> {
        let board = &self.board;
        let mut other = self.clone();

        let mut fork_info: Option<(Coord, usize, usize, Coord)> = None;

        for (flow_idx, (f_self, f_other)) in self
            .flows
            .iter_mut()
            .zip(other.flows.iter_mut())
            .enumerate()
        {
            for i in 0..=1 {
                let sf = f_self.pair[i].clone();
                let moves = Self::moves_from(board, *sf.last().unwrap());
                if moves.len() == 2 && !f_self.complete {
                    fork_info = Some((moves[0], flow_idx, i, moves[1]));
                    break;
                }
            }
            if fork_info.is_some() {
                break;
            }
        }

        if let Some((m0, flow_idx, i, m1)) = fork_info {
            self.make((m0, flow_idx, i));
            other.make((m1, flow_idx, i));
            Some((self.clone(), other))
        } else {
            // self.board.print();
            // println!("Any moves left? : {}", self.any_moves_left());
            // panic!()
            return None;
        }
    }

    fn any_moves_left(&self) -> bool {
        for f in &self.flows {
            let tips = f.tips();
            for i in 0..=1 {
                if Self::moves_from(&self.board, tips[i]).len() > 0 && !f.complete {
                    return true;
                }
            }
        }
        false
    }

    fn some_blocked_tip(&self) -> bool {
        for f in &self.flows {
            let tips = f.tips();
            for i in 0..=1 {
                if Self::moves_from(&self.board, tips[i]).len() == 0 && !f.complete {
                    return true;
                }
            }
        }
        false
    }

    fn some_pocket(&self) -> bool {
        'L: for i in 0..self.board.rows * self.board.cols {
            if self.board[i].is_empty() {
                let (r, c) = self.board.inverse_ind(i);
                if self.board.empty_neighbors(r, c).iter().flatten().count() == 0 {
                    for f in &self.flows {
                        for i in 0..=1 {
                            if adjacent((r, c), *f.pair[i].last().unwrap()) {
                                continue 'L;
                            }
                        }
                    }
                    return true;
                }
            }
        }
        // let tips = f.tips();
        // for i in 0..=1 {
        //     if Self::moves_from(&self.board, tips[i]).len() == 0 && !f.complete {
        //         return true;
        //     }
        // }

        false
    }

    pub fn moves_from(board: &Board, c: Coord) -> Vec<Coord> {
        let slots = board.empty_neighbors(c.0, c.1);
        slots
            .iter()
            .filter(|result| result.is_some())
            .map(|x| x.unwrap())
            .collect()
    }

    // pub fn solve_board(mut self) -> Board {
    //     while !self.board.is_solved() {
    //         self.timid_step();
    //     }
    //     self.board
    // }

    pub fn timid_step(&mut self) -> bool {
        //True if there is a shouldn't take a split step
        if self.board.is_solved() {
            return true;
        }
        let move_result: Option<(Coord, usize, usize)> = self.forced_move();
        if move_result.is_none() && !self.board.is_full() {
            return false;
        }
        let (loc, flow_idx, i) = move_result.unwrap();
        let flow = &mut self.flows[flow_idx];

        let c = flow.color;
        flow.pair[i].push(loc);
        Self::check_complete(flow);
        self.board.add_path(loc.0, loc.1, c);
        return true;
    }

    fn make(&mut self, m: (Coord, usize, usize)) {
        let (loc, flow_idx, i) = m;
        let flow = &mut self.flows[flow_idx];
        if flow.complete {
            return;
        }
        let c = flow.color;
        flow.pair[i].push(loc);
        Self::check_complete(flow);
        self.board.add_path(loc.0, loc.1, c);
    }

    fn check_complete(flow: &mut Flow) {
        let tips = flow.tips();
        if adjacent(tips[0], tips[1]) {
            flow.complete = true;
        }
    }

    pub fn done(&self) -> bool {
        self.flows.iter().all(|flow| flow.complete)
    }

    pub fn failed(&self) -> bool {
        if self.board.is_solved() {
            return false;
        }
        !self.board.is_legal()
            || self.board.is_full()
            || self.some_blocked_tip()
            || self.some_pocket()
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
}

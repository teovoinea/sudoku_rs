use std::env;
use std::time::Instant;

fn main() {
	//take the first (should also be only) string to parse into a grid
    let arg = env::args().nth(1).unwrap_or(String::from("."));

    let mut grid = Grid::new(); 

    let mut row_index = 0;
    let mut col_index = 0;
    for c in arg.chars(){
    	if c.is_digit(10){
            let mut value = Possible::new();
    		if c != '0'{
                value.set(c.to_digit(10).unwrap() as usize - 1);
    		}
            grid.cells[row_index][col_index] = value;
            col_index += 1;
            if col_index % 9 == 0{
                col_index = 0;
                row_index += 1;
            }
    	}
    }
    println!("This is the parsed grid");
    grid.string();
    let start = Instant::now();
    println!("Starting now: {:?}", start);
    while !grid.is_solved() {
        //for all in grid where we know it _is_ the value
        //call solve on the value
        for row_index in 0..9{
            for col_index in 0..9{
                let current_value = grid.cells[row_index][col_index].solved_value(); 
                if current_value.is_some(){
                    grid.solve(row_index, col_index, current_value.unwrap());
                }
                
            }
        }
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    //print solved grid
    grid.string();
}
#[derive(Copy, Clone)]
struct Possible {
	v: [bool; 9],
}

impl Possible {

	fn new() -> Possible {
		let n_v: [bool; 9] = [true, true, true, true, true, true, true, true, true];
		Possible {
			v: n_v,
		}
	}

    fn count(&self) -> i8 {
    	let mut i = 0;
    	for n in &self.v{
    		if n == &true {
    		    i+=1;
    		}
    	}
    	return i;
    }

    fn set(&mut self, i: usize) {
    	for j in 0..self.v.len() {
    		self.v[j] = false;
    	}
    	self.v[i] = true;
    }

    fn eliminate(&mut self, i: usize) {
        self.v[i] = false;
    }

    fn solved_value(&self) -> Option<usize> {
        if self.count() == 1{
            for i in 0..self.v.len() {
                if self.v[i] {
                    return Some(i+1 as usize);
                }
            }
        }
        return None;
    }
}

struct Grid {
    cells: [[Possible; 9]; 9],
}

impl Grid {
    fn new() -> Grid {
        let n_v: [[Possible; 9]; 9] = [[Possible::new(); 9]; 9];

        Grid {
            cells: n_v,
        }
    }

    fn is_solved(&mut self) -> bool {
        
        for i in 0..9 {
            for j in 0..9 {
                
                if self.cells[i][j].count() > 1 {
                    
                    return false;
                }
            }
        }
        return true;
    }

    fn group(&self, i: usize, j: usize) -> i8 {
        let col_group = ((i) as f32/3.0_f32).floor() as i8;
        let row_group = ((j) as f32/3.0_f32).floor() as i8;
        //TODO: fix later
        if col_group == 0 && row_group == 0 {
            return 0;
        }
        if col_group == 0 && row_group == 1 {
            return 1;
        }
        if col_group == 0 && row_group == 2 {
            return 2;
        }
        if col_group == 1 && row_group == 0 {
            return 3;
        }
        if col_group == 1 && row_group == 1 {
            return 4;
        }
        if col_group == 1 && row_group == 2 {
            return 5;
        }
        if col_group == 2 && row_group == 0 {
            return 6;
        }
        if col_group == 2 && row_group == 1 {
            return 7;
        }
        if col_group == 2 && row_group == 2 {
            return 8;
        }
        panic!("{:?}","Something went terribly wrong");
    }

    //cell[i][j] contains x
    //i is row
    //j is column
    fn solve(&mut self, i: usize, j: usize, x: usize) {

        //eliminate all x from column i
        for a in 0..9 {
            if a != i {
                self.cells[a][j].eliminate(x - 1);
            }
        }

        //eliminate all x from row j
        for b in 0..9 {
            if b != j {
                self.cells[i][b].eliminate(x - 1);
            }
        }

        //eliminate all x from group containing [i][j]
        let c_group = self.group(i, j);
        for a in 0..9 {
            for b in 0..9 {
                let t_group = self.group(a, b);
                if c_group == t_group && !(a == i && b == j) {
                    self.cells[a][b].eliminate(x - 1);
                }
            }
        }
    }

    fn string(&self){
        println!("{:?}", "--------------------------------");
        for i in 0..9 {
            for j in 0..9 {
                if self.cells[i][j].count() == 1 {
                    print!("|   {:?}   |", self.cells[i][j].solved_value().unwrap());
                    
                }
                else{
                    print!("|({:?})", self.cells[i][j].count());
                    print!(" {:?}|", "?");
                }
            }
            println!("{:?}", "");
        }
        println!("{:?}", "--------------------------------");
    }
}
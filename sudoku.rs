use std::env;

fn main() {
	//take the first (should also be only) string to parse into a grid
    let arg = env::args().nth(1).unwrap_or(String::from("."));

    println!("{:?}", arg);

    let mut grid = Grid::new(); 

    //figure out how to add to the grid
    for c in arg.chars(){
    	if c.is_digit(10){
    		if c == '0'{
    			println!("{:?}", "We don't know");
    		}
    		else{
    			println!("{:?}", c);
    		}
    	}
    }

    while !grid.is_solved() {
        //for all in grid where we know it _is_ the value
        //call solve on the value
    }

    //print solved grid
    grid.string();
}
#[derive(Copy, Clone)]
struct Possible {
	v: [bool; 9],
}

impl Possible {

	fn new() -> Possible {
		let mut n_v: [bool; 9] = [false, false, false, false, false, false, false, false, false];
		Possible {
			v: n_v,
		}
	}

    fn is_on(&self, i: usize) -> bool {
    	return self.v[i];
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
    	for i in 0..self.v.len() {
    		self.v[i] = false;
    	}
    	self.v[i] = true;
    }

    fn eliminate(&mut self, i: usize) {
        self.v[i] = false;
    }

    fn solved_value(&self) -> usize {
        for i in 0..self.v.len() {
            if self.v[i] {
                return i+1 as usize;
            }
        }
        panic!("{:?}", "No solved value!");
    }
}

struct Grid {
    cells: [[Possible; 9]; 9],
}

impl Grid {
    fn new() -> Grid {
        let mut n_v: [[Possible; 9]; 9] = [[Possible::new(); 9]; 9];

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


    fn column(&self, i: usize) -> [Possible; 8] {
        let mut r: [Possible; 8] = [Possible::new(); 8];
        for j in 0..9 {
            if j != i {
                r[j] = self.cells[i][j];
            }
        }
        return r;
    }

    fn row(&self, j: usize) -> [Possible; 8] {
        let mut r: [Possible; 8] = [Possible::new(); 8];
        for i in 0..9 {
            if i != j {
                r[i] = self.cells[i][j];
            }
        }
        return r;
    }

    fn group(&self, i: usize, j: usize) -> i8 {
        let col_group = ((i+ 1) as f32/3.0_f32).ceil() as i8;
        let row_group = ((j+ 1) as f32/3.0_f32).ceil() as i8;
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
    fn solve(&mut self, i: usize, j: usize, x: usize) {
        //eliminate all x from column i
        for a in 0..9 {
            if a != i {
                self.cells[a][j].eliminate(x);
            }
        }
        //eliminate all x from row j
        for b in 0..9 {
            if b != j {
                self.cells[i][b].eliminate(x);
            }
        }
        //eliminate all x from group containing [i][j]
        let c_group = self.group(i, j);
        for a in 0..9 {
            for b in 0..9 {
                if c_group == self.group(a, b) && a != i && b != j {
                    self.cells[a][b].eliminate(x);
                }
            }
        }
    }

    fn string(&self){
        for i in 0..9 {
            for j in 0..9 {
                if self.cells[i][j].count() == 1 {
                    print!("{:?}", self.cells[i][j].solved_value());
                }
                else{
                    print!("{:?}", "?");
                }
            }
            println!("{:?}", "");
        }
    }
}
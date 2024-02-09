
use crate::GRID_X;
use crate::GRID_Y;
use crate::files;
use crate::display;
use crate::Renderer;


#[derive(Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {

    pub fn is_between(&self, p1: &Point, p2: &Point) -> bool {

        if self.x >= p1.x && self.y >= p1.y && self.x <= p2.x && self.y <= p2.y {

            return true;
            
        }

        return false
    }

}

#[derive(Clone, Copy)]
pub struct Block {
    pub position: Point,
    pub neighbours: usize,
}

impl Block {
    fn new(pos: Point) -> Self {
        Self {position: pos, neighbours: 0}
    }
    
    // when the addition overflows the Grid its starts at the other end of the screen
    fn add_with_boundries(&self, add_pos: &Point) -> Point {

        let mut p: Point = Point { x: 0, y: 0 };

        if self.position.x + add_pos.x >= GRID_X as i32   {
            
            p.x = self.position.x + add_pos.x - (GRID_X as i32);

        }
        else if self.position.x + add_pos.x < 0 {


            p.x = self.position.x + add_pos.x + (GRID_X as i32);

        }
        else {

            

            p.x = self.position.x + add_pos.x;

        }


        if self.position.y + add_pos.y >= GRID_Y as i32 {


            p.y = self.position.y + add_pos.y - (GRID_Y as i32);

        }
        else if self.position.y + add_pos.y < 0 {


            p.y = self.position.y + add_pos.y + (GRID_Y as i32);
        }
        else {


            p.y = self.position.y + add_pos.y;

        }

        return p

    }

    fn get_neighbours(&mut self, blocks: &Vec<Vec<bool>>) -> Vec<Block> {
        let mut neigh_pos: Point = Point{x: -1, y: -1};

        let mut revived: Vec<Block> = Vec::new();

        self.neighbours = 0;

        loop {
            

            if neigh_pos.x == 0 && neigh_pos.y == 0 {
                neigh_pos.x += 1;

                continue;
            }


            let current_neighbor = self.add_with_boundries(&neigh_pos);

            if blocks[(current_neighbor.y) as usize][(current_neighbor.x) as usize] {
                self.neighbours += 1
            }
            else {
                let mut dead: Block = Block::new(Point{x: current_neighbor.x, y: current_neighbor.y});

                dead.check_revive(&blocks);

                if dead.neighbours == 3 {
                    revived.push(dead)
                }
            }



            neigh_pos.x += 1;

            if neigh_pos.x >= 2 {
                neigh_pos.x = -1;

                neigh_pos.y += 1;
            }

            if neigh_pos.y >= 2 {
                break;
            }
        } 

        return revived
    }
    fn check_revive(&mut self, blocks: &Vec<Vec<bool>>)  {
        let mut neigh_pos: Point = Point{x: -1, y: -1};


        loop {
            

            if neigh_pos.x == 0 && neigh_pos.y == 0 {
                neigh_pos.x += 1;
                continue;
            }


            let current_neighbor = self.add_with_boundries(&neigh_pos);

            if blocks[(current_neighbor.y) as usize][(current_neighbor.x) as usize] {
                self.neighbours += 1
            }
            



            neigh_pos.x += 1;

            if neigh_pos.x >= 2 {
                neigh_pos.x = -1;

                neigh_pos.y += 1;
            }

            if neigh_pos.y >= 2 {
                break;
            }
        }

    }
}

pub fn round(mut render: &mut Renderer, mut alive: Vec<Block>, mut blocks: Vec<Vec<bool>>) -> (Vec<Block>, Vec<Vec<bool>>) {


    let mut temp_blocks = blocks.clone();

    let mut remove: Vec<usize> = Vec::new();
    let mut append: Vec<Block> = Vec::new(); 

    for count in 0..alive.len() {
        let mut revived: Vec<Block> = alive[count].get_neighbours(&blocks);

        files::log(&("revived: ".to_owned() + &revived.len().to_string()));

        for i in 0..revived.len() {

            if !temp_blocks[revived[i].position.y as usize][revived[i].position.x as usize] { 

                // display::write_block(revived[i].position.x as u16, revived[i].position.y as u16);
                
                render.draw_block(revived[i].position.clone()).unwrap();

                temp_blocks[revived[i].position.y as usize][revived[i].position.x as usize] = true;

                append.push(revived[i]);
            }
        }


        //println!("{}", alive[count].neighbours);
        files::log(&("neighbours: ".to_owned() + &alive[count].neighbours.to_string()));

        if alive[count].neighbours > 3 || alive[count].neighbours < 2 {
            // display::delete_block(alive[count].position.x as u16, alive[count].position.y as u16);

            render.remove_block(alive[count].position.clone()).unwrap();

            temp_blocks[alive[count].position.y as usize][alive[count].position.x as usize] = false;

            remove.push(count);
        }
    }

    blocks = temp_blocks;

    remove.sort();
    remove.reverse();

    for count in remove {
        alive.remove(count);
        files::log(&("rm ".to_owned() + &count.to_string()))
    }

    alive.append(&mut append);


    return (alive, blocks);
}

pub fn place_block(mut render: &mut Renderer,pos: Point, mut alive: Vec<Block>, mut blocks: Vec<Vec<bool>>) -> (Vec<Block>, Vec<Vec<bool>>) {
    // display::write_block(pos.x as u16, pos.y as u16);
    
    render.draw_block(pos.clone());

    alive.push(Block::new(Point{x:pos.x,y:pos.y}));
    blocks[pos.y as usize][pos.x as usize] = true;

    return (alive, blocks);
}

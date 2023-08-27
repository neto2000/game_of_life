use termion::{self, raw::IntoRawMode};
use std::io::Write;
use std::{thread, time};

pub mod display;
pub mod files;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Block {
    pub position: Point,
    pub neighbours: usize,
}

impl Block {
    fn new(pos: Point) -> Self {
        Self {position: pos, neighbours: 0}
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



            if blocks[(self.position.y + neigh_pos.y) as usize][(self.position.x + neigh_pos.x) as usize] {
                self.neighbours += 1
            }
            else {
                let mut dead: Block = Block::new(Point{x:self.position.x + neigh_pos.x, y:self.position.y + neigh_pos.y});

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



            if blocks[(self.position.y + neigh_pos.y) as usize][(self.position.x + neigh_pos.x) as usize] {
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

fn main() {

    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}{}", termion::cursor::Goto(1,1), termion::cursor::Hide, termion::clear::All).unwrap();

    stdout.flush().unwrap();


    let (columns, rows) = termion::terminal_size().unwrap();

    files::clear();


    let mut blocks: Vec<Vec<bool>> = Vec::new();

    for i in 0..rows {
        blocks.push(Vec::new());

        for _j in 0..columns {
            blocks[i as usize].push(false);
        }
    }

    let mut alive: Vec<Block> = Vec::new();


    
    let (mut alive, mut blocks) = place_block(Point{x:25,y:25}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:25,y:26}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:25,y:27}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:27,y:24}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:27,y:28}, alive, blocks);
    
    let (mut alive, mut blocks) = place_block(Point{x:28,y:24}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:28,y:28}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:30,y:25}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:30,y:26}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:30,y:27}, alive, blocks);

    let (mut alive, mut blocks) = place_block(Point{x:33,y:25}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:33,y:26}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:33,y:27}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:35,y:24}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:35,y:28}, alive, blocks);
    
    let (mut alive, mut blocks) = place_block(Point{x:36,y:24}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:36,y:28}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:38,y:25}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:38,y:26}, alive, blocks);
    let (mut alive, mut blocks) = place_block(Point{x:38,y:27}, alive, blocks);


    display::write_block(1, 1);
    display::write_block(1, 2);

    stdout.flush().unwrap();

    let mut rounds = 0;

    loop {
        
        files::log(&(alive.len().to_string()));

        thread::sleep(time::Duration::from_millis(500));

        let mut temp_blocks = blocks.clone();

        let mut remove: Vec<usize> = Vec::new();
        let mut append: Vec<Block> = Vec::new(); 

        for count in 0..alive.len() {
            let mut revived: Vec<Block> = alive[count].get_neighbours(&blocks);

            files::log(&("revived: ".to_owned() + &revived.len().to_string()));

            for i in 0..revived.len() {

                if !temp_blocks[revived[i].position.y as usize][revived[i].position.x as usize] { 

                    display::write_block(revived[i].position.x as u16, revived[i].position.y as u16);

                    temp_blocks[revived[i].position.y as usize][revived[i].position.x as usize] = true;

                    append.push(revived[i]);
                }
            }


            //println!("{}", alive[count].neighbours);
            files::log(&("neighbours: ".to_owned() + &alive[count].neighbours.to_string()));

            if alive[count].neighbours > 3 || alive[count].neighbours < 2 {
                display::delete_block(alive[count].position.x as u16, alive[count].position.y as u16);

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

        stdout.flush().unwrap();

        rounds += 1;

        if rounds >= 30 {
            break;
        }
    }

    write!(stdout, "{}{}{}", termion::cursor::Show, termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
}

fn place_block(pos: Point, mut alive: Vec<Block>, mut blocks: Vec<Vec<bool>>) -> (Vec<Block>, Vec<Vec<bool>>) {
    display::write_block(pos.x as u16, pos.y as u16);
    alive.push(Block::new(Point{x:pos.x,y:pos.y}));
    blocks[pos.y as usize][pos.x as usize] = true;

    return (alive, blocks);
}

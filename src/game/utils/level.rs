#!allow[unused]

use super::{obstacle::Obstacle, structs::Entity};

#[derive(Debug)]
pub enum Tile {
    Empty,
    Obstacle,
    Enemy,
    Player,
}

#[derive(Debug)]
pub struct Level {
    name: String,
    grid: Vec<Tile>,
    pub height: i32,
    pub width: i32,
}

impl Level {
    pub fn new(height: i32, width: i32) -> Self {
        let mut tiles: Vec<Tile> = vec![];
        for _i in 0..(height * width) {
            tiles.push(Tile::Empty);
        }
        Level {
            name: String::from("value"),
            grid: tiles,
            height: height,
            width: width,
        }
    }

    pub fn update_grid(
        &mut self,
        player: &Entity,
        obstacles: &Vec<Obstacle>,
        enemies: &Vec<Entity>,
    ) {
        for i in 0..self.height * self.width {
            self.grid[i as usize] = Tile::Empty
        }

        self.grid[(player.pos.0 + (player.pos.1 * self.width)) as usize] = Tile::Player;

        for o in obstacles {
            self.grid[(o.pos.0 + (o.pos.1 * self.width)) as usize] = Tile::Obstacle;
        }

        for e in enemies {
            self.grid[(e.pos.0 + (e.pos.1 * self.width)) as usize] = Tile::Enemy;
        }
    }

    pub fn print_grid(&mut self) {
        let mut w = self.width;
        let mut h = self.height;

        for i in 0..(h * w) {
            if i % w == 0 && i != 0 {
                println!("line {}", i);
            }
            match self.grid[i as usize] {
                Tile::Player => print!("@"),
                Tile::Empty => print!("."),
                Tile::Enemy => print!("E"),
                Tile::Obstacle => print!("█"),
                _ => println!("there has been an error printing the grid"),
            }
        }
    }
}

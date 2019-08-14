use rand::*;
use std::io::{self, BufRead};
use rand::{thread_rng, Rng};

struct Game{
  board: Vec<Vec<u8>>, 
  board_size: usize, 
  water: u32, 
}
impl Game{
  fn new(size: usize) -> Self{
    Game{
      board: vec![vec![0; size]; size], 
      board_size: size, 
      water: 20, 
    }
  }
  fn print_info(&self) {
    print!("idx ");
    for i in 0..self.board_size{
      print!("x{}  ", i);
    }
    println!();
    let mut index = 0;
    for i in self.board.iter(){
      print!("y{}  ", index);
      for j in i.iter(){
        print!("{}   ",j);
      }
      index += 1;
      println!();
      println!();
    }
    println!("Water: {}", self.water);
    println!();
  }
  fn init(&mut self){
    println!("GAME INIT");
    for i in 0..self.board_size{
      for j in 0..self.board_size{
        let mut rng = thread_rng();
        self.board[i][j] = rng.gen_range(0, 5);
      }
    }
  }
  fn reset(&mut self){
    for i in self.board.iter(){
      for j in i.iter(){
        if *j != 0{
          return;
        }
      }
    }
    self.print_info();
    self.init();
  }
  fn add_one_water(&mut self, x: usize, y: usize){
    self.board[y][x] += 1;
    self.water -= 1;
  }
  fn get_input_and_play(&mut self){
    let mut x:usize = 999;
    let mut y:usize = 999;
    let stdin = io::stdin();

    println!("Input position x:");
    let mut choice = String::new();
    stdin.read_line(&mut choice).expect("failed to read from stdin");
    let trimmed = choice.trim();
    match trimmed.parse::<u32>(){
      Ok(i) => {
        x = i as usize;
      }, 
      Err(..) => println!("this was not an integer: {}", trimmed),
    }
    println!("Input position y:");
    let mut choice = String::new();
    stdin.read_line(&mut choice).expect("failed to read from stdin");
    let trimmed = choice.trim();
    match trimmed.parse::<u32>(){
      Ok(i) => {
        y = i as usize;
      }, 
      Err(..) => println!("this was not an integer: {}", trimmed),
    }
    self.add_one_water(x, y);
    println!();
    let count = self.boom(x, y);
    self.water += (count/3) as u32;
    println!("You blew up {} droplets of water", count);
    println!();
  }
  fn game_continue(&self) -> bool{
    self.water != 0
  }
  fn boom(&mut self, x: usize, y: usize) -> u32{
    if self.board[y][x] != 5{
      return 0;
    }
    let mut count = 1;
    /*  Shot water   */
    self.board[y][x] = 0;
    for i in x..self.board_size{
      if self.board[y][i] == 0{
        continue;
      } else {
        self.board[y][i] += 1;
        count += self.boom(i, y);
        break;
      }
    }
    for i in y..self.board_size{
      if self.board[i][x] == 0{
        continue;
      } else {
        self.board[i][x] += 1;
        count += self.boom(x, i);
        break;
      }
    }
    for i in (0..=x).rev(){
      if self.board[y][i] == 0{
        continue;
      } else {
        self.board[y][i] += 1;
        count += self.boom(i, y);
        break;
      }
    }
    for i in (0..=y).rev(){
      if self.board[i][x] == 0{
        continue;
      } else {
        self.board[i][x] += 1;
        count += self.boom(x, i);
        break;
      }
    }
    /*  Shot water   */
    count
  }
}

fn main() {
  let mut game = Game::new(6);
  game.init();
  game.print_info();
  while game.game_continue(){
    game.get_input_and_play();
    game.print_info();
    game.reset();
  }
  for i in 0..=10{
    println!("You lost the game");
  }
}

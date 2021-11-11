use rand::{random, distributions::{Distribution, Standard}, Rng};

#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq)]
enum Block {
    Wall,
    Path,
}

impl Default for Block {
    fn default() -> Self {
        Block::Wall
    }
}

impl Distribution<Block> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Block {
        match rng.gen::<bool>() {
            false => Block::Wall,
            true => Block::Path,
        }
    }
}

fn generate<const X: usize, const Y: usize>(maze: &mut [[Block; X]; Y]) {
    use Block::*;
    maze[0].fill(Path);
    for row in &mut maze[1..] {
        row.fill(Wall);
    }
    for i  in 1..Y {
        for j in 2..X-1 {
            let a: Block = maze[i][j-2];
            let b: Block = maze[i][j-1];
            let c: Block = maze[i-1][j-1];
            let d: Block = maze[i-1][j];
            let e: Block = maze[i-1][j+1];
            // remember: 1 = Wall, 0 = Path
            let next_block = match (a, b, c, d, e) {
                // invariant 1
                (_, Path, Path, Path, _) => Wall,
                (_, Wall, Wall, Wall, _) => Path,
                // invariant 2
                (_, _, Path, Wall, Path) => Wall,
                (_, _, Wall, Path, Wall) => Path,
                (Path, Wall, Path, _, _) => Wall,
                (Wall, Path, Wall, _, _) => Path,
                // invariant 3
                (_, Wall, Path, Path, Wall) => Path,
                // (_, _, Wall, Path, Wall) => Path, // already covered
                // invariant 4
                (Path, Path, Wall, Path, Path) => Path,
                _ => {
                    // random choice
                    random()
                }
            };
            maze[i][j] = next_block;
        }
    }
}

fn main() {
    let mut maze: [[Block; 20]; 20] = Default::default();
    generate(&mut maze);
    for row in maze {
        for col in row {
            match col {
                Block::Path => print!(" "),
                Block::Wall => print!("\u{2588}"),
            }
        }
        println!("");
    }
}

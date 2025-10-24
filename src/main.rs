use clap::Parser;
use sunoku::Board;

/// A Sudoku Puzzle Solver
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Naive method of solving
    #[arg(short, long, default_value_t = false)]
    naive: bool,

    /// Bax's strategy of solving
    #[arg(short, long, default_value_t = false)]
    bax_strat: bool,

    /// The Input values for the board
    #[arg(short, long)]
    file_inputs: String,
}

/// Main function for the solver.
fn main() {
    let args = Args::parse();

    let mut board = Board::default();

    board.load(args.file_inputs);

    if args.naive {
        board.solve(sunoku::SolvingMethod::Naive);
    } else if args.bax_strat {
        board.solve(sunoku::SolvingMethod::BaxStrat);
    } else {
        println!("No algorithm selected, Naive [-n, --naive] or BaxStrat [-b, --bax-strat]")
    }
}

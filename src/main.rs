use structopt::StructOpt;
use std::time::SystemTime;
use std::rc::{Rc};

// used for using the SearchTree interface 
use dogs::searchspace::SearchSpace;


// load tree search algorithms
use dogs::treesearch::algo::iterative_beamsearch::IterativeBeamSearch;
// use dogs::treesearch::algo::dfs::DFS;

// load Decorators
use dogs::treesearch::decorators::pe_dominance::PEDominanceTsDecorator;
use dogs::treesearch::decorators::stats::StatTsDecorator;
use dogs::treesearch::decorators::pruning::PruningDecorator;
use dogs::treesearch::decorators::bounding::BoundingDecorator;

// test logger
use dogs::metriclogger::{MetricLogger};

mod sop;
mod soptree;

#[derive(Debug, StructOpt)]
struct Cli {
    /// path to instance file
    instance: String,
    /// time for the algorithm to run
    t: f32,
}

fn main() {
    // start computing time
    let start_time = SystemTime::now();

    // read command line arguments
    let args = Cli::from_args();

    // create logger
    let logger = Rc::new(MetricLogger::new());

    // create search space
    let mut space = BoundingDecorator::new(
        PEDominanceTsDecorator::new(
            PruningDecorator::new(
                StatTsDecorator::new(
                    soptree::ForwardSearch::new(&args.instance)
                ).bind_logger(Rc::downgrade(&logger))
            )
        )
    ).bind_logger(Rc::downgrade(&logger));

    // explore the SOP search tree
    let mut ts = IterativeBeamSearch::new(&mut space, 1, 2.).bind_logger(Rc::downgrade(&logger));

    // run the search algorithm and display the results afterwards
    logger.display_headers();
    ts.run(|_| start_time.elapsed().unwrap().as_secs_f32() < args.t);
    space.display_statistics();
}

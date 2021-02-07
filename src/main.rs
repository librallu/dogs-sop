use structopt::StructOpt;
use std::time::SystemTime;
use std::rc::{Rc};

// used for using the SearchTree interface 
use dogs::searchspace::SearchSpace;


// load tree search algorithms
use dogs::treesearch::algo::iterative_beamsearch::IterativeBeamSearch;
use dogs::treesearch::algo::pce_beamsearch::IterativePCEBeamSearch;

// load Decorators
use dogs::treesearch::decorators::pe_dominance::PEDominanceTsDecorator;
use dogs::treesearch::decorators::stats::StatTsDecorator;
use dogs::treesearch::decorators::pruning::PruningDecorator;

// test logger
use dogs::metriclogger::{MetricLogger};

mod sop;
mod soptree;

#[derive(Debug, StructOpt)]
struct Cli {
    /// path to instance file
    instance: String,
    /// time in seconds for the algorithm to run
    t: f32,
    /// children expansion type. Values: [total, partial]
    expansion_type: String,
}

fn main() {
    // start computing time
    let start_time = SystemTime::now();

    // read command line arguments
    let args = Cli::from_args();

    // create logger
    let logger = Rc::new(MetricLogger::new());

    // create search space
    let mut space = PEDominanceTsDecorator::new(
        PruningDecorator::new(
            StatTsDecorator::new(
                soptree::ForwardSearch::new(&args.instance)
            ).bind_logger(Rc::downgrade(&logger))
        )
    );

    logger.display_headers();

    // explore the SOP search tree
    if args.expansion_type == "partial" {
        let mut ts = IterativePCEBeamSearch::new(&mut space, 1, 2.).bind_logger(Rc::downgrade(&logger));
        ts.run(|_| start_time.elapsed().unwrap().as_secs_f32() < args.t);
    } else if args.expansion_type == "total" {
        let mut ts = IterativeBeamSearch::new(&mut space, 1, 2.).bind_logger(Rc::downgrade(&logger));
        ts.run(|_| start_time.elapsed().unwrap().as_secs_f32() < args.t);
    } else {
        panic!("expansion_type unknown (use --help for more information about the program parameters)");
    }

    // display the results afterwards
    space.display_statistics();
    
    
}

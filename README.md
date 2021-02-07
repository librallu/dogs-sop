# DOGS-SOP

DOGS implementation for the Sequential Ordering Problem ([reference paper](https://www.researchgate.net/publication/343267812_Tree_search_for_the_Sequential_Ordering_Problem))

This solver is built using the [DOGS framework](https://github.com/librallu/dogs)

# building & running

- compilation: `cargo build --release`
- running an example: `./target/release/dogs-sop insts/R.700.1000.15.sop 30 total`


# components implemented

Two variants are implemented:
 - The “total children expansion'' that generates all possible children from a node at once.
 - The “partial children expansion'' that generates one child at a time.

Both components should provide a similar performance. One can expect the partial expansion to be better on loosely constrained instances and the total expansion on the highly constrained instances.
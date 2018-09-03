/*
inspiration:
https://github.com/scikit-learn/scikit-learn/blob/master/sklearn/tree/
Hasties et al

TODO:
- multiple splitting criteria / impurity functions
- classification tree instead. refactor to make this extension easy.
- best-first + max number of leaves, if it's useful
- cost-complexity pruning instead of CART. sklearn uses basic greedy cart.
  see hasties et. al. pg 308
- use apache arrow; takes tables as training data sources. specify which table
  column is the label
  - since ^ might take a long time to stabilize (unless i spend time to
    contribute) might want to start off with a flatbuffer or at least protobuf
    first
*/

use std::collections::BitVec;

// decision tree classification and regression using the CART training algorithm

// DenseRegressionTrainingData
struct DRTD {
  features: Vec<Vec<f32>>,
  labels: Vec<f32>,
}

// generic params trait should include a validation function
// that takes self and returns either fine or error message (Result)...
// used to configure a CART instance (hyperparameters?)
struct Params {
  // which criterion function to use
  // potentially what kind of splitter to use; seems to be for more generic
    // there's only "best" or "random". not sure what the difference is yet
  // tree method
  // random state seed. this should depend on the rust canonical rng
  // min impurity decrease - however much the tree reduces the impurity before
  // accepting a split
}

struct CART {
  conf: HyperParams,
  tree: Model,
}

// should be a static CTOR basically?
fn train(params: Params, t_data: DRTD) -> CART {
  // depthfirst or bestfirst depends on which regularization criterion is used
  // features are randomly permuted / sampled at each split? dafurq?
  // basic idea is look at feature values, if impurity > min allowed, split node

  // best first build. make this parametrized by stack vs. frontier...
  // basically need a sorted version of each feature with labels first.
  // this is brute-forcey, but fuck it.
  // find best split...

  // first: for every feature, zip and sort

  // depthfirst: until hit given depth, choose highest variance feature
}

// computes variances from sum, squared sum, and count
fn variance(x_sum, x_ssum, x_count) {
  let mu = x_sum / x_count;
  x_ssum / x_count - mu * mu
}

// input: [(feature column, labels)] sorted by feature value
// output: best feature value to split on, variance decrease with split
fn best_variance_split(data: &Vec<(f32, f32)>) -> (f32, f32) {
  let n = data.len();
  let (right_sum, right_ssum) = data.iter()
    .fold((0.0, 0.0), |(s, ss), (x, y)| s + y, ss + y*y);

  let total_var = variance(right_sum, right_ssum, n);

  data.iter().enumerate()
    .scan((0.0, 0.0, right_sum, right_ssum), |state, (i, (x, y))| {
      let (ls, lss, rs, rss) = *state;
      *state = (ls + y, lss + y*y, rs - y, rs - y * y);
      Some((x, variance(ls, lss, i) + variance(rs, rss, n - i - 1))
    })
    .max_by_key(|(x, var)| var)
    .map(|x, var| (x, total_var - var))
}

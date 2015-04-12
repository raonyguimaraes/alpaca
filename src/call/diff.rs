use call::{Caller, GenotypeLikelihoods};
use utils;
use Prob;


pub struct Diff<L: Caller, R: Caller> {
    left: L,
    right: R,
}


impl<L: Caller, R: Caller> Caller for Diff<L, R> {
    fn call(&self, likelihoods: &[GenotypeLikelihoods]) -> Prob {
        // 1 - ((1-p_l) * p_r)
        let p = ((-self.left.call(likelihoods).exp()).ln_1p() + self.right.call(likelihoods));
        (-p.exp()).ln_1p()
    }
}

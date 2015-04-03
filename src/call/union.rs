use call::Caller;


pub struct Union<L: Caller, R: Caller> {
    left: L,
    right: R,
}


impl<L: Caller, R: Caller> Caller for Union<L, R> {
    fn call(&self, likelihoods: &[GenotypeLikelihoods]) -> f64 {
        left.call(likelihoods) + right.call(likelihoods)
    }
}

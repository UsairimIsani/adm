pub struct DNA<T> {
    buf: Box<[Option<T>]>,
    len: usize,
    cap: usize,
}
impl<T> DNA<T> {
    fn new() -> DNA<T> {
        DNA {
            len: 0,
            cap: 0,
            buf: Box::new([None]),
        }
    }
    // fn check_bound()->usize{
    //     unimplemented!
    // }
    // fn insert(index:usize)->
}

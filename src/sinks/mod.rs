pub mod file;

pub trait Sink<T> {
    fn sink(&self, data: T);
}

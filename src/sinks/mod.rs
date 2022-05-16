pub mod file;
pub mod http_post;

pub trait Sink<T> {
    fn sink(&self, data: T);
}

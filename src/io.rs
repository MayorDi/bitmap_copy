pub trait Write<'a>: Sized {
    fn write(&mut self, buf: &'a [u8]) -> std::io::Result<usize>;
}

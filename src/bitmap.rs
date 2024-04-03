use crate::io::Write;
use getset::*;

#[derive(Debug, Default, Clone, Copy, Hash, Eq, Setters, CopyGetters)]
pub struct BitMap<'a> {
    #[getset(get_copy = "pub", set = "pub")]
    width: usize,
    #[getset(get_copy = "pub", set = "pub")]
    height: usize,
    hash: u64,
    body: &'a [u8],
}

impl<'a> BitMap<'a> {
    pub fn new(width: usize, height: usize) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let size_body = width * height;

        unsafe {
            let ptr_body = libc::malloc(size_body);
            let ptr_body = libc::memset(ptr_body, 0, size_body) as *const u8;
            let slice = std::slice::from_raw_parts::<'a, u8>(ptr_body, size_body);

            let mut default_hash = DefaultHasher::new();
            slice.hash(&mut default_hash);

            Self {
                width,
                height,
                hash: default_hash.finish(),
                body: slice,
            }
        }
    }

    /// Creating a hash of the container for faster comparison of BitMaps, 
    /// without their full content comparison.
    /// 
    /// # Examples
    /// ``` rust
    /// use bitmap_copy::BitMap;
    ///
    /// let bit_map1 = BitMap::new(32, 32);
    /// let bit_map2 = BitMap::new(16, 16);
    ///
    /// assert_ne!(bit_map1, bit_map2);
    /// ```
    pub fn hash_update(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut default_hash = DefaultHasher::new();
        self.body.hash(&mut default_hash);
        self.hash = default_hash.finish();
    }

    /// Uploading an image to a container using crate Image.
    pub fn from_img(img: &'a image::DynamicImage) -> std::io::Result<Self> {
        let body = img.as_bytes();
        let (width, height) = (img.width() as usize, img.height() as usize);

        let mut bit_map = BitMap::new(width, height);
        bit_map.write(body)?;
        bit_map.hash_update();

        Ok(bit_map)
    }

    pub fn write_texture_sdl2(&self, buffer: &mut [u8], pitch: usize) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let offset = y * pitch + x * 3;
                let i = (y * self.width() + x) * 4;
                buffer[offset] = self[i];
                buffer[offset + 1] = self[i + 1];
                buffer[offset + 2] = self[i + 2];
            }
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'a, u8> {
        self.body.iter()
    }
}

impl<'a> Iterator for BitMap<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.body.iter().cloned().next()
    }
}

impl<'a> std::io::Read for BitMap<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.body.read(buf)
    }
}

impl<'a> Write<'a> for BitMap<'a> {
    fn write(&mut self, buf: &'a [u8]) -> std::io::Result<usize> {
        self.body = buf;

        self.hash_update();

        Ok(buf.len())
    }
}

impl<'a> std::ops::Index<usize> for BitMap<'a> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.body[index]
    }
}

impl<'a> PartialEq for BitMap<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }

    fn ne(&self, other: &Self) -> bool {
        self.hash != other.hash
    }
}

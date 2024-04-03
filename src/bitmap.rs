use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, ops::Index};

use getset::*;
use image::DynamicImage;
use crate::io::Write;

#[derive(Debug, Default, Clone, Copy, Hash, Setters, CopyGetters)]
pub struct BitMap<'a> {
    #[getset(get_copy = "pub", set = "pub")]
    width: usize,
    #[getset(get_copy = "pub", set = "pub")]
    height: usize,
    hash: u64,
    pub body: &'a [u8],
}

impl<'a> BitMap<'a> {
    pub fn new(width: usize, height: usize) -> Self {
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
                body: slice
            }
        }
    }

    pub fn hash_update(&mut self) {
        let mut default_hash = DefaultHasher::new();
        self.body.hash(&mut default_hash);
        self.hash = default_hash.finish();
    }

    pub fn from_img(img: &'a DynamicImage) -> std::io::Result<Self> {
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
                let i = (y * self.width() + x)*4;
                buffer[offset] = self[i];
                buffer[offset + 1] = self[i + 1];
                buffer[offset + 2] = self[i + 2];
            }
        }
    }
}

impl<'a> Write<'a> for BitMap<'a> {
    fn write(&mut self, buf: &'a [u8]) -> std::io::Result<usize> {
        self.body = buf;

        self.hash_update();

        Ok(buf.len()) 
    }
}

impl<'a> Index<usize> for BitMap<'a> {
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

impl<'a> Eq for BitMap<'a> {}

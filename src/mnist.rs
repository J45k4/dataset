use std::path::Path;

use crate::utility::{download_file, decompress_file};


pub struct MNistLabels {
    index: usize,
    data: Vec<u8>,
    count: u32,
    magic: u32,
}

impl MNistLabels {
    pub async fn from_ubyte<P: AsRef<Path>>(path: P) -> Self {
        let data = tokio::fs::read(path).await.unwrap();

        let magic = u32::from_be_bytes([data[0],data[1],data[2],data[3]]);
        let count = u32::from_be_bytes([data[4],data[5],data[6],data[7]]);

        Self {
            index: 0,
            count,
            magic,
            data,
        }
    }

    pub fn magic(&self) -> u32 {
        self.magic
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn get(&self, index: u32) -> Option<u8> {
        if index >= self.count {
            return None;
        }

        Some(self.data[8 + index as usize])
    }

    pub fn get_batch(&self, index: u32, batch_size: u32) -> Option<&[u8]> {
        if index >= self.count {
            return None;
        }

        let start = 8 + (index * batch_size) as usize;
        let mut end = start + batch_size as usize;

        if end > self.data.len() {
            end = self.data.len();
        }

        Some(&self.data[start..end])
    }

    pub fn increment(&mut self) {
        self.index += 1;
    }
}

pub struct MnistImages {
    data: Vec<u8>,
    count: u32,
    magic: u32,
    width: u32,
    height: u32
}

impl MnistImages {
    pub async fn from_ubyte<P: AsRef<Path>>(path: P) -> Self {
        let data = tokio::fs::read(path).await.unwrap();

        let magic = u32::from_be_bytes([data[0],data[1],data[2],data[3]]);
        let count = u32::from_be_bytes([data[4],data[5],data[6],data[7]]);
        let height = u32::from_be_bytes([data[8],data[9],data[10],data[11]]);
        let width = u32::from_be_bytes([data[12],data[13],data[14],data[15]]);

        Self {
            data,
            count,
            magic,
            width,
            height
        }
    }

    pub fn magic(&self) -> u32 {
        self.magic
    }

    pub fn count(&self) -> u32 {
        self.count
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    pub fn get(&self, index: u32) -> Option<&[u8]> {
        if index >= self.count {
            return None;
        }

        let start = 16 + (index * self.width * self.height) as usize;
        let end = start + (self.width * self.height) as usize;

        if end > self.data.len() {
            log::error!("index: {} start: {} end: {} > data.len(): {}", index, start, end, self.data.len());

            return None;
        }

        Some(&self.data[start..end])
    }

    pub fn get_batch(&self, index: u32, batch_size: u32) -> Option<&[u8]> {
        if index >= self.count {
            return None;
        }

        let start = 16 + (index * batch_size * self.width * self.height) as usize;
        let mut end = start + (batch_size * self.width * self.height) as usize;

        if end > self.data.len() {
            end = self.data.len();
        }

        Some(&self.data[start..end])
    }
}

pub struct Mnist {
    pub train_images: MnistImages,
    pub train_labels: MNistLabels,
    pub test_images: MnistImages,
    pub test_labels: MNistLabels,
}

impl Mnist {
    pub async fn load() -> anyhow::Result<Self> {
        download_file("http://yann.lecun.com/exdb/mnist/train-images-idx3-ubyte.gz", "./datasets/train-images-idx3-ubyte.gz").await?;
        decompress_file("./datasets/train-images-idx3-ubyte.gz","./datasets/train-images-idx3-ubyte").await?;
        download_file("http://yann.lecun.com/exdb/mnist/train-labels-idx1-ubyte.gz", "./datasets/train-labels-idx1-ubyte.gz").await?;
        decompress_file("./datasets/train-labels-idx1-ubyte.gz", "./datasets/train-labels-idx1-ubyte").await?;
        download_file("http://yann.lecun.com/exdb/mnist/t10k-images-idx3-ubyte.gz", "./datasets/t10k-images-idx3-ubyte.gz").await?;
        decompress_file("./datasets/t10k-images-idx3-ubyte.gz", "./datasets/t10k-images-idx3-ubyte").await?;
        download_file("http://yann.lecun.com/exdb/mnist/t10k-labels-idx1-ubyte.gz", "./datasets/t10k-labels-idx1-ubyte.gz").await?;
        decompress_file("./datasets/t10k-labels-idx1-ubyte.gz", "./datasets/t10k-labels-idx1-ubyte").await?;

        Ok(Self {
            train_images: MnistImages::from_ubyte("./datasets/train-images-idx3-ubyte").await,
            train_labels: MNistLabels::from_ubyte("./datasets/train-labels-idx1-ubyte").await,
            test_images: MnistImages::from_ubyte("./datasets/t10k-images-idx3-ubyte").await,
            test_labels: MNistLabels::from_ubyte("./datasets/t10k-labels-idx1-ubyte").await,
        })
    }
}
use std::mem::discriminant;
use std::{fs, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum Block {
    Free,
    File(usize),
}

impl PartialEq<Self> for Block {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Block {}

#[derive(Debug)]
struct Disk {
    disk: Vec<Block>,
}

impl Disk {
    #[cfg(feature = "part1")]
    fn defragment(&mut self) {
        let mut free_ptr = 0;
        let mut file_ptr = self.disk.len() - 1;
        loop {
            // Move free_ptr to free block
            while self.disk[free_ptr] != Block::Free {
                free_ptr += 1;
            }

            // Move file_ptr to file block
            while self.disk[file_ptr] != Block::File(0) {
                file_ptr -= 1;
            }

            if file_ptr <= free_ptr {
                break;
            }

            self.disk.swap(free_ptr, file_ptr);
            // self.print_disk();
        }
    }

    fn print_disk(&self) {
        self.disk.iter().for_each(|block| match block {
            Block::Free => print!("."),
            Block::File(n) => print!("{n}"),
        });
        println!()
    }

    fn checksum(&self) -> usize {
        let mut pos = 0;
        let mut checksum = 0;
        for block in self.disk.iter() {
            match block {
                Block::Free => (),
                Block::File(n) => checksum += n * pos,
            }
            pos += 1;
        }

        checksum
    }

    #[cfg(feature = "part2")]
    fn defragment(&mut self) {
        let mut file_ptr = self.disk.len() - 1;
        loop {
            // Move file_ptr to file block
            while self.disk[file_ptr] != Block::File(0) {
                file_ptr -= 1;
            }

            if file_ptr == 0 {
                break;
            }

            let id = match self.disk[file_ptr] {
                Block::Free => unreachable!(),
                Block::File(id) => id,
            };

            // Find size of file block
            let mut file_size = 1;
            let mut i = 1;
            while let Block::File(n) = self.disk[file_ptr - i] {
                if n != id {
                    break;
                }

                if file_ptr - i == 0 {
                    break;
                }

                file_size += 1;
                i += 1;
            }

            // Search for first space big enough for file size
            let mut size = 0;
            let mut found = false;
            let mut free_ptr = 0;
            for j in 0..file_ptr {
                match self.disk[j] {
                    Block::Free => {
                        size += 1;
                        if size == file_size {
                            found = true;
                            free_ptr = j;
                            break;
                        }
                    }
                    Block::File(_) => {
                        if size != 0 {
                            size = 0;
                        }
                    }
                }
            }

            if found {
                for _ in 0..file_size {
                    self.disk.swap(free_ptr, file_ptr);
                    free_ptr -= 1;
                    file_ptr -= 1;
                }
            } else {
                file_ptr -= file_size;
            }
        }
    }
}

#[derive(Debug)]
struct DiskError;

impl FromStr for Disk {
    type Err = DiskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut block_type = Block::File(id);
        let mut disk = Vec::new();
        for c in s.chars() {
            let n = c.to_digit(10).ok_or(DiskError)?;
            for _ in 0..n {
                disk.push(block_type.clone());
            }
            block_type = match block_type {
                Block::Free => {
                    id += 1;
                    Block::File(id)
                }
                Block::File(_) => Block::Free,
            }
        }

        Ok(Disk { disk })
    }
}

fn main() {
    let content = fs::read_to_string("input/day9").expect("Read input file");
    let mut disk = Disk::from_str(content.trim_end()).expect("Parsing disk information");
    disk.defragment();
    let checksum = disk.checksum();
    println!("Filesystem checksum: {checksum}");
}

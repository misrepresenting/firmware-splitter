use std::{fs, os::unix::prelude::MetadataExt, io, process::Command};

#[derive(Debug)]
pub struct FirmwarePart {
    name: String,
    size: usize,
    offset: usize,
}

impl FirmwarePart {
    pub fn new(name: String, size: usize, offset: usize) -> Self {
        Self { name, size, offset }
    }

    pub fn create_from_offsets(offsets: &[usize]) -> Vec<Self> {
        let mut parts = Vec::new();

        for window in offsets.windows(2) {
            let size = window[1] - window[0];
            parts.push(FirmwarePart::new(window[0].to_string(), size, window[0]));
        }

        return parts
    }

    pub fn carve_from_rom(&self, rom: &Rom) -> io::Result<()> {
        let buffer = fs::read(&rom.path)?;
        let slice = &buffer[self.offset..self.offset+self.size];

        fs::write(&self.name, slice)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Rom {
    pub path: String,
    pub size: u64,
}

impl Rom {
    pub fn new(path: String) -> io::Result<Self> {
        let size = fs::metadata(&path)?.size();
        Ok(Self { path, size })
    }
    
    fn run_binwalk(&self) -> String {
        let binwalk = Command::new("binwalk")
            .arg("-B")
            .arg(&self.path)
            .output()
            .expect("binwalk not working");
        
        // consumes vec and makes it a string
        String::from_utf8(binwalk.stdout).unwrap()
    }

    pub fn get_offsets(&self) -> Vec<usize> {
        let output = self.run_binwalk();
        let offsets = output.lines();
        let mut vec: Vec<usize> = Vec::new();
        
        for offset in offsets {
            if offset.matches(char::is_numeric).next().is_some() {
                let number = offset.split_whitespace().next().unwrap();
                vec.push(number.parse().unwrap());
            }
        }
        vec.push(self.size as usize); // size of the file

        return vec
    }
}

use std::{
    fs::{
        create_dir_all, read as fs_read, read_dir as fs_read_dir, remove_dir_all,
        remove_file as fs_remove_file, write as fs_write, File,
    },
    io::Write,
    path::PathBuf,
};

use crate::system::{DirEntry, DirEntryType};

pub fn create_dir(path: &PathBuf) -> Result<(), ()> {
    match create_dir_all(path) {
        Err(..) => Err(()),
        Ok(..) => Ok(())
    }
}

pub fn read(path: &PathBuf) -> Option<Vec<u8>> {
    fs_read(path).ok()
}

pub fn read_dir(path: &PathBuf) -> Option<Vec<DirEntry>> {
    match fs_read_dir(path) {
        Err(..) => None,
        Ok(entries) => {
            let mut vec: Vec<DirEntry> = Vec::new();

            for e in entries.filter(|el| el.is_ok()).map(|el| el.unwrap()) {
                if let Some(name) = e.file_name().to_str().map(|str| str.to_string()) {
                    if let Ok(typ) = e.file_type() {
                        let is_dir = typ.is_dir();
                        let is_file = typ.is_file();
                        if is_file {
                            vec.push(DirEntry::new(name, DirEntryType::File));
                        } else if is_dir {
                            vec.push(DirEntry::new(name, DirEntryType::Folder));
                        }
                    }
                }
            }

            Some(vec)
        }
    }
}

pub fn remove_dir(path: &PathBuf) -> Result<(), ()> {
    match remove_dir_all(path) {
        Err(..) => Err(()),
        Ok(..) => Ok(())
    }
}

pub fn remove_file(path: &PathBuf) -> Result<(), ()> {
    match fs_remove_file(path) {
        Err(..) => Err(()),
        Ok(..) => Ok(())
    }
}

pub fn write(path: &PathBuf, data: &[u8]) -> Result<(), ()> {
    match fs_write(path, data) {
        Err(..) => Err(()),
        Ok(..) => Ok(())
    }
}

pub struct WritableFile {
    file: File,
}

impl WritableFile {
    fn new(filename: &PathBuf) -> Option<Self> {
        File::create(filename).ok().map(|file| Self { file })
    }
}

impl Write for WritableFile {
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.file.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: std::fmt::Arguments<'_>) -> std::io::Result<()> {
        self.file.write_fmt(fmt)
    }

    fn write_vectored(&mut self, bufs: &[std::io::IoSlice<'_>]) -> std::io::Result<usize> {
        self.file.write_vectored(bufs)
    }
}

pub fn open_file(path: &PathBuf) -> Option<WritableFile> {
    WritableFile::new(path)
}
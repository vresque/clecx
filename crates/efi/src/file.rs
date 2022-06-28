use crate::{
    as_slice, debugln, println,
    proto::Protocol,
    raw::{
        file::{FileInfo, FileMode, RawFile, SimpleFileSystemProtocol},
        guid::FILE_INFO_ID,
    },
    status::{EfiResult, Status},
    wrap_proto, UInt8, UIntN, WString,
};

wrap_proto!(FileSystem wraps SimpleFileSystemProtocol; FILE_SYSTEM_GUID);

impl FileSystem {
    pub fn open_root(&mut self) -> EfiResult<Directory> {
        let mut i = core::ptr::null_mut();
        (self.0.open_volume)(self.0, &mut i);

        Ok(Directory(File(unsafe { &mut *i })))
    }
}

pub struct File(pub &'static mut RawFile);

impl File {
    pub fn new(inner: &'static mut RawFile) -> Self {
        Self(inner)
    }
    // Reads buf.len() bytes into the buffer, returns bytes read
    pub fn read(&mut self, buf: &mut [UInt8]) -> EfiResult<UIntN> {
        let mut bytes = buf.len();
        (self.0.read)(self.0, &mut bytes, buf.as_mut_ptr()).result()?;
        Ok(bytes)
    }

    pub fn get_file_info(&mut self) -> EfiResult<FileInfo> {
        let mut info: FileInfo = FileInfo::default();
        let buf = as_slice!(mut info);
        let mut bytes = buf.len();
        (self.0.get_info)(self.0, &FILE_INFO_ID, &mut bytes, buf.as_mut_ptr()).result()?;
        Ok(info)
    }

    pub fn read_whole(&mut self, buf: &mut crate::alloc::vec::Vec<u8>) -> EfiResult<UIntN> {
        let mut read = 0;

        loop {
            let mut data = [0; 8192];
            let count = self.read(&mut data)?;
            if count == 0 {
                break;
            }
            buf.extend(&data[0..count]);
            read += count;
        }
        Ok(read)
    }

    pub fn rewind(&mut self) -> EfiResult<()> {
        (self.0.set_position)(&mut self.0, 0).result()
    }

    pub fn write_to_file<'buffer>(&mut self, buf: impl Into<&'buffer [u8]>) -> EfiResult<UIntN> {
        let buf = buf.into();
        let mut bytes = buf.len();
        (self.0.write)(self.0, &mut bytes, buf.as_ptr()).result()?;
        Ok(bytes)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        (self.0.close)(self.0);
    }
}

pub struct Directory(File);

impl Directory {
    pub fn open_file(&mut self, name: &WString) -> EfiResult<File> {
        let mut i = core::ptr::null_mut();
        (self.0 .0.open)(
            (self.0).0,
            &mut i,
            WString(name.0),
            FileMode::Read as u64,
            0,
        )
        .result()?;

        Ok(File(unsafe { &mut *i }))
    }

    pub fn open_directory(&'static mut self, name: &WString) -> EfiResult<Directory> {
        let file = self.open_file(name)?;
        Ok(Directory(file))
    }
}

pub struct Searcher;

impl Searcher {
    pub fn find(path: &str) -> EfiResult<File> {
        let path = WString::from(path);
        for mut fs in FileSystem::get_all() {
            let mut root = fs.open_root()?;
            match root.open_file(&path) {
                Ok(f) => {
                    return Ok(f);
                }
                Err(err) => {
                    if err != Status::NotFound {
                        return Err(err);
                    }
                }
            }
        }
        Err(Status::NotFound)
    }

    pub fn load(path: &str) -> EfiResult<crate::alloc::vec::Vec<u8>> {
        let mut file = Self::find(path)?;
        let mut data = crate::alloc::vec![];
        let _ = file.read_whole(&mut data)?;

        Ok(data)
    }
}

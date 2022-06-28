pub type EfiResult<T> = Result<T, Status>;

#[derive(PartialEq, Eq, Debug)]
#[repr(u64)]
pub enum Status {
    Success = 0,
    LoadError = 1,
    InvalidParam = 2,
    Unsupported = 3,
    BadBufferSize = 4,
    BufferTooSmall = 5,
    NotReady = 6,
    DeviceError = 7,
    WriteProtected = 8,
    OutOfResources = 9,
    VolumeCorrupted = 10,
    VolumeFull = 11,
    NoMedia = 12,
    MediaChanged = 13,
    NotFound = 14,
    AccessDenied = 15,
    NoResponse = 16,
    NoMapping = 17,
    Timeout = 18,
    NotStarted = 19,
    AlreadyStarted = 20,
    Aborted = 21,
    IcmpError = 22,
    TftpError = 23,
    ProtocolError = 24,
    IncompatibleVersion = 25,
    SecurityViolation = 26,
    CrcError = 27,
    EndOfMedia = 28,
    EndOfFile = 31,
    InvalidLanguage = 32,
    CompromisedData = 33,
    IpAddressConflict = 34,
    HttpError = 35,
}

impl Status {
    pub fn result(self) -> Result<(), Status> {
        if self == Status::Success {
            Ok(())
        } else {
            Err(self)
        }
    }

    pub fn unwrap(self) {
        self.result().unwrap();
    }
}

use core::mem::ManuallyDrop;

use crate::{alloc::string::String, debugln, print};

#[derive(Clone)]
pub struct WString(pub *const u16);

impl WString {
    //pub fn new(ptr: *const u16) -> Self { Self(ptr) }

    pub fn rustify(&self) -> String {
        let mut str = String::new();
        let mut index = 0;
        loop {
            let char = unsafe { *self.0.offset(index) };
            print!("{}", unsafe { char::from_u32_unchecked(char as u32) });
            index += 1;
            if char == 0 {
                // '\0'
                break;
            }
            str.push(unsafe { char::from_u32_unchecked(char as u32) });
        }
        str
    }

    pub fn from<'a, S>(s: S) -> Self
    where
        S: Into<&'a str>,
    {
        let s: &str = s.into();
        let mut string = ManuallyDrop::new(crate::alloc::vec![]);
        for c in s.chars() {
            string.push(c as u16);
        }
        string.push(0);
        debugln!("{:?}", string);
        Self(string.as_ptr())
    }
}

impl Drop for WString {
    fn drop(&mut self) {
        let mut index: usize = 0;
        loop {
            let char = unsafe { *self.0.offset(index as isize) };
            index += 1;
            if char == 0 {
                // '\0'
                break;
            }
        }
        unsafe { core::mem::drop(core::slice::from_raw_parts(self.0, index).to_vec()) }
    }
}

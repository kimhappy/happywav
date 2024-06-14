use core::ops::DerefMut;
use super::AsU8Slice;

pub struct Saver< D: DerefMut< Target = [u8] > > {
    slice : D,
    offset: usize
}

impl< D: DerefMut< Target = [u8] > > Saver< D > {
    pub fn from(slice: D) -> Self {
        Self {
            slice,
            offset: 0
        }
    }

    pub fn save< T: AsU8Slice + ?Sized >(&mut self, to: &mut T) -> Option< () > {
        let s        = T::as_u8_slice(to);
        let range    = self.offset..self.offset + s.len();
        let write    = self.slice.get_mut(range)?;
        self.offset += s.len();
        write.copy_from_slice(s);
        Some(())
    }

    pub unsafe fn seek_unchecked(&mut self, n: usize) {
        self.offset = n;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.slice.len()
    }
}

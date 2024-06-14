use core::{ mem::MaybeUninit, ops::Deref };
use super::{ AsU8Slice, Pod };

pub struct Loader< D: Deref< Target = [u8] > > {
    slice : D,
    offset: usize
}

impl< D: Deref< Target = [u8] > > Loader< D > {
    pub fn from(slice: D) -> Self {
        Self {
            slice,
            offset: 0
        }
    }

    pub fn load_to< T: AsU8Slice + ?Sized >(&mut self, to: &mut T) -> Option< () > {
        let s        = T::as_mut_u8_slice(to);
        let range    = self.offset..self.offset + s.len();
        let read     = self.slice.get(range)?;
        self.offset += s.len();
        s.copy_from_slice(read);
        Some(())
    }

    pub fn load< T: Pod >(&mut self) -> Option< T > {
        let mut ret = unsafe { MaybeUninit::uninit().assume_init() };
        self.load_to(&mut ret).map(|_| ret)
    }

    pub fn end(&self) -> Option< () > {
        if self.offset == self.slice.len() {
            Some(())
        }
        else {
            None
        }
    }

    pub unsafe fn skip_unchecked(&mut self, n: usize) {
        self.offset += n;
    }

    pub unsafe fn rewind_unchecked(&mut self, n: usize) {
        self.offset -= n;
    }

    pub fn seek(&mut self, n: usize) -> Option< () > {
        if n <= self.slice.len() {
            self.offset = n;
            Some(())
        }
        else {
            None
        }
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

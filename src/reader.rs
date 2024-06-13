use core::ops::Deref;
use crate::{ utils::Loader, common::{ FileFormat, Sample } };

pub struct Reader< D: Deref< Target = [u8] > > {
    loader: Loader< D >,
    format: FileFormat ,
    begin : usize      ,
    end   : usize
}

enum RiffType {
    RIFF,
    RF64(Option< (u64, u64) >)
}

impl< D: Deref< Target = [u8] > > Reader< D > {
    pub fn from(slice: D) -> Option< Reader< D > > {
        let mut loader = Loader::from(slice);

        let riff_id       : [u8; 4] = loader.load()?;
        let riff_file_size: u32     = loader.load()?;
        let riff_format_id: [u8; 4] = loader.load()?;

        if &riff_format_id != b"WAVE" {
            return None
        }

        let mut riff = match &riff_id {
            b"RIFF" if riff_file_size as usize == loader.len() => RiffType::RIFF      ,
            b"RF64" if riff_file_size          == 0xFFFFFFFF   => RiffType::RF64(None),
            _                                                  => return None
        };

        let mut format = Option::< FileFormat     >::None;
        let mut be     = Option::< (usize, usize) >::None;

        while loader.end().is_none() {
            let chunk_id: [u8; 4] = loader.load()?;

            match &chunk_id {
                b"ds64" => {
                    if let RiffType::RIFF = riff {
                        return None
                    }

                    let ds64_chunk_size  : u64 = loader.load  ()?;
                    let ds64_end_offset        = loader.offset() + ds64_chunk_size as usize;
                    let ds64_file_size   : u64 = loader.load  ()?;
                    let ds64_data_size   : u64 = loader.load  ()?;
                    let ds64_sample_count: u64 = loader.load  ()?;

                    if ds64_file_size as usize != loader.len() {
                        return None
                    }

                    riff = RiffType::RF64(Some((ds64_data_size, ds64_sample_count)));

                    if loader.offset() > ds64_end_offset {
                        return None
                    }

                    loader.seek(ds64_end_offset)?;
                },
                b"fmt " => {
                    let fmt_chunk_size  : u32 = loader.load  ()?;
                    let fmt_end_offset        = loader.offset() + fmt_chunk_size as usize;
                    let fmt_audio_format: u16 = loader.load  ()?;
                    let fmt_num_channels: u16 = loader.load  ()?;
                    let fmt_sample_rate : u32 = loader.load  ()?;
                    let fmt_byte_rate   : u32 = loader.load  ()?;
                    let fmt_block_align : u16 = loader.load  ()?;
                    let fmt_bit_depth   : u16 = loader.load  ()?;

                    if fmt_byte_rate != fmt_sample_rate * fmt_num_channels as u32 * fmt_bit_depth as u32 / 8 {
                        return None
                    }

                    if fmt_block_align != fmt_num_channels as u16 * fmt_bit_depth as u16 / 8 {
                        return None
                    }

                    format = Some(FileFormat {
                        sample: match (fmt_audio_format, fmt_bit_depth) {
                            (1,  8) => Sample::U8 ,
                            (1, 16) => Sample::I16,
                            (1, 24) => Sample::I24,
                            (1, 32) => Sample::I32,
                            (3, 32) => Sample::F32,
                            (3, 64) => Sample::F64,
                            _       => return None
                        },
                        num_channels: fmt_num_channels,
                        sample_rate : fmt_sample_rate
                    });

                    if loader.offset() > fmt_end_offset {
                        return None
                    }

                    loader.seek(fmt_end_offset)?;
                },
                b"data" => {
                    let data_chunk_size  : u32 = loader.load  ()?;
                    let data_begin_offset      = loader.offset() ;
                    let data_chunk_size        = match riff {
                        RiffType::RF64(Some((ds64_data_size, ..))) if data_chunk_size == 0xFFFFFFFF => ds64_data_size  as usize,
                        RiffType::RIFF                                                              => data_chunk_size as usize,
                        _                                                                           => return None
                    };
                    let data_end_offset        = data_begin_offset + data_chunk_size;
                    be                         = Some((data_begin_offset, data_begin_offset + data_chunk_size));
                    loader.seek(data_end_offset)?;
                },
                _ => {
                    let chunk_size: u32 = loader.load()?;
                    let end_offset      = loader.offset() + chunk_size as usize;
                    loader.seek(end_offset)?;
                },
            }
        }

        format.zip(be).and_then(|(format, (begin, end))| {
            let blen = end - begin;
            let bs   = format.sample.size() * format.num_channels as usize;

            if blen % bs != 0 {
                return None
            }

            if let RiffType::RF64(Some((_, sample_count))) = riff && sample_count as usize != blen / bs {
                return None
            }

            Some(Reader { loader, format, begin, end })
        })
    }

    pub fn len(&self) -> usize {
        (self.end - self.begin) / self.format.sample.size() as usize
    }

    pub fn format(&self) -> FileFormat {
        self.format
    }

    pub fn skip(&mut self, n: usize) -> Option< () > {
        if self.loader.offset() + n * self.format.sample.size() <= self.end {
            unsafe { self.loader.skip_unchecked(n * self.format.sample.size()) }
            Some(())
        }
        else {
            None
        }
    }

    pub fn rewind(&mut self, n: usize) -> Option< () > {
        if self.loader.offset() >= self.begin + n * self.format.sample.size() {
            unsafe { self.loader.rewind_unchecked(n * self.format.sample.size()) }
            Some(())
        }
        else {
            None
        }
    }

    pub fn seek(&mut self, n: usize) -> Option< () > {
        if self.begin + n * self.format.sample.size() <= self.end {
            unsafe { self.loader.seek_unchecked(self.begin + n * self.format.sample.size()) }
            Some(())
        }
        else {
            None
        }
    }

    pub fn read(&mut self, to: &mut [f32]) -> Option< () > {
        Some(match self.format.sample {
            Sample::U8 => {
                const A: f32 = 2.0 / u8::MAX as f32;

                for x in to.iter_mut() {
                    let value: u8 = self.loader.load()?;
                    *x = (value as f32 * A) - 1.0;
                }
            },
            Sample::I16 => {
                const A: f32 = 1.0 / i16::MAX as f32;

                for x in to.iter_mut() {
                    let value: i16 = self.loader.load()?;
                    *x = value as f32 / A;
                }
            }
            Sample::I24 => {
                const A: f32 = 1.0 / 0x7FFFFF as f32;

                for x in to.iter_mut() {
                    let bytes: [u8; 3] = self.loader.load()?;
                    let third          = (bytes[ 2 ] >> 7) * 0xFF;
                    let value          = i32::from_le_bytes([bytes[ 0 ], bytes[ 1 ], bytes[ 2 ], third]);
                    *x = value as f32 * A;
                }
            }
            Sample::I32 => {
                const A: f32 = 1.0 / i32::MAX as f32;

                for x in to.iter_mut() {
                    let value: i32 = self.loader.load()?;
                    *x = value as f32 * A;
                }
            }
            Sample::F32 => {
                self.loader.load_to(&mut to[..])?;
            }
            Sample::F64 => {
                for x in to.iter_mut() {
                    let value: f64 = self.loader.load()?;
                    *x = value as f32;
                }
            }
        })
    }
}

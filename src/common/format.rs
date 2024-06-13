#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Sample {
    U8 ,
    I16,
    I24,
    I32,
    F32,
    F64
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FileFormat {
    pub sample      : Sample,
    pub num_channels: u16   ,
    pub sample_rate : u32   ,
}

impl Sample {
    pub fn size(&self) -> usize {
        match self {
            Sample::U8  => 1,
            Sample::I16 => 2,
            Sample::I24 => 3,
            Sample::I32 => 4,
            Sample::F32 => 4,
            Sample::F64 => 8,
        }
    }
}

impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sample::U8  => write!(f, "U8" ),
            Sample::I16 => write!(f, "I16"),
            Sample::I24 => write!(f, "I24"),
            Sample::I32 => write!(f, "I32"),
            Sample::F32 => write!(f, "F32"),
            Sample::F64 => write!(f, "F64"),
        }
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}Hz {}ch", self.sample, self.sample_rate, self.num_channels)
    }
}

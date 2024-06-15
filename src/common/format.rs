#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Pcm       = 1,
    IeeeFloat = 3
}

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
    sample      : Sample,
    num_channels: u16   ,
    sample_rate : u32
}

impl AudioFormat {
    pub fn new(n: u16) -> Option< Self > {
        match n {
            1 => Some(AudioFormat::Pcm      ),
            3 => Some(AudioFormat::IeeeFloat),
            _ => None
        }
    }
}

impl Sample {
    pub fn new(audio_format: AudioFormat, bit_depth: u16) -> Option< Self > {
        match (audio_format, bit_depth) {
            (AudioFormat::Pcm      ,  8) => Some(Sample::U8 ),
            (AudioFormat::Pcm      , 16) => Some(Sample::I16),
            (AudioFormat::Pcm      , 24) => Some(Sample::I24),
            (AudioFormat::Pcm      , 32) => Some(Sample::I32),
            (AudioFormat::IeeeFloat, 32) => Some(Sample::F32),
            (AudioFormat::IeeeFloat, 64) => Some(Sample::F64),
            _                            => None
        }
    }

    pub fn depth(&self) -> u16 {
        match self {
            Sample::U8  => 1,
            Sample::I16 => 2,
            Sample::I24 => 3,
            Sample::I32 => 4,
            Sample::F32 => 4,
            Sample::F64 => 8,
        }
    }

    pub fn audio_format(&self) -> AudioFormat {
        match self {
            Sample::U8  => AudioFormat::Pcm      ,
            Sample::I16 => AudioFormat::Pcm      ,
            Sample::I24 => AudioFormat::Pcm      ,
            Sample::I32 => AudioFormat::Pcm      ,
            Sample::F32 => AudioFormat::IeeeFloat,
            Sample::F64 => AudioFormat::IeeeFloat
        }
    }
}

impl FileFormat {
    pub fn new(sample: Sample, num_channels: u16, sample_rate: u32) -> Self {
        Self {
            sample      ,
            num_channels,
            sample_rate
        }
    }

    pub fn sample(&self) -> Sample {
        self.sample
    }

    pub fn num_channels(&self) -> u16 {
        self.num_channels
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn byte_rate(&self) -> u32 {
        self.sample_rate * self.num_channels as u32 * self.sample.depth() as u32 / 8
    }

    pub fn block_align(&self) -> u16 {
        self.num_channels * self.sample.depth() / 8
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

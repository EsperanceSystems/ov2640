/// Camera configuration registers and values
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(dead_code)]
pub(crate) struct Config {
    pub(crate) reg: u8,
    pub(crate) val: u8,
}

/// Struct containing standard delay values used by the camera
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) struct Delays {
    pub POST_RESET_DELAY: u32,
    pub POST_SW_RESET_DELAY: u32,
    pub WRITE_DELAY: u32,
}

// Register configurations
pub(crate) const JPEG_INIT: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x2c,
        val: 0xff,
    },
    Config {
        reg: 0x2e,
        val: 0xdf,
    },
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x3c,
        val: 0x32,
    },
    Config {
        reg: 0x11,
        val: 0x00,
    },
    Config {
        reg: 0x09,
        val: 0x02,
    },
    Config {
        reg: 0x04,
        val: 0x28,
    },
    Config {
        reg: 0x13,
        val: 0xe5,
    },
    Config {
        reg: 0x14,
        val: 0x48,
    },
    Config {
        reg: 0x2c,
        val: 0x0c,
    },
    Config {
        reg: 0x33,
        val: 0x78,
    },
    Config {
        reg: 0x3a,
        val: 0x33,
    },
    Config {
        reg: 0x3b,
        val: 0xfB,
    },
    Config {
        reg: 0x3e,
        val: 0x00,
    },
    Config {
        reg: 0x43,
        val: 0x11,
    },
    Config {
        reg: 0x16,
        val: 0x10,
    },
    Config {
        reg: 0x39,
        val: 0x92,
    },
    Config {
        reg: 0x35,
        val: 0xda,
    },
    Config {
        reg: 0x22,
        val: 0x1a,
    },
    Config {
        reg: 0x37,
        val: 0xc3,
    },
    Config {
        reg: 0x23,
        val: 0x00,
    },
    Config {
        reg: 0x34,
        val: 0xc0,
    },
    Config {
        reg: 0x36,
        val: 0x1a,
    },
    Config {
        reg: 0x06,
        val: 0x88,
    },
    Config {
        reg: 0x07,
        val: 0xc0,
    },
    Config {
        reg: 0x0d,
        val: 0x87,
    },
    Config {
        reg: 0x0e,
        val: 0x41,
    },
    Config {
        reg: 0x4c,
        val: 0x00,
    },
];

pub(crate) const YUV422: &[Config] = &[
    Config {
        reg: 0xFF,
        val: 0x00,
    },
    Config {
        reg: 0x05,
        val: 0x00,
    },
    Config {
        reg: 0xDA,
        val: 0x10,
    },
    Config {
        reg: 0xD7,
        val: 0x03,
    },
    Config {
        reg: 0xDF,
        val: 0x00,
    },
    Config {
        reg: 0x33,
        val: 0x80,
    },
    Config {
        reg: 0x3C,
        val: 0x40,
    },
    Config {
        reg: 0xe1,
        val: 0x77,
    },
    Config {
        reg: 0x00,
        val: 0x00,
    },
];

pub(crate) const JPEG: &[Config] = &[
    Config {
        reg: 0xe0,
        val: 0x14,
    },
    Config {
        reg: 0xe1,
        val: 0x77,
    },
    Config {
        reg: 0xe5,
        val: 0x1f,
    },
    Config {
        reg: 0xd7,
        val: 0x03,
    },
    Config {
        reg: 0xda,
        val: 0x10,
    },
    Config {
        reg: 0xe0,
        val: 0x00,
    },
    Config {
        reg: 0xFF,
        val: 0x01,
    },
    Config {
        reg: 0x04,
        val: 0x08,
    },
];

pub(crate) const RESOLUTION_160X120: &[Config] = &[
    Config {
        reg: 0xFF,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x40,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x43,
    },
    Config {
        reg: 0x19,
        val: 0x00,
    },
    Config {
        reg: 0x1a,
        val: 0x4b,
    },
    Config {
        reg: 0x32,
        val: 0x09,
    },
    Config {
        reg: 0x4f,
        val: 0xca,
    },
    Config {
        reg: 0x50,
        val: 0xa8,
    },
    Config {
        reg: 0x5a,
        val: 0x23,
    },
    Config {
        reg: 0x6d,
        val: 0x00,
    },
    Config {
        reg: 0x39,
        val: 0x12,
    },
    Config {
        reg: 0x35,
        val: 0xda,
    },
    Config {
        reg: 0x22,
        val: 0x1a,
    },
    Config {
        reg: 0x37,
        val: 0xc3,
    },
    Config {
        reg: 0x23,
        val: 0x00,
    },
    Config {
        reg: 0x34,
        val: 0xc0,
    },
    Config {
        reg: 0x36,
        val: 0x1a,
    },
    Config {
        reg: 0x06,
        val: 0x88,
    },
    Config {
        reg: 0x07,
        val: 0xc0,
    },
    Config {
        reg: 0x0d,
        val: 0x87,
    },
    Config {
        reg: 0x0e,
        val: 0x41,
    },
    Config {
        reg: 0x4c,
        val: 0x00,
    },
];

pub(crate) const RESOLUTION_320X240: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x40,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x43,
    },
    Config {
        reg: 0x19,
        val: 0x00,
    },
    Config {
        reg: 0x1a,
        val: 0x4b,
    },
    Config {
        reg: 0x32,
        val: 0x09,
    },
    Config {
        reg: 0x4f,
        val: 0xca,
    },
    Config {
        reg: 0x50,
        val: 0xa8,
    },
    Config {
        reg: 0x5a,
        val: 0x23,
    },
    Config {
        reg: 0x6d,
        val: 0x00,
    },
    Config {
        reg: 0x39,
        val: 0x12,
    },
    Config {
        reg: 0x35,
        val: 0xda,
    },
    Config {
        reg: 0x22,
        val: 0x1a,
    },
    Config {
        reg: 0x37,
        val: 0xc3,
    },
    Config {
        reg: 0x23,
        val: 0x00,
    },
    Config {
        reg: 0x34,
        val: 0xc0,
    },
    Config {
        reg: 0x36,
        val: 0x1a,
    },
    Config {
        reg: 0x06,
        val: 0x88,
    },
    Config {
        reg: 0x07,
        val: 0xc0,
    },
    Config {
        reg: 0x0d,
        val: 0x87,
    },
    Config {
        reg: 0x0e,
        val: 0x41,
    },
    Config {
        reg: 0x4c,
        val: 0x00,
    },
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xe0,
        val: 0x04,
    },
    Config {
        reg: 0xc0,
        val: 0x64,
    },
    Config {
        reg: 0xc1,
        val: 0x4b,
    },
    Config {
        reg: 0x86,
        val: 0x35,
    },
    Config {
        reg: 0x50,
        val: 0x89,
    },
    Config {
        reg: 0x51,
        val: 0xc8,
    },
    Config {
        reg: 0x52,
        val: 0x96,
    },
    Config {
        reg: 0x53,
        val: 0x00,
    },
    Config {
        reg: 0x54,
        val: 0x00,
    },
    Config {
        reg: 0x55,
        val: 0x00,
    },
    Config {
        reg: 0x57,
        val: 0x00,
    },
    Config {
        reg: 0x5a,
        val: 0x50,
    },
    Config {
        reg: 0x5b,
        val: 0x3c,
    },
    Config {
        reg: 0x5c,
        val: 0x00,
    },
];

pub(crate) const RESOLUTION_640X480: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x11,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x00,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x75,
    },
    Config {
        reg: 0x32,
        val: 0x36,
    },
    Config {
        reg: 0x19,
        val: 0x01,
    },
    Config {
        reg: 0x1a,
        val: 0x97,
    },
    Config {
        reg: 0x03,
        val: 0x0f,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x4f,
        val: 0xbb,
    },
    Config {
        reg: 0x50,
        val: 0x9c,
    },
    Config {
        reg: 0x5a,
        val: 0x57,
    },
    Config {
        reg: 0x6d,
        val: 0x80,
    },
    Config {
        reg: 0x3d,
        val: 0x34,
    },
    Config {
        reg: 0x39,
        val: 0x02,
    },
    Config {
        reg: 0x35,
        val: 0x88,
    },
    Config {
        reg: 0x22,
        val: 0x0a,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x34,
        val: 0xa0,
    },
    Config {
        reg: 0x06,
        val: 0x02,
    },
    Config {
        reg: 0x0d,
        val: 0xb7,
    },
    Config {
        reg: 0x0e,
        val: 0x01,
    },
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xe0,
        val: 0x04,
    },
    Config {
        reg: 0xc0,
        val: 0xc8,
    },
    Config {
        reg: 0xc1,
        val: 0x96,
    },
    Config {
        reg: 0x86,
        val: 0x3d,
    },
    Config {
        reg: 0x50,
        val: 0x89,
    },
    Config {
        reg: 0x51,
        val: 0x90,
    },
    Config {
        reg: 0x52,
        val: 0x2c,
    },
    Config {
        reg: 0x53,
        val: 0x00,
    },
    Config {
        reg: 0x54,
        val: 0x00,
    },
    Config {
        reg: 0x55,
        val: 0x88,
    },
    Config {
        reg: 0x57,
        val: 0x00,
    },
    Config {
        reg: 0x5a,
        val: 0xa0,
    },
    Config {
        reg: 0x5b,
        val: 0x78,
    },
    Config {
        reg: 0x5c,
        val: 0x00,
    },
    Config {
        reg: 0xd3,
        val: 0x04,
    },
];

pub(crate) const RESOLUTION_800X600: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x11,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x00,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x75,
    },
    Config {
        reg: 0x32,
        val: 0x36,
    },
    Config {
        reg: 0x19,
        val: 0x01,
    },
    Config {
        reg: 0x1a,
        val: 0x97,
    },
    Config {
        reg: 0x03,
        val: 0x0f,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x4f,
        val: 0xbb,
    },
    Config {
        reg: 0x50,
        val: 0x9c,
    },
    Config {
        reg: 0x5a,
        val: 0x57,
    },
    Config {
        reg: 0x6d,
        val: 0x80,
    },
    Config {
        reg: 0x3d,
        val: 0x34,
    },
    Config {
        reg: 0x39,
        val: 0x02,
    },
    Config {
        reg: 0x35,
        val: 0x88,
    },
    Config {
        reg: 0x22,
        val: 0x0a,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x34,
        val: 0xa0,
    },
    Config {
        reg: 0x06,
        val: 0x02,
    },
    Config {
        reg: 0x0d,
        val: 0xb7,
    },
    Config {
        reg: 0x0e,
        val: 0x01,
    },
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xe0,
        val: 0x04,
    },
    Config {
        reg: 0xc0,
        val: 0xc8,
    },
    Config {
        reg: 0xc1,
        val: 0x96,
    },
    Config {
        reg: 0x86,
        val: 0x35,
    },
    Config {
        reg: 0x50,
        val: 0x89,
    },
    Config {
        reg: 0x51,
        val: 0x90,
    },
    Config {
        reg: 0x52,
        val: 0x2c,
    },
    Config {
        reg: 0x53,
        val: 0x00,
    },
    Config {
        reg: 0x54,
        val: 0x00,
    },
    Config {
        reg: 0x55,
        val: 0x88,
    },
    Config {
        reg: 0x57,
        val: 0x00,
    },
    Config {
        reg: 0x5a,
        val: 0xc8,
    },
    Config {
        reg: 0x5b,
        val: 0x96,
    },
    Config {
        reg: 0x5c,
        val: 0x00,
    },
    Config {
        reg: 0xd3,
        val: 0x02,
    },
];

pub(crate) const RESOLUTION_1024X768: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x11,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x00,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x75,
    },
    Config {
        reg: 0x32,
        val: 0x36,
    },
    Config {
        reg: 0x19,
        val: 0x01,
    },
    Config {
        reg: 0x1a,
        val: 0x97,
    },
    Config {
        reg: 0x03,
        val: 0x0f,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x4f,
        val: 0xbb,
    },
    Config {
        reg: 0x50,
        val: 0x9c,
    },
    Config {
        reg: 0x5a,
        val: 0x57,
    },
    Config {
        reg: 0x6d,
        val: 0x80,
    },
    Config {
        reg: 0x3d,
        val: 0x34,
    },
    Config {
        reg: 0x39,
        val: 0x02,
    },
    Config {
        reg: 0x35,
        val: 0x88,
    },
    Config {
        reg: 0x22,
        val: 0x0a,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x34,
        val: 0xa0,
    },
    Config {
        reg: 0x06,
        val: 0x02,
    },
    Config {
        reg: 0x0d,
        val: 0xb7,
    },
    Config {
        reg: 0x0e,
        val: 0x01,
    },
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc0,
        val: 0xc8,
    },
    Config {
        reg: 0xc1,
        val: 0x96,
    },
    Config {
        reg: 0x8c,
        val: 0x00,
    },
    Config {
        reg: 0x86,
        val: 0x3d,
    },
    Config {
        reg: 0x50,
        val: 0x00,
    },
    Config {
        reg: 0x51,
        val: 0x90,
    },
    Config {
        reg: 0x52,
        val: 0x2c,
    },
    Config {
        reg: 0x53,
        val: 0x00,
    },
    Config {
        reg: 0x54,
        val: 0x00,
    },
    Config {
        reg: 0x55,
        val: 0x88,
    },
    Config {
        reg: 0x5a,
        val: 0x00,
    },
    Config {
        reg: 0x5b,
        val: 0xc0,
    },
    Config {
        reg: 0x5c,
        val: 0x01,
    },
    Config {
        reg: 0xd3,
        val: 0x02,
    },
];

pub(crate) const RESOLUTION_1280X960: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x01,
    },
    Config {
        reg: 0x11,
        val: 0x01,
    },
    Config {
        reg: 0x12,
        val: 0x00,
    },
    Config {
        reg: 0x17,
        val: 0x11,
    },
    Config {
        reg: 0x18,
        val: 0x75,
    },
    Config {
        reg: 0x32,
        val: 0x36,
    },
    Config {
        reg: 0x19,
        val: 0x01,
    },
    Config {
        reg: 0x1a,
        val: 0x97,
    },
    Config {
        reg: 0x03,
        val: 0x0f,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x4f,
        val: 0xbb,
    },
    Config {
        reg: 0x50,
        val: 0x9c,
    },
    Config {
        reg: 0x5a,
        val: 0x57,
    },
    Config {
        reg: 0x6d,
        val: 0x80,
    },
    Config {
        reg: 0x3d,
        val: 0x34,
    },
    Config {
        reg: 0x39,
        val: 0x02,
    },
    Config {
        reg: 0x35,
        val: 0x88,
    },
    Config {
        reg: 0x22,
        val: 0x0a,
    },
    Config {
        reg: 0x37,
        val: 0x40,
    },
    Config {
        reg: 0x34,
        val: 0xa0,
    },
    Config {
        reg: 0x06,
        val: 0x02,
    },
    Config {
        reg: 0x0d,
        val: 0xb7,
    },
    Config {
        reg: 0x0e,
        val: 0x01,
    },
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xe0,
        val: 0x04,
    },
    Config {
        reg: 0xc0,
        val: 0xc8,
    },
    Config {
        reg: 0xc1,
        val: 0x96,
    },
    Config {
        reg: 0x86,
        val: 0x3d,
    },
    Config {
        reg: 0x50,
        val: 0x00,
    },
    Config {
        reg: 0x51,
        val: 0x90,
    },
    Config {
        reg: 0x52,
        val: 0x2c,
    },
    Config {
        reg: 0x53,
        val: 0x00,
    },
    Config {
        reg: 0x54,
        val: 0x00,
    },
    Config {
        reg: 0x55,
        val: 0x88,
    },
    Config {
        reg: 0x57,
        val: 0x00,
    },
    Config {
        reg: 0x5a,
        val: 0x40,
    },
    Config {
        reg: 0x5b,
        val: 0xf0,
    },
    Config {
        reg: 0x5c,
        val: 0x01,
    },
    Config {
        reg: 0xd3,
        val: 0x02,
    },
    Config {
        reg: 0xe0,
        val: 0x00,
    },
];

pub(crate) const SPECIAL_EFFECT_NORMAL: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
];

pub(crate) const SPECIAL_EFFECT_ANTIQUE: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
    Config {
        reg: 0x7d,
        val: 0xa6,
    },
];

pub(crate) const SPECIAL_EFFECT_BW: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
];

pub(crate) const SPECIAL_EFFECT_NEGATIVE: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
];

pub(crate) const SPECIAL_EFFECT_BW_NEGATIVE: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x58,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
    Config {
        reg: 0x7d,
        val: 0x80,
    },
];

pub(crate) const SPECIAL_EFFECT_BLUISH: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0xa0,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
];

pub(crate) const SPECIAL_EFFECT_GREENISH: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
];

pub(crate) const SPECIAL_EFFECT_REDDISH: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7c,
        val: 0x05,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
    Config {
        reg: 0x7d,
        val: 0xc0,
    },
];

pub(crate) const LIGHT_MODE_AUTO: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc7,
        val: 0x00,
    },
];

pub(crate) const LIGHT_MODE_SUNNY: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc7,
        val: 0x40,
    },
    Config {
        reg: 0xcc,
        val: 0x5e,
    },
    Config {
        reg: 0xcd,
        val: 0x41,
    },
    Config {
        reg: 0xce,
        val: 0x54,
    },
];

pub(crate) const LIGHT_MODE_CLOUDY: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc7,
        val: 0x40,
    },
    Config {
        reg: 0xcc,
        val: 0x65,
    },
    Config {
        reg: 0xcd,
        val: 0x41,
    },
    Config {
        reg: 0xce,
        val: 0x4f,
    },
];

pub(crate) const LIGHT_MODE_OFFICE: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc7,
        val: 0x40,
    },
    Config {
        reg: 0xcc,
        val: 0x52,
    },
    Config {
        reg: 0xcd,
        val: 0x41,
    },
    Config {
        reg: 0xce,
        val: 0x66,
    },
];

pub(crate) const LIGHT_MODE_HOME: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0xc7,
        val: 0x40,
    },
    Config {
        reg: 0xcc,
        val: 0x42,
    },
    Config {
        reg: 0xcd,
        val: 0x3f,
    },
    Config {
        reg: 0xce,
        val: 0x71,
    },
];

pub(crate) const BRIGHTNESS_M2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x09,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
];

pub(crate) const BRIGHTNESS_M1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x09,
    },
    Config {
        reg: 0x7d,
        val: 0x10,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
];

pub(crate) const BRIGHTNESS_0: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x09,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
];

pub(crate) const BRIGHTNESS_P1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x09,
    },
    Config {
        reg: 0x7d,
        val: 0x30,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
];

pub(crate) const BRIGHTNESS_P2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x09,
    },
    Config {
        reg: 0x7d,
        val: 0x40,
    },
    Config {
        reg: 0x7d,
        val: 0x00,
    },
];

pub(crate) const CONTRAST_M2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x07,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x18,
    },
    Config {
        reg: 0x7d,
        val: 0x34,
    },
];

pub(crate) const CONTRAST_M1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x07,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x1c,
    },
    Config {
        reg: 0x7d,
        val: 0x2a,
    },
];

pub(crate) const CONTRAST_0: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x07,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
];

pub(crate) const CONTRAST_P1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x07,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x24,
    },
    Config {
        reg: 0x7d,
        val: 0x16,
    },
];

pub(crate) const CONTRAST_P2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x04,
    },
    Config {
        reg: 0x7c,
        val: 0x07,
    },
    Config {
        reg: 0x7d,
        val: 0x20,
    },
    Config {
        reg: 0x7d,
        val: 0x28,
    },
    Config {
        reg: 0x7d,
        val: 0x0c,
    },
];

pub(crate) const SATURATION_M2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x02,
    },
    Config {
        reg: 0x7c,
        val: 0x03,
    },
    Config {
        reg: 0x7d,
        val: 0x28,
    },
    Config {
        reg: 0x7d,
        val: 0x28,
    },
];

pub(crate) const SATURATION_M1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x02,
    },
    Config {
        reg: 0x7c,
        val: 0x03,
    },
    Config {
        reg: 0x7d,
        val: 0x38,
    },
    Config {
        reg: 0x7d,
        val: 0x38,
    },
];

pub(crate) const SATURATION_0: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x02,
    },
    Config {
        reg: 0x7c,
        val: 0x03,
    },
    Config {
        reg: 0x7d,
        val: 0x48,
    },
    Config {
        reg: 0x7d,
        val: 0x48,
    },
];

pub(crate) const SATURATION_P1: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x02,
    },
    Config {
        reg: 0x7c,
        val: 0x03,
    },
    Config {
        reg: 0x7d,
        val: 0x58,
    },
    Config {
        reg: 0x7d,
        val: 0x58,
    },
];

pub(crate) const SATURATION_P2: &[Config] = &[
    Config {
        reg: 0xff,
        val: 0x00,
    },
    Config {
        reg: 0x7c,
        val: 0x00,
    },
    Config {
        reg: 0x7d,
        val: 0x02,
    },
    Config {
        reg: 0x7c,
        val: 0x03,
    },
    Config {
        reg: 0x7d,
        val: 0x68,
    },
    Config {
        reg: 0x7d,
        val: 0x68,
    },
];

/// Default configuration delays for initialization and register writes (in nanoseconds)
pub(crate) const DELAYS: Delays = Delays {
    /// Delay after hardware reset
    POST_RESET_DELAY: 100_000_000, // 100ms
    /// Delay after software reset
    POST_SW_RESET_DELAY: 100_000_000, // 100ms
    /// Delay between register writes
    WRITE_DELAY: 10_000_000, // 10ms
};

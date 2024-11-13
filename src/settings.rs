/// Special effects options for image processing
#[derive(Debug, Clone, Copy)]
pub enum SpecialEffect {
    Normal,
    Antique,
    Bluish,
    Greenish,
    Reddish,
    BlackWhite,
    Negative,
    BlackWhiteNegative,
}

/// Light mode settings for white balance
#[derive(Debug, Clone, Copy)]
pub enum LightMode {
    Auto,
    Sunny,
    Cloudy,
    Office,
    Home,
}

/// Level settings for brightness/contrast/saturation
#[derive(Debug, Clone, Copy)]
pub enum Level {
    VeryLow,
    Low,
    Normal,
    High,
    VeryHigh,
}

/// Resolution options
#[derive(Debug, Clone, Copy)]
pub enum Resolution {
    _160x120,
    _320x240,
    _640x480,
    _800x600,
    _1024x768,
    _1280x960,
}

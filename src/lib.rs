#[cfg(test)]
mod test {
    use super::SoilTexture;

    fn create_texture(name: &str, clay: f64, sand: f64, silt: f64) -> SoilTexture {
        match name {
            "sands" => SoilTexture::Sands { clay, sand, silt },
            "sandyclay" => SoilTexture::SandyClay { clay, sand, silt },
            "sandyloam" => SoilTexture::SandyLoams { clay, sand, silt },
            "sandyclayloam" => SoilTexture::SandyClayLoam { clay, sand, silt },

            "loam" => SoilTexture::Loam { clay, sand, silt },
            "loamysands" => SoilTexture::LoamSands { clay, sand, silt },

            "silt" => SoilTexture::Silt { clay, sand, silt },
            "siltloam" => SoilTexture::SiltLoam { clay, sand, silt },
            "siltyclay" => SoilTexture::SiltyClay { clay, sand, silt },
            "siltyclayloam" => SoilTexture::SiltyClayLoam { clay, sand, silt },

            "clay" => SoilTexture::Clay { clay, sand, silt },
            "clayloam" => SoilTexture::ClayLoam { clay, sand, silt },
            _ => panic!("unknown texture"),
        }
    }

    fn assert_boundaries(boundaries: Vec<(&str, f64, f64, f64)>) {
        boundaries.iter().for_each(|boundary| {
            let (name, clay, sand, silt) = boundary.clone();

            assert_eq!(
                create_texture(name, clay, sand, silt),
                SoilTexture::new(clay, sand, silt).unwrap()
            );
        });
    }

    #[test]
    fn test_clay() {
        assert_boundaries(vec![
            ("clay", 1.0, 0.0, 0.0),        // top
            ("clay", 0.5501, 0.4499, 0.0),  // left
            ("clay", 0.6001, 0.0, 0.3999),  // right
            ("clay", 0.40, 0.4499, 0.1501), // bottom left
            ("clay", 0.40, 0.2001, 0.3999), // bottom right
        ]);
    }

    #[test]
    fn test_sandy_clay() {
        assert_boundaries(vec![
            ("sandyclay", 0.55, 0.45, 0.0),  // top
            ("sandyclay", 0.35, 0.65, 0.0),  // bottom left
            ("sandyclay", 0.35, 0.45, 0.20), // bottom right
        ]);
    }

    #[test]
    fn test_silty_clay() {
        assert_boundaries(vec![
            ("siltyclay", 0.6, 0.0, 0.4), // top
            ("siltyclay", 0.4, 0.2, 0.4), // bottom left
            ("siltyclay", 0.4, 0.0, 0.6), // bottom right
        ]);
    }

    #[test]
    fn test_sandy_clay_loam() {
        assert_boundaries(vec![
            ("sandyclayloam", 0.3499, 0.6501, 0.0),  // top left
            ("sandyclayloam", 0.3499, 0.4501, 0.20), // top right
            ("sandyclayloam", 0.2799, 0.4501, 0.27), // right
            ("sandyclayloam", 0.20, 0.80, 0.0),      // bottom left
            ("sandyclayloam", 0.20, 0.53, 0.27),     // bottom right
        ]);
    }

    #[test]
    fn test_clay_loam() {
        assert_boundaries(vec![
            ("clayloam", 0.3999, 0.4499, 0.1502), // top left
            ("clayloam", 0.3999, 0.2001, 0.40),   // top right
            ("clayloam", 0.27, 0.4499, 0.2801),   // bottom left
            ("clayloam", 0.27, 0.2001, 0.5299),   // bottom right
        ]);
    }

    #[test]
    fn test_silty_clay_loam() {
        assert_boundaries(vec![
            ("siltyclayloam", 0.3999, 0.0, 0.6001),  // top right
            ("siltyclayloam", 0.3999, 0.20, 0.4001), // top left
            ("siltyclayloam", 0.27, 0.0, 0.73),      // bottom right
            ("siltyclayloam", 0.27, 0.20, 0.53),     // bottom left
        ]);
    }

    #[test]
    fn test_sands() {
        assert_boundaries(vec![
            ("sands", 0.0, 1.0, 0.0),       // bottom left
            ("sands", 0.0, 0.8501, 0.1499), // bottom right
            ("sands", 0.0999, 0.9001, 0.0), // top
        ]);
    }

    #[test]
    fn test_loamy_sands() {
        assert_boundaries(vec![
            ("loamysands", 0.1499, 0.8501, 0.0), // top
            ("loamysands", 0.10, 0.90, 0.0),     // left
            ("loamysands", 0.0, 0.85, 0.15),     // bottom left
            ("loamysands", 0.0, 0.7001, 0.2999), // bottom right
        ]);
    }

    #[test]
    fn test_sandy_loam() {
        assert_boundaries(vec![
            ("sandyloam", 0.1999, 0.8001, 0.0),    // top left
            ("sandyloam", 0.1999, 0.5301, 0.27),   // top right
            ("sandyloam", 0.15, 0.85, 0.0),        // left
            ("sandyloam", 0.0699, 0.5301, 0.40),   // middle right
            ("sandyloam", 0.0699, 0.4302, 0.4999), // middle far right
            ("sandyloam", 0.0, 0.70, 0.30),        // bottom left
            ("sandyloam", 0.0, 0.5001, 0.4999),    // bottom right
        ]);
    }

    #[test]
    fn test_loam() {
        assert_boundaries(vec![
            ("loam", 0.2699, 0.45, 0.2801),   // top left
            ("loam", 0.2699, 0.2311, 0.4999), // top right
            ("loam", 0.20, 0.52, 0.28),       // left
            ("loam", 0.07, 0.52, 0.41),       // bottom left
            ("loam", 0.0701, 0.43, 0.4999),   // bottom right
        ]);
    }

    #[test]
    fn test_silt_loam() {
        assert_boundaries(vec![
            ("siltloam", 0.2699, 0.2399, 0.50), // top left
            ("siltloam", 0.2699, 0.0, 0.7301),  // top right
            ("siltloam", 0.12, 0.0801, 0.7999), // middle right
            ("siltloam", 0.12, 0.0, 0.88),      // middle far right
            ("siltloam", 0.0, 0.50, 0.50),      // bottom left
            ("siltloam", 0.0, 0.2001, 0.7999),  // bottom right
        ]);
    }

    #[test]
    fn test_silt() {
        assert_boundaries(vec![
            ("silt", 0.0, 0.0, 1.0),        // bottom right
            ("silt", 0.0, 0.20, 0.80),      // bottom left
            ("silt", 0.1199, 0.0, 0.8801),  // top right
            ("silt", 0.1199, 0.0801, 0.80), // top left
        ]);
    }
}

#[derive(Debug, Clone)]
pub enum SoilTextureError {
    InvalidValues {
        sum: f64,
        clay: f64,
        sand: f64,
        silt: f64,
    },
    Unknown {
        clay: f64,
        sand: f64,
        silt: f64,
    },
}

use std::fmt;

impl fmt::Display for SoilTextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SoilTextureError::InvalidValues { clay, sand , silt, sum } => write!(f, "the sum ({}) must equal 1.0 with values clay ({}), sand ({}) and silt ({})", sum, clay, sand, silt),
            SoilTextureError::Unknown { clay, sand , silt } => write!(f, "soil texture could not be determined with values clay ({}), sand ({}) and silt ({})", clay, sand, silt)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SoilTexture {
    Clay { clay: f64, sand: f64, silt: f64 },
    ClayLoam { clay: f64, sand: f64, silt: f64 },
    Loam { clay: f64, sand: f64, silt: f64 },
    LoamSands { clay: f64, sand: f64, silt: f64 },
    Silt { clay: f64, sand: f64, silt: f64 },
    SiltyClay { clay: f64, sand: f64, silt: f64 },
    SiltLoam { clay: f64, sand: f64, silt: f64 },
    SiltyClayLoam { clay: f64, sand: f64, silt: f64 },
    Sands { clay: f64, sand: f64, silt: f64 },
    SandyLoams { clay: f64, sand: f64, silt: f64 },
    SandyClay { clay: f64, sand: f64, silt: f64 },
    SandyClayLoam { clay: f64, sand: f64, silt: f64 },
}

impl SoilTexture {
    /// Returns a new soil texture enum determined by the
    ///
    /// The sum of sand, silt and clay must be equal to 1.0 when truncated to
    /// two decimal places.
    ///
    /// # Arguments
    ///
    /// * `sand` - The percentage of sand in the sample (0.0 - 1.0)
    /// * `silt` - The percentage of silt in the sample (0.0 - 1.0)
    /// * `clay` - The percentage of clay in the sample (0.0 - 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use soil::SoilTexture;
    ///
    /// let texture = SoilTexture::new(0.0, 0.0, 1.0).unwrap();
    ///
    /// match texture {
    ///     SoilTexture::Clay { .. } => println!("clay"),
    ///     _ => println!("not clay"),
    /// }
    ///
    /// ```
    pub fn new(clay: f64, sand: f64, silt: f64) -> Result<SoilTexture, SoilTextureError> {
        let sum = ((sand + silt + clay) * 100.0).trunc() / 100.0;
        if sum > 1.0 || sum < 1.0 {
            return Err(SoilTextureError::InvalidValues {
                sum,
                clay,
                sand,
                silt,
            });
        }

        if sand > 0.85 && (silt + (1.5 * clay)) < 0.15 {
            // Sands: More than 85 percent sand, the percentage of silt plus 1.5 times the percentage of clay is less than 15.
            return Ok(SoilTexture::Sands { clay, sand, silt });
        } else if clay >= 0.35 && sand >= 0.45 {
            // Sandy clay: 35 percent or more clay and 45 percent or more sand.
            return Ok(SoilTexture::SandyClay { clay, sand, silt });
        } else if (clay >= 0.20 && clay < 0.35) && silt < 0.28 && sand > 0.45 {
            // Sandy clay loam: 20 to 35 percent clay, less than 28 percent silt, and more than 45 percent sand.
            return Ok(SoilTexture::SandyClayLoam { clay, sand, silt });
        } else if (clay >= 0.07 && clay < 0.27) && (silt >= 0.28 && silt < 0.50) && sand <= 0.52 {
            // Loam: 7 to 27 percent clay, 28 to 50 percent silt, and 52 percent or less sand.
            return Ok(SoilTexture::Loam { clay, sand, silt });
        } else if (sand >= 0.70 && sand < 0.91)
            && (silt + (1.5 * clay) >= 0.15)
            && (silt + (2.0 * clay) < 0.30)
        {
            // Loamy sands: Between 70 and 91 percent sand and the percentage of silt plus 1.5 times the percentage of clay is 15 or more; and the percentage of silt plus twice the percentage of clay is less than 30.
            return Ok(SoilTexture::LoamSands { clay, sand, silt });
        } else if clay >= 0.40 && sand <= 0.45 && silt < 0.40 {
            // Clay: 40 percent or more clay, 45 percent or less sand, and less than 40 percent silt.
            return Ok(SoilTexture::Clay { clay, sand, silt });
        } else if (clay >= 0.27 && clay < 0.40) && (sand > 0.20 && sand < 0.45) {
            // Clay loam: 27 to 40 percent clay and more than 20 to 46 percent sand.
            return Ok(SoilTexture::ClayLoam { clay, sand, silt });
        } else if silt >= 0.80 && clay < 0.12 {
            // Silt: 80 percent or more silt and less than 12 percent clay.
            return Ok(SoilTexture::Silt { clay, sand, silt });
        } else if clay >= 0.40 && silt >= 0.40 {
            // Silty clay: 40 percent or more clay and 40 percent or more silt.
            return Ok(SoilTexture::SiltyClay { clay, sand, silt });
        } else if silt >= 0.50
            && ((silt >= 0.12 && clay < 0.27) || ((silt >= 0.50 && silt < 0.80) && clay < 0.12))
        {
            // Silt loam: 50 percent or more silt and 12 to 27 percent clay, or 50 to 80 percent silt and less than 12 percent clay.
            return Ok(SoilTexture::SiltLoam { clay, sand, silt });
        } else if clay >= 0.27 && clay < 0.40 && sand <= 0.20 {
            // Silty clay loam: 27 to 40 percent clay and 20 percent or less sand.
            return Ok(SoilTexture::SiltyClayLoam { clay, sand, silt });
        } else if ((clay >= 0.07 && clay < 0.20) && sand > 0.52 && (silt + (clay * 2.0) >= 0.30))
            || (clay < 0.07 && silt < 0.50 && sand > 0.43)
        {
            // Sandy loams: 7 to 20 percent clay, more than 52 percent sand, and the percentage of silt plus twice the percentage of clay is 30 or more;
            // or less than 7 percent clay, less than 50 percent silt, and more than 43 percent sand.
            return Ok(SoilTexture::SandyLoams { clay, sand, silt });
        }

        return Err(SoilTextureError::Unknown { clay, sand, silt });
    }
}

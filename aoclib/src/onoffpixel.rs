use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum OnOffPixel {
    On,
    Off,
}


impl OnOffPixel {
    pub fn parse(ch : char) -> OnOffPixel {
        if ch == '.' {
            OnOffPixel::Off
        } else {
            OnOffPixel::On
        }
    }

    pub fn opposite(&self) -> OnOffPixel {
        if self == &OnOffPixel::On {
            OnOffPixel::Off
        } else {
            OnOffPixel::On
        }
    }

    pub fn is_on(&self) -> bool {
        self == &OnOffPixel::On
    }
}

impl fmt::Display for OnOffPixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self == &OnOffPixel::Off { '.' } else { '#' })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(OnOffPixel::parse('.'), OnOffPixel::Off);
        assert_eq!(OnOffPixel::parse('#'), OnOffPixel::On);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", OnOffPixel::Off), ".");
        assert_eq!(format!("{}", OnOffPixel::On), "#");
    }

    #[test]
    fn opposite() {
        assert_eq!(OnOffPixel::On.opposite(), OnOffPixel::Off);
        assert_eq!(OnOffPixel::Off.opposite(), OnOffPixel::On);
    }

    #[test]
    fn is_on() {
        assert_eq!(OnOffPixel::On.is_on(), true);
        assert_eq!(OnOffPixel::Off.is_on(), false);
    }
}

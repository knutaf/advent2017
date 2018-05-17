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
}

use std::fmt;

impl fmt::Display for crate::Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl fmt::Display for crate::GalaxyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(f, "Inner: {:.2}, ", self.inner)?;
            write!(f, "Terminus: {:.2}, ", self.terminus)?;
            write!(f, "Earth: {:.2}, ", self.earth)?;
            write!(f, "Outer: {:.2}, ", self.outer)?;
            write!(f, "Attican: {:.2}", self.terminus)
        } else {
            writeln!(f, "Sector statuses:")?;
            writeln!(f, "  Inner: {:.2}", self.inner)?;
            writeln!(f, "  Terminus: {:.2}", self.terminus)?;
            writeln!(f, "  Earth: {:.2}", self.earth)?;
            writeln!(f, "  Outer: {:.2}", self.outer)?;
            write!(f, "  Attican: {:.2}", self.terminus)
        }
    }
}

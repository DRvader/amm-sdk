#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum Accidental {
  #[default]
  None,
  Natural,
  Sharp,
  Flat,
  DoubleSharp,
  DoubleFlat,
}

impl Accidental {
  pub fn value(&self) -> i16 {
    match self {
      Self::Sharp => 1,
      Self::Flat => -1,
      Self::DoubleSharp => 2,
      Self::DoubleFlat => -2,
      _ => 0,
    }
  }
}

impl std::fmt::Display for Accidental {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Natural => "♮",
        Self::Sharp => "♯",
        Self::Flat => "♭",
        Self::DoubleSharp => "𝄪",
        Self::DoubleFlat => "𝄫",
        _ => "",
      }
    )
  }
}

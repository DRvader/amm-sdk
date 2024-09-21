use crate::note::Accidental;
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const FIFTHS_A_MAJOR: i8 = 3;
const FIFTHS_A_FLAT_MAJOR: i8 = -4;
const FIFTHS_B_MAJOR: i8 = 5;
const FIFTHS_B_FLAT_MAJOR: i8 = -2;
const FIFTHS_C_MAJOR: i8 = 0;
const FIFTHS_C_SHARP_MAJOR: i8 = 7;
const FIFTHS_C_FLAT_MAJOR: i8 = -7;
const FIFTHS_D_MAJOR: i8 = 2;
const FIFTHS_D_FLAT_MAJOR: i8 = -5;
const FIFTHS_E_MAJOR: i8 = 4;
const FIFTHS_E_FLAT_MAJOR: i8 = -3;
const FIFTHS_F_MAJOR: i8 = -1;
const FIFTHS_F_SHARP_MAJOR: i8 = 6;
const FIFTHS_G_MAJOR: i8 = 1;
const FIFTHS_G_FLAT_MAJOR: i8 = -6;

const FIFTHS_F_SHARP_MINOR: i8 = 3;
const FIFTHS_F_MINOR: i8 = -4;
const FIFTHS_G_SHARP_MINOR: i8 = 5;
const FIFTHS_G_MINOR: i8 = -2;
const FIFTHS_A_MINOR: i8 = 0;
const FIFTHS_A_SHARP_MINOR: i8 = 7;
const FIFTHS_A_FLAT_MINOR: i8 = -7;
const FIFTHS_B_MINOR: i8 = 2;
const FIFTHS_B_FLAT_MINOR: i8 = -5;
const FIFTHS_C_SHARP_MINOR: i8 = 4;
const FIFTHS_C_MINOR: i8 = -3;
const FIFTHS_D_MINOR: i8 = -1;
const FIFTHS_D_SHARP_MINOR: i8 = 6;
const FIFTHS_E_MINOR: i8 = 1;
const FIFTHS_E_FLAT_MINOR: i8 = -6;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum KeyMode {
  #[default]
  Major,
  Minor,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub enum KeySignature {
  A,
  ASharp,
  AFlat,
  B,
  BFlat,
  #[default]
  C,
  CSharp,
  CFlat,
  D,
  DSharp,
  DFlat,
  E,
  EFlat,
  F,
  FSharp,
  G,
  GSharp,
  GFlat,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Key {
  pub mode: KeyMode,
  pub signature: KeySignature,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Key {
  #[must_use]
  pub fn new(signature: KeySignature, mode: KeyMode) -> Self {
    Self { signature, mode }
  }

  #[must_use]
  pub fn from_fifths(fifths: i8, mode: Option<KeyMode>) -> Self {
    let mode = mode.unwrap_or(KeyMode::Major);
    let signature = match (fifths, mode) {
      (FIFTHS_A_MAJOR, KeyMode::Major) | (FIFTHS_A_MINOR, KeyMode::Minor) => KeySignature::A,
      (FIFTHS_A_SHARP_MINOR, KeyMode::Minor) => KeySignature::ASharp,
      (FIFTHS_A_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_A_FLAT_MINOR, KeyMode::Minor) => KeySignature::AFlat,
      (FIFTHS_B_MAJOR, KeyMode::Major) | (FIFTHS_B_MINOR, KeyMode::Minor) => KeySignature::B,
      (FIFTHS_B_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_B_FLAT_MINOR, KeyMode::Minor) => KeySignature::BFlat,
      (FIFTHS_C_SHARP_MAJOR, KeyMode::Major) | (FIFTHS_C_SHARP_MINOR, KeyMode::Minor) => KeySignature::CSharp,
      (FIFTHS_C_FLAT_MAJOR, KeyMode::Major) => KeySignature::CFlat,
      (FIFTHS_D_MAJOR, KeyMode::Major) | (FIFTHS_D_MINOR, KeyMode::Minor) => KeySignature::D,
      (FIFTHS_D_SHARP_MINOR, KeyMode::Minor) => KeySignature::DSharp,
      (FIFTHS_D_FLAT_MAJOR, KeyMode::Major) => KeySignature::DFlat,
      (FIFTHS_E_MAJOR, KeyMode::Major) | (FIFTHS_E_MINOR, KeyMode::Minor) => KeySignature::E,
      (FIFTHS_E_FLAT_MAJOR, KeyMode::Major) | (FIFTHS_E_FLAT_MINOR, KeyMode::Minor) => KeySignature::EFlat,
      (FIFTHS_F_MAJOR, KeyMode::Major) | (FIFTHS_F_MINOR, KeyMode::Minor) => KeySignature::F,
      (FIFTHS_F_SHARP_MAJOR, KeyMode::Major) | (FIFTHS_F_SHARP_MINOR, KeyMode::Minor) => KeySignature::FSharp,
      (FIFTHS_G_MAJOR, KeyMode::Major) | (FIFTHS_G_MINOR, KeyMode::Minor) => KeySignature::G,
      (FIFTHS_G_FLAT_MAJOR, KeyMode::Major) => KeySignature::GFlat,
      (FIFTHS_G_SHARP_MINOR, KeyMode::Minor) => KeySignature::GSharp,
      _ => KeySignature::C,
    };
    Self { signature, mode }
  }

  #[must_use]
  pub fn fifths(&self) -> i8 {
    match (self.signature, self.mode) {
      (KeySignature::A, KeyMode::Major) => FIFTHS_A_MAJOR,
      (KeySignature::A, KeyMode::Minor) => FIFTHS_A_MINOR,
      (KeySignature::ASharp, KeyMode::Minor) => FIFTHS_A_SHARP_MINOR,
      (KeySignature::AFlat, KeyMode::Major) => FIFTHS_A_FLAT_MAJOR,
      (KeySignature::AFlat, KeyMode::Minor) => FIFTHS_A_FLAT_MINOR,
      (KeySignature::B, KeyMode::Major) => FIFTHS_B_MAJOR,
      (KeySignature::B, KeyMode::Minor) => FIFTHS_B_MINOR,
      (KeySignature::BFlat, KeyMode::Major) => FIFTHS_B_FLAT_MAJOR,
      (KeySignature::BFlat, KeyMode::Minor) => FIFTHS_B_FLAT_MINOR,
      (KeySignature::C, KeyMode::Minor) => FIFTHS_C_MINOR,
      (KeySignature::CSharp, KeyMode::Major) => FIFTHS_C_SHARP_MAJOR,
      (KeySignature::CSharp, KeyMode::Minor) => FIFTHS_C_SHARP_MINOR,
      (KeySignature::CFlat, KeyMode::Major) => FIFTHS_C_FLAT_MAJOR,
      (KeySignature::D, KeyMode::Major) => FIFTHS_D_MAJOR,
      (KeySignature::D, KeyMode::Minor) => FIFTHS_D_MINOR,
      (KeySignature::DSharp, KeyMode::Minor) => FIFTHS_D_SHARP_MINOR,
      (KeySignature::DFlat, KeyMode::Major) => FIFTHS_D_FLAT_MAJOR,
      (KeySignature::E, KeyMode::Major) => FIFTHS_E_MAJOR,
      (KeySignature::E, KeyMode::Minor) => FIFTHS_E_MINOR,
      (KeySignature::EFlat, KeyMode::Major) => FIFTHS_E_FLAT_MAJOR,
      (KeySignature::EFlat, KeyMode::Minor) => FIFTHS_E_FLAT_MINOR,
      (KeySignature::F, KeyMode::Major) => FIFTHS_F_MAJOR,
      (KeySignature::F, KeyMode::Minor) => FIFTHS_F_MINOR,
      (KeySignature::FSharp, KeyMode::Major) => FIFTHS_F_SHARP_MAJOR,
      (KeySignature::FSharp, KeyMode::Minor) => FIFTHS_F_SHARP_MINOR,
      (KeySignature::G, KeyMode::Major) => FIFTHS_G_MAJOR,
      (KeySignature::G, KeyMode::Minor) => FIFTHS_G_MINOR,
      (KeySignature::GSharp, KeyMode::Minor) => FIFTHS_G_SHARP_MINOR,
      (KeySignature::GFlat, KeyMode::Major) => FIFTHS_G_FLAT_MAJOR,
      _ => FIFTHS_C_MAJOR,
    }
  }

  #[must_use]
  pub fn to_parallel(&self) -> Self {
    Self {
      mode: if self.mode == KeyMode::Major {
        KeyMode::Minor
      } else {
        KeyMode::Major
      },
      signature: self.signature,
    }
  }

  #[must_use]
  pub fn to_relative(&self) -> Self {
    let new_mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
    Key::from_fifths(self.fifths(), Some(new_mode))
  }

  pub fn make_parallel(&mut self) {
    self.mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
  }

  pub fn make_relative(&mut self) {
    let new_mode = if self.mode == KeyMode::Major {
      KeyMode::Minor
    } else {
      KeyMode::Major
    };
    *self = Key::from_fifths(self.fifths(), Some(new_mode));
  }

  #[must_use]
  pub(crate) fn accidentals(self) -> [Accidental; 8] {
    let fifths = self.fifths();
    [
      Accidental::None,
      match fifths {
        x if x <= -3 => Accidental::Flat,
        x if x >= 5 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -1 => Accidental::Flat,
        x if x >= 7 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -6 => Accidental::Flat,
        x if x >= 2 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -4 => Accidental::Flat,
        x if x >= 4 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -2 => Accidental::Flat,
        x if x >= 6 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -7 => Accidental::Flat,
        x if x >= 1 => Accidental::Sharp,
        _ => Accidental::None,
      },
      match fifths {
        x if x <= -5 => Accidental::Flat,
        x if x >= 3 => Accidental::Sharp,
        _ => Accidental::None,
      },
    ]
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for KeyMode {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Major => "Major",
        Self::Minor => "Minor",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for KeySignature {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::A => "A",
        Self::ASharp => "A♯",
        Self::AFlat => "A♭",
        Self::B => "B",
        Self::BFlat => "B♭",
        Self::C => "C",
        Self::CSharp => "C♯",
        Self::CFlat => "C♭",
        Self::D => "D",
        Self::DSharp => "D♯",
        Self::DFlat => "D♭",
        Self::E => "E",
        Self::EFlat => "E♭",
        Self::F => "F",
        Self::FSharp => "F♯",
        Self::G => "G",
        Self::GSharp => "G♯",
        Self::GFlat => "G♭",
      }
    )
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Key {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(
      f,
      "{}{}",
      self.signature,
      if self.mode == KeyMode::Major { "" } else { "m" }
    )
  }
}

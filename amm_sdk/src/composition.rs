use crate::context::{Key, Tempo, TimeSignature};
use crate::note::Note;
use crate::structure::{Chord, MultiVoice, Part, Phrase, Section, Staff};
use crate::temporal::{place_and_merge_part_timeslice, PartTimeslice};
use amm_internal::amm_prelude::*;
use amm_macros::{JsonDeserialize, JsonSerialize};

#[derive(Debug, Default, Eq, PartialEq, JsonDeserialize, JsonSerialize)]
pub struct Composition {
  title: String,
  copyright: Option<String>,
  publisher: Option<String>,
  composers: Vec<String>,
  lyricists: Vec<String>,
  arrangers: Vec<String>,
  metadata: BTreeMap<String, String>,
  parts: Vec<Part>,
  tempo: Tempo,
  starting_key: Key,
  starting_time_signature: TimeSignature,
}

impl Composition {
  #[must_use]
  pub fn new(title: &str, tempo: Option<Tempo>, key: Option<Key>, time_signature: Option<TimeSignature>) -> Self {
    Self {
      title: String::from(title),
      copyright: None,
      publisher: None,
      composers: Vec::new(),
      lyricists: Vec::new(),
      arrangers: Vec::new(),
      metadata: BTreeMap::new(),
      parts: Vec::new(),
      tempo: tempo.unwrap_or_default(),
      starting_key: key.unwrap_or_default(),
      starting_time_signature: time_signature.unwrap_or_default(),
    }
  }

  #[must_use]
  pub fn flatten(&self) -> Self {
    // Combines simultaneously played parts (i.e., multivoices) into single phrases
    // and returns a new Composition that is guaranteed to have no multivoices
    Self {
      title: self.title.clone(),
      copyright: self.copyright.clone(),
      publisher: self.publisher.clone(),
      composers: self.composers.clone(),
      lyricists: self.lyricists.clone(),
      arrangers: self.arrangers.clone(),
      metadata: self.metadata.clone(),
      parts: self.parts.iter().map(Part::flatten).collect(),
      tempo: self.tempo,
      starting_key: self.starting_key,
      starting_time_signature: self.starting_time_signature,
    }
  }

  #[must_use]
  pub fn restructure_staves_as_parts(&self) -> Self {
    // Converts each staff in a part into a new part, ensuring that each part
    // contains only a single staff
    Self {
      title: self.title.clone(),
      copyright: self.copyright.clone(),
      publisher: self.publisher.clone(),
      composers: self.composers.clone(),
      lyricists: self.lyricists.clone(),
      arrangers: self.arrangers.clone(),
      metadata: self.metadata.clone(),
      parts: self.parts.iter().flat_map(Part::extract_staves_as_parts).collect(),
      tempo: self.tempo,
      starting_key: self.starting_key,
      starting_time_signature: self.starting_time_signature,
    }
  }

  pub fn set_title(&mut self, title: &str) -> &mut Self {
    self.title = String::from(title);
    self
  }

  pub fn set_copyright(&mut self, copyright: &str) -> &mut Self {
    self.copyright = Some(String::from(copyright));
    self
  }

  pub fn set_publisher(&mut self, publisher: &str) -> &mut Self {
    self.publisher = Some(String::from(publisher));
    self
  }

  pub fn set_tempo(&mut self, tempo: Tempo) -> &mut Self {
    self.tempo = tempo;
    self
  }

  pub fn set_starting_key(&mut self, key: Key) -> &mut Self {
    self.starting_key = key;
    self
  }

  pub fn set_starting_time_signature(&mut self, time_signature: TimeSignature) -> &mut Self {
    self.starting_time_signature = time_signature;
    self
  }

  pub fn add_composer(&mut self, name: &str) -> &mut Self {
    self.composers.push(String::from(name));
    self
  }

  pub fn add_lyricist(&mut self, name: &str) -> &mut Self {
    self.lyricists.push(String::from(name));
    self
  }

  pub fn add_arranger(&mut self, name: &str) -> &mut Self {
    self.arrangers.push(String::from(name));
    self
  }

  pub fn add_metadata(&mut self, key: &str, value: &str) -> &mut Self {
    self.metadata.insert(String::from(key), String::from(value));
    self
  }

  pub fn add_part(&mut self, name: &str) -> &mut Part {
    self.remove_part_by_name(name).parts.push(Part::new(name));
    unsafe { self.parts.last_mut().unwrap_unchecked() }
  }

  #[must_use]
  pub fn get_title(&self) -> &str {
    &self.title
  }

  #[must_use]
  pub fn get_copyright(&self) -> &Option<String> {
    &self.copyright
  }

  #[must_use]
  pub fn get_publisher(&self) -> &Option<String> {
    &self.publisher
  }

  #[must_use]
  pub fn get_tempo(&self) -> &Tempo {
    &self.tempo
  }

  #[must_use]
  pub fn get_starting_key(&self) -> &Key {
    &self.starting_key
  }

  #[must_use]
  pub fn get_starting_time_signature(&self) -> &TimeSignature {
    &self.starting_time_signature
  }

  #[must_use]
  pub fn get_composers(&self) -> &[String] {
    &self.composers
  }

  #[must_use]
  pub fn get_lyricists(&self) -> &[String] {
    &self.lyricists
  }

  #[must_use]
  pub fn get_arrangers(&self) -> &[String] {
    &self.arrangers
  }

  #[must_use]
  pub fn get_metadata(&self) -> &BTreeMap<String, String> {
    &self.metadata
  }

  #[must_use]
  pub fn get_part_names(&self) -> Vec<String> {
    self.parts.iter().map(|part| String::from(part.get_name())).collect()
  }

  #[must_use]
  pub fn get_part_by_name(&self, name: &str) -> Option<&Part> {
    self.parts.iter().find(|part| part.get_name() == name)
  }

  #[must_use]
  pub fn get_part_mut_by_name(&mut self, name: &str) -> Option<&mut Part> {
    self.parts.iter_mut().find(|part| part.get_name() == name)
  }

  #[must_use]
  pub fn get_part(&self, id: usize) -> Option<&Part> {
    self.parts.iter().find(|part| part.get_id() == id)
  }

  #[must_use]
  pub fn get_part_mut(&mut self, id: usize) -> Option<&mut Part> {
    self.parts.iter_mut().find(|part| part.get_id() == id)
  }

  #[must_use]
  pub fn get_chord(&self, id: usize) -> Option<&Chord> {
    self.parts.iter().find_map(|part| part.get_chord(id))
  }

  #[must_use]
  pub fn get_chord_mut(&mut self, id: usize) -> Option<&mut Chord> {
    self.parts.iter_mut().find_map(|part| part.get_chord_mut(id))
  }

  #[must_use]
  pub fn get_multivoice(&self, id: usize) -> Option<&MultiVoice> {
    self.parts.iter().find_map(|part| part.get_multivoice(id))
  }

  #[must_use]
  pub fn get_multivoice_mut(&mut self, id: usize) -> Option<&mut MultiVoice> {
    self.parts.iter_mut().find_map(|part| part.get_multivoice_mut(id))
  }

  #[must_use]
  pub fn get_note(&self, id: usize) -> Option<&Note> {
    self.parts.iter().find_map(|part| part.get_note(id))
  }

  #[must_use]
  pub fn get_note_mut(&mut self, id: usize) -> Option<&mut Note> {
    self.parts.iter_mut().find_map(|part| part.get_note_mut(id))
  }

  #[must_use]
  pub fn get_phrase(&self, id: usize) -> Option<&Phrase> {
    self.parts.iter().find_map(|part| part.get_phrase(id))
  }

  #[must_use]
  pub fn get_phrase_mut(&mut self, id: usize) -> Option<&mut Phrase> {
    self.parts.iter_mut().find_map(|part| part.get_phrase_mut(id))
  }

  #[must_use]
  pub fn get_section(&self, id: usize) -> Option<&Section> {
    self.parts.iter().find_map(|part| part.get_section(id))
  }

  #[must_use]
  pub fn get_section_mut(&mut self, id: usize) -> Option<&mut Section> {
    self.parts.iter_mut().find_map(|part| part.get_section_mut(id))
  }

  #[must_use]
  pub fn get_staff(&self, id: usize) -> Option<&Staff> {
    self.parts.iter().find_map(|part| part.get_staff(id))
  }

  #[must_use]
  pub fn get_staff_mut(&mut self, id: usize) -> Option<&mut Staff> {
    self.parts.iter_mut().find_map(|part| part.get_staff_mut(id))
  }

  #[must_use]
  pub fn get_beats(&self) -> f64 {
    self
      .parts
      .iter()
      .map(|part| part.get_beats(&self.tempo.base_note))
      .reduce(f64::max)
      .unwrap_or_default()
  }

  #[must_use]
  pub fn get_duration(&self) -> f64 {
    // Note: Does not take into account fermatas or gradual tempo changes like accelerandos as these are style-dependent
    self.get_beats() * 60.0 / f64::from(self.tempo.beats_per_minute)
  }

  pub fn remove_copyright(&mut self) -> &mut Self {
    self.copyright = None;
    self
  }

  pub fn remove_publisher(&mut self) -> &mut Self {
    self.publisher = None;
    self
  }

  pub fn remove_composer(&mut self, name: &str) -> &mut Self {
    self.composers.retain(|composer| composer != name);
    self
  }

  pub fn remove_lyricist(&mut self, name: &str) -> &mut Self {
    self.lyricists.retain(|lyricist| lyricist != name);
    self
  }

  pub fn remove_arranger(&mut self, name: &str) -> &mut Self {
    self.arrangers.retain(|arranger| arranger != name);
    self
  }

  pub fn remove_metadata(&mut self, key: &str) -> &mut Self {
    self.metadata.remove(key);
    self
  }

  pub fn remove_part_by_name(&mut self, name: &str) -> &mut Self {
    self.parts.retain(|part| part.get_name() != name);
    self
  }

  pub fn remove_item(&mut self, id: usize) -> &mut Self {
    self.parts.retain(|part| part.get_id() != id);
    self.parts.iter_mut().for_each(|part| {
      part.remove_item(id);
    });
    self
  }

  pub fn remove_modification(&mut self, id: usize) -> &mut Self {
    self.iter_mut().for_each(|part| {
      part.remove_modification(id);
    });
    self
  }

  #[must_use]
  pub fn num_timeslices(&self) -> usize {
    self.parts.iter().map(Part::num_timeslices).max().unwrap_or_default()
  }

  pub fn iter(&self) -> core::slice::Iter<'_, Part> {
    self.parts.iter()
  }

  pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Part> {
    self.parts.iter_mut()
  }

  #[must_use]
  pub fn iter_timeslices(&self) -> impl core::iter::FusedIterator<Item = PartTimeslice> {
    // Return PartTimeslices where each slice contains a map of parts and their current timeslice
    // Note: If you want timeslices for a single part, call `iter_timeslices()` on the part directly
    let mut timeslices: Vec<(f64, PartTimeslice)> = Vec::new();
    for part in &self.parts {
      let part_name = part.get_name();
      let (mut index, mut curr_time) = (0, 0.0);
      for slice in part.iter_timeslices() {
        (index, curr_time) = place_and_merge_part_timeslice(part_name, &mut timeslices, slice, index, curr_time);
      }
    }
    timeslices.into_iter().map(|(_, slice)| slice)
  }
}

impl IntoIterator for Composition {
  type Item = Part;
  type IntoIter = alloc::vec::IntoIter<Self::Item>;
  fn into_iter(self) -> Self::IntoIter {
    self.parts.into_iter()
  }
}

impl<'a> IntoIterator for &'a Composition {
  type Item = &'a Part;
  type IntoIter = core::slice::Iter<'a, Part>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Composition {
  type Item = &'a mut Part;
  type IntoIter = core::slice::IterMut<'a, Part>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

#[cfg(feature = "print")]
impl core::fmt::Display for Composition {
  #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let duration = self.get_duration();
    write!(f, "Composition:\n  Title: {}\n  First Composer: {}\n  First Lyricist: {}\n  First Arranger: {}\n  Publisher: {}\n  Copyright: {}\n  Tempo: {}\n  Key: {}\n  Time Signature: {}\n  Num Parts: {}\n  Length: {:02}:{:02}",
      self.title,
      self.composers.first().unwrap_or(&String::from("Unknown")),
      self.lyricists.first().unwrap_or(&String::from("Unknown")),
      self.arrangers.first().unwrap_or(&String::from("Unknown")),
      self.publisher.as_deref().unwrap_or("Unknown"),
      self.copyright.as_deref().unwrap_or("None"),
      self.tempo,
      self.starting_key,
      self.starting_time_signature,
      self.parts.len(),
      duration as u32 / 60,
      duration as u32 % 60
    )
  }
}

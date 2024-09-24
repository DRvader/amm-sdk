use amm::{Composition, Storage};

#[allow(dead_code)]
fn test_iterating(composition: &mut Composition) {
  println!("{}", composition);
  for part_name in &composition.get_part_names() {
    if let Some(part) = composition.get_part_by_name(part_name) {
      println!("{part}\n\n\n");
    } else {
      println!("Part {part_name} not found");
    }
  }
}

#[allow(dead_code)]
fn test_timeslices(composition: &mut Composition) {
  println!("{}", composition);
  for part_name in &composition.get_part_names() {
    if let Some(part) = composition.get_part_by_name(part_name) {
      println!("\nPart {part_name}:");
      part.iter_timeslices().into_iter().for_each(|timeslice| {
        println!("  {timeslice}");
      });
    } else {
      println!("Part {part_name} not found");
    }
  }
}

#[allow(dead_code)]
fn test_flattened_and_restructured_iterating(composition: &mut Composition) {
  let mut composition = composition.restructure_staves_as_parts().flatten();
  println!("{}", composition);
  test_iterating(&mut composition);
}

#[allow(dead_code)]
fn test_flattened_and_restructured_timeslices(composition: &mut Composition) {
  let mut composition = composition.restructure_staves_as_parts().flatten();
  println!("{}", composition);
  test_timeslices(&mut composition);
}

fn main() {
  let mut composition = Storage::MusicXML.load("./amm/examples/Grande Valse Brillante.musicxml");
  match composition {
    Ok(ref mut composition) => {
      test_iterating(composition);
      //test_timeslices(composition);
      //test_flattened_and_restructured_iterating(composition);
      //test_flattened_and_restructured_timeslices(composition);
    }
    Err(error) => println!("{}", error),
  }
}

use std::fs;

pub fn get_bib_test_content() -> String {
    r#"@Article{Newton2026DivergentGravity,
  author       = {Newton, Isaac and Mach, Ernst and Doe, Jane},
  title        = {Foundations of the Divergent Gravity Model: Repulsive Force and Machian Inertia},
  journaltitle = {Journal of Advanced Theoretical Physics},
  date         = {2026-02-21},
  volume       = {42},
  number       = {3},
  pages        = {150-185},
  issn         = {1234-5678},
  doi          = {10.1000/tg-2026-xyz},
  url          = {https://example.org/divergent-gravity-simulation},
  urldate      = {2026-02-21},
  language     = {english},
  abstract     = {This paper explores the Divergent Gravity model, proposing that gravity functions as a repulsive force. This shift provides a novel framework for Machian inertia, suggesting that the local resistance to acceleration is a byproduct of the global distribution of matter via repulsive interactions.},
  keywords     = {divergent gravity, machian inertia, repulsion, theoretical physics},
  pubstate     = {published},
  editor       = {Smith, Robert},
  series       = {New Horizons in Cosmology},
  location     = {Cambridge, MA},
  publisher    = {Academic Simulation Press},
  shorttitle   = {Divergent Gravity Foundations},
  note   = {This is a test entry containing a variety of keys to ensure robust parsing in the simulation to-do list project.},
  file         = {:path/to/Newton2026_Simulation.pdf:PDF},
  groups       = {Simulation To-Do List},
}

@Article{Newton2026DivergentGravity2,
  author       = {Newton, Isaac and Mach, Ernst and Doe, Jane},
  title        = {Foundations of the Divergent Gravity Model: Repulsive Force and Machian Inertia},
  journaltitle = {Journal of Advanced Theoretical Physics},
  date         = {2026-02-21},
  volume       = {42},
  number       = {3},
  pages        = {150-185},
  issn         = {1234-5678},
  doi          = {10.1000/tg-2026-xyz},
  url          = {https://example.org/divergent-gravity-simulation},
  urldate      = {2026-02-21},
  language     = {english},
  abstract     = {This paper explores the Divergent Gravity model, proposing that gravity functions as a repulsive force. This shift provides a novel framework for Machian inertia, suggesting that the local resistance to acceleration is a byproduct of the global distribution of matter via repulsive interactions.},
  keywords     = {divergent gravity, machian inertia, repulsion, theoretical physics},
  pubstate     = {published},
  editor       = {Smith, Robert},
  series       = {New Horizons in Cosmology},
  location     = {Cambridge, MA},
  publisher    = {Academic Simulation Press},
  shorttitle   = {Divergent Gravity Foundations},
  annotation   = {This is a test entry containing a variety of keys to ensure robust parsing in the simulation to-do list project.},
  file         = {:path/to/Newton2026_Simulation.pdf:PDF},
  groups       = {Simulation To-Do List},
}

@Article{Newton2026DivergentGravity3,
  author       = {Newton, Isaac and Mach, Ernst and Doe, Jane},
  title        = {Foundations of the Divergent Gravity Model: Repulsive Force and Machian Inertia},
  journaltitle = {Journal of Advanced Theoretical Physics},
  date         = {2026-02-21},
  volume       = {42},
  number       = {3},
  pages        = {150-185},
  issn         = {1234-5678},
  doi          = {10.1000/tg-2026-xyz},
  url          = {https://example.org/divergent-gravity-simulation},
  urldate      = {2026-02-21},
  language     = {english},
  abstract     = {This paper explores the Divergent Gravity model, proposing that gravity functions as a repulsive force. This shift provides a novel framework for Machian inertia, suggesting that the local resistance to acceleration is a byproduct of the global distribution of matter via repulsive interactions.},
  keywords     = {divergent gravity, machian inertia, repulsion, theoretical physics},
  pubstate     = {published},
  editor       = {Smith, Robert},
  series       = {New Horizons in Cosmology},
  location     = {Cambridge, MA},
  publisher    = {Academic Simulation Press},
  shorttitle   = {Divergent Gravity Foundations},
  annotation   = {This is a test entry containing a variety of keys to ensure robust parsing in the simulation to-do list project.},
  file         = {:path/to/Newton2026_Simulation.pdf:PDF},
  groups       = {Simulation To-Do List},
}

        "#.to_string()
}

pub fn get_test_csl_content(file_name: Option<String>) -> String {
    let _fn = file_name.unwrap_or("apa.csl".to_string());
    let this_file = file!();
    println!("File: {}", this_file);
    let p = std::path::Path::new(this_file);
    let x = p
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("bibliography_embedded")
        .join("csl")
        .join(_fn);

    println!("X: {}", x.display());
    std::fs::read_to_string(x).expect("Must return test csl content")
}

pub fn get_test_csl_locale() -> String {
    let this_file = file!();
    let p = std::path::Path::new(this_file);
    let x = p
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("bibliography_embedded")
        .join("csl_locale")
        .join("en_us.xml");
    println!("X: {}", x.display());
    std::fs::read_to_string(x).expect("Must return test csl locale")
}

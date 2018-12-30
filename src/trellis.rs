use git2::Repository;
use std::fs::{remove_dir_all, DirBuilder};
use std::path::Path;

use super::{Bedrock, Env, Sage, Theme};

pub struct Trellis {
  name: String,
  sites: Vec<Bedrock>,
  environments: Vec<Env>,
}

impl Trellis {
  pub fn new(name: String) -> Trellis {
    let site = Bedrock::new(
      name.to_string(),
      Theme::IsSage(Sage::new(String::from("sage"), &name.to_string())),
    );

    Trellis {
      name,
      sites: vec![site],
      environments: vec![
        Env::new(String::from("production")),
        Env::new(String::from("development")),
        Env::new(String::from("staging")),
      ],
    }
  }

  pub fn init(&self) {
    let path = Path::new(&self.name);

    if !path.exists() {
      DirBuilder::new()
        .recursive(true)
        .create(&self.name)
        .unwrap();
      println!("Created project directory at ./{}", &self.name);
    }

    // Install Trellis
    self.install_trellis();

    // Install Bedrock sites
    self.create_sites();

    // Initialize git repository
    match Repository::init(&self.name) {
      Ok(repo) => repo,
      Err(e) => panic!("{}", e),
    };
    println!("Initialized project git repository.");
  }

  fn install_trellis(&self) {
    let path = Path::new(&self.name).join("trellis");

    if !path.exists() {
      match Repository::clone("https://github.com/roots/trellis", &path) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", e),
      };

      match remove_dir_all(&path.join(".git")) {
        Ok(remove_git) => remove_git,
        Err(e) => panic!("{}", e),
      };

      println!("Installed Trellis at ./{}/trellis", &path.to_str().unwrap());
    } else {
      println!("Trellis is already installed.");
    }
  }

  fn create_sites(&self) {
    println!("Creating Bedrock sites...");

    for bedrock in &self.sites {
      bedrock.init();
    }
  }
}

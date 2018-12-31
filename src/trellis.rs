use git2::Repository;
use std::fs::{remove_dir_all, DirBuilder};
use std::path::Path;

use super::{Bedrock, Env, Sage, Theme};

pub struct Trellis {
    name: String,
    sites: Vec<Bedrock>,
    environments: Vec<Env>,
    repo: String,
}

impl Trellis {
    pub fn new(name: String, without_sage: bool) -> Trellis {
        let site = Bedrock::new(
            name.to_string(),
            match without_sage {
                true => Theme::NotSage,
                false => Theme::IsSage(Sage::new(String::from("sage"), &name.to_string())),
            },
        );

        Trellis {
            name,
            sites: vec![site],
            environments: vec![
                Env::new(String::from("production")),
                Env::new(String::from("development")),
                Env::new(String::from("staging")),
            ],
            repo: String::from("https://github.com/roots/trellis"),
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
        Repository::init(&self.name).unwrap();
        println!("Initialized project git repository.");
    }

    fn install_trellis(&self) {
        let path = Path::new(&self.name).join("trellis");

        if !path.exists() {
            Repository::clone(&self.repo, &path).unwrap();
            remove_dir_all(&path.join(".git")).unwrap();
            println!("Installed Trellis at ./{}/trellis", &path.to_str().unwrap());
        } else {
            println!("Trellis is already installed.");
        }
    }

    fn create_sites(&self) {
        println!("Creating Bedrock sites...");

        for bedrock in &self.sites {
            bedrock.init(self.sites.len() == 1usize, &self.name);
        }
    }
}

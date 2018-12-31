use git2::Repository;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Sage {
    name: String,
    path: PathBuf,
    repo: String,
}

pub enum BuildEnv {
    Production,
    Development,
}

pub enum Theme {
    IsSage(Sage),
    NotSage,
}

impl Sage {
    pub fn new(name: String, site_name: &str) -> Sage {
        let path = Path::new(&site_name)
            .join("site/web/app/themes")
            .join(&name);

        Sage {
            name,
            path,
            repo: String::from("https://github.com/roots/sage"),
        }
    }

    pub fn init(&self) {
        println!("Cloning Sage...");
        Repository::clone(&self.repo, &self.path).unwrap();

        // Remove Sage's git repo
        remove_dir_all(&self.path.join(".git")).unwrap();

        println!("Installing Composer dependencies...");
        Command::new("composer")
            .current_dir(&self.path)
            .arg("install")
            .output()
            .unwrap();

        self.build(BuildEnv::Development);

        println!("Sage installed at ./{}", &self.path.to_str().unwrap());
    }

    pub fn build(&self, env: BuildEnv) {
        self.ensure_node_modules();

        match env {
            BuildEnv::Development => {
                println!("Building assets for development...");
                Command::new("yarn")
                    .current_dir(&self.path)
                    .arg("build")
                    .output()
                    .unwrap();
            }
            BuildEnv::Production => {
                println!("Building assets for production...");
                Command::new("yarn")
                    .current_dir(&self.path)
                    .arg("build::production")
                    .output()
                    .unwrap();
            }
        };
    }

    pub fn activate(&self) {
        // Activate Sage in WordPress
    }

    fn ensure_node_modules(&self) {
        let node_modules_path = &self.path.join("node_modules").exists();
        if !node_modules_path {
            println!("Installing Node dependencies...");
            Command::new("yarn")
                .current_dir(&self.path)
                .output()
                .unwrap();
        } else {
            println!("Node already installed.");
        }
    }
}

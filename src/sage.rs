use git2::Repository;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Sage {
  name: String,
  path: PathBuf,
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

    Sage { name, path }
  }

  pub fn init(&self) {
    println!("Cloning Sage...");
    match Repository::clone("https://github.com/roots/sage", &self.path) {
      Ok(repo) => repo,
      Err(e) => panic!("{}", e),
    };

    println!("Installing Composer dependencies...");
    match Command::new("composer")
      .current_dir(&self.path)
      .arg("install")
      .output()
    {
      Ok(composer_install) => composer_install,
      Err(e) => panic!("{}", e),
    };

    self.build(BuildEnv::Development);

    println!("Sage installed at ./{}", &self.path.to_str().unwrap());
  }

  pub fn build(&self, env: BuildEnv) {
    self.ensure_node_modules();

    match env {
      BuildEnv::Development => {
        println!("Building assets for development...");
        match Command::new("yarn")
          .current_dir(&self.path)
          .arg("build")
          .output()
        {
          Ok(build_dev_assets) => build_dev_assets,
          Err(e) => panic!("{}", e),
        };
      }
      BuildEnv::Production => {
        println!("Building assets for production...");
        match Command::new("yarn")
          .current_dir(&self.path)
          .arg("build::production")
          .output()
        {
          Ok(build_prod_assets) => build_prod_assets,
          Err(e) => panic!("{}", e),
        };
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
      match Command::new("yarn").current_dir(&self.path).output() {
        Ok(install_node_modules) => install_node_modules,
        Err(e) => panic!("{}", e),
      };
    } else {
      println!("Node already installed.");
    }
  }
}

use git2::Repository;
use std::fs::remove_dir_all;
use std::path::Path;

pub struct Bedrock {
  name: String,
}

impl Bedrock {
  pub fn new(name: String) -> Bedrock {
    Bedrock { name }
  }

  pub fn init(&self) {
    let path = Path::new(&self.name).join("site");

    if !path.exists() {
      match Repository::clone("https://github.com/roots/bedrock", &path) {
        Ok(repo) => repo,
        Err(e) => panic!("{}", e),
      };

      match remove_dir_all(&path.join(".git")) {
        Ok(remove_git) => remove_git,
        Err(e) => panic!("{}", e),
      };

      println!("Created site at ./{}/site", &self.name);
    } else {
      println!("Site {} already exists.", &self.name);
    }
  }
}

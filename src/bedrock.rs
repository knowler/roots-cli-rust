use git2::Repository;
use std::fs::remove_dir_all;
use std::path::Path;

use super::Theme;

pub struct Bedrock {
  pub name: String,
  theme: Theme,
}

impl Bedrock {
  pub fn new(name: String, theme: Theme) -> Bedrock {
    Bedrock { name, theme }
  }

  pub fn init(&self) {
    let path = Path::new(&self.name).join("site");

    if !path.exists() {
      Repository::clone("https://github.com/roots/bedrock", &path).unwrap();
      remove_dir_all(&path.join(".git")).unwrap();
      println!("Created site at ./{}", &path.to_str().unwrap());

      // If theme is set to Sage install Sage.
      match &self.theme {
        Theme::IsSage(theme) => theme.init(),
        Theme::NotSage => (),
      };
    } else {
      println!("Site {} already exists.", &self.name);
    }
  }
}

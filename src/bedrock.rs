use git2::Repository;
use std::fs::remove_dir_all;
use std::path::Path;

use super::Theme;

pub struct Bedrock {
    pub name: String,
    theme: Theme,
    repo: String,
}

impl Bedrock {
    pub fn new(name: String, theme: Theme) -> Bedrock {
        Bedrock {
            name,
            theme,
            repo: String::from("https://github.com/roots/bedrock"),
        }
    }

    pub fn init(&self, alone: bool, project_name: &str) {
        let path = match alone {
            true => Path::new(&self.name).join("site"),
            false => Path::new(project_name).join(&self.name),
        };

        if !path.exists() {
            Repository::clone(&self.repo, &path).unwrap();
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

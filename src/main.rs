#[macro_use]
extern crate clap;
use clap::App;

extern crate roots;
use roots::Trellis;

fn main() {
  let cli = load_yaml!("cli.yml");
  let m = App::from_yaml(cli).get_matches();

  if let Some(m) = m.subcommand_matches("new") {
    new_project(m.value_of("project").unwrap(), m.is_present("without_sage"));
  };

  if let Some(m) = m.subcommand_matches("sage") {
    println!("Manage Sage...");
  };
}

fn new_project(name: &str, without_sage: bool) {
  println!("Creating new Roots project...");
  let project = Trellis::new(name.to_string(), without_sage);
  project.init();
}

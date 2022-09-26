pub mod config;
pub mod layer;

#[derive(Debug)]
pub struct Kmonad {
	pub config: config::Config,

	pub layers: Vec<layer::Layer>,
}

impl Kmonad {
	pub fn new(cfg: config::Config) -> Kmonad {
		Kmonad{
			config: cfg,
			layers: vec![],
		}
	}
}

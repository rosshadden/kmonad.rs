mod kmonad;

fn main() {
	let k = kmonad::Kmonad::new(kmonad::config::Config {
		input: "in".to_string(),
		output: "out".to_string(),
	});

	println!("cfg: {}\nvec: {}", k.config.input, k.layers.len());
}

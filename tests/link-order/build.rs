extern crate xmake;

use xmake::Config;

fn main() {
    let mut config = Config::new(".");
    config.build();

    let links: Vec<_> = config
        .build_info()
        .links()
        .iter()
        .map(|link| link.name())
        .collect();
    println!("xmake-rs link order: {:?}", links);

    let position = |name: &str| {
        links
            .iter()
            .position(|link| *link == name)
            .unwrap_or_else(|| panic!("link `{}` is missing from {:?}", name, links))
    };

    assert!(
        position("gamma") < position("beta"),
        "expected gamma before beta, got {:?}",
        links
    );
    assert!(
        position("beta") < position("alpha"),
        "expected beta before alpha, got {:?}",
        links
    );
}

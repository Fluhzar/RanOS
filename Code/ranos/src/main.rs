use std::{fs::File, process::exit};

use ranos_draw::DrawBuilder;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        let mut drawer = ron::de::from_reader::<_, Box<dyn DrawBuilder>>(File::open(args[1].as_str()).unwrap())
            .unwrap()
            .build();
        drawer.run();
        println!("{}", drawer.stats());
    } else {
        eprintln!("Please provide a .ron configuration file.");
        exit(1);
    }
}

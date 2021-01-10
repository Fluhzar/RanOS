use std::{fs::File, process::exit};

use ranos_draw::DrawBuilder;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        ron::de::from_reader::<_, Box<dyn DrawBuilder>>(File::open(args[1].as_str()).unwrap())
            .unwrap()
            .build()
            .run();
    } else {
        eprintln!("Please provide a .ron configuration file.");
        exit(1);
    }
}

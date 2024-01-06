// Enums
#[derive(Debug)]
enum WineRegion {
    ITALY,
    FRANCE,
}

struct Wine {
    name: String,
    region: WineRegion,
}

impl WineRegion {
    fn call(&self) {
        println!("{:?}", self)
    }
}

fn printwine(w: &WineRegion) {
    match w {
        WineRegion::FRANCE => {
            println!("A great wine from france")
        }

        WineRegion::ITALY => {
            println!("Best wine from italy")
        } // _ => {
          //     println!("Region not found")
          // }
    }
}

fn main() {
    let wine1 = Wine {
        name: "shp".to_string(),
        region: WineRegion::FRANCE,
    };
    let wine2 = Wine {
        name: "ty".to_string(),
        region: WineRegion::ITALY,
    };

    printwine(&wine1.region);

    wine2.region.call();

    println!("{} form {:?}", wine1.name, wine1.region);
    println!("{} form {:?}", wine2.name, wine2.region);

    let x = 8;
    let y: Option<i32> = Some(10);
    let z = x + y.unwrap_or(0);

    println!("{z}")
}

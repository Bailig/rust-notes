// structs, enums, closures, and functions all can implement traits
// Debug trait: to derive the Degug trait, all fields must also implement Debug
// Clone trait: to derive the Clone trait, all fields must also implement Clone
// Copy trait: only for types that are stored on the stack
// Copy is a subset of Clone, so if a type implements Copy, it also implements Clone
// Tiny structs and enums are better off implementing Copy becuase it's faster to copy than to move
// From and Into traits: if From is implemented, Into is automatically implemented
// references to types are their own types, so they can implement traits too

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

impl Default for Party {
    fn default() -> Self {
        Self {
            at_restaurant: true,
            num_people: 8,
            cake: Cake::Chocolate,
        }
    }
}

impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        self.cake == other.cake
    }
}

impl From<&Party> for Cake {
    fn from(value: &Party) -> Self {
        value.cake.clone()
    }
}

pub fn run() {
    let cake = Cake::Spice;
    admire_cake(cake);

    match cake {
        Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
        Cake::MapleBacon => println!("Dreams do come true!"),
        Cake::Spice => println!("Great, let's spice it up!"),
    }

    println!("The default Party is\n{:#?}", Party::default());

    let party = Party {
        cake: Cake::MapleBacon,
        ..Party::default()
    };
    println!("Yes! My party has my favorite {:?} cake!", party.cake);

    let other_party = Party {
        at_restaurant: false,
        num_people: 235,
        cake: Cake::MapleBacon,
    };
    if party == other_party {
        println!("Your party is just like mine!");
    }

    smell_cake(&party);

    println!(
        "Yum! I'm eating this cake: {:?}. Oops, I dropped it on the floor.",
        party.cake
    );
    drop(cake);
}

pub fn smell_cake<T: Into<Cake>>(item: T) {
    let cake: Cake = item.into();
    println!("Hmm...something smells like a {:?} cake!", cake);
}

pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! 🎂", cake);
}

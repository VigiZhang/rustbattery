#[macro_use]
extern crate bitflags;

bitflags! {
    #[derive(Default)]
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    // basics
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC); // union
    assert_eq!((e1 & e2), Flags::C); // intersection
    assert_eq!((e1 - e2), Flags::A); // set difference
    assert_eq!(!e2, Flags::A); // set complement

    println!("{:?}", Flags::A);
    println!("{:?}", Flags::ABC);

    // clear
    let mut flags = Flags::A | Flags::B;
    flags.clear();
    assert!(flags.is_empty());

    // fmt::Display
    assert_eq!(format!("{}", flags), "hi!");
    assert_eq!(format!("{:?}", Flags::A | Flags::B), "A | B");
    assert_eq!(format!("{:?}", Flags::B), "B");
    
    assert_eq!(format!("{:?}", Flags::ABC), "A | B | C | ABC");

    // default is 0
    let derived_default: Flags = Default::default();
    assert_eq!(derived_default.bits(), 0);
}

/* custom default
impl Default for Flags {
    fn default() -> Flags {
        Flags::A | Flags::C
    }
}
*/

use std::fmt;

impl Flags {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hi!")
    }
}

mod example {
    bitflags! {
        // need pub in mod
        pub struct Flags1: u32 {
            const A = 0b00000001;
        }
    }
    bitflags! {
        pub struct Flags2: u32 {
            const B = 0b00000010;
        }
    }
}

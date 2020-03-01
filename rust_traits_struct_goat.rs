struct Goat { is_demonic: bool, name: &'static str }

trait Satan {
    fn new(name: &'static str) -> Self;
    // Self is implementor type; we don't know what it is
    // but our trait's functionalities can get latched onto it

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // default method def via trait:
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Goat {
    fn is_demonic(&self) -> bool {
        self.is_demonic
    }
    fn become_vessel(&mut self) {
        if self.is_demonic() {
            println!("{} is already a tool of The Beast!", self.name());
        } else {
            println!("{} hath become a vessel of The Beast!", self.name());
            self.is_demonic = true;
        }
    }
}

//Implement the 'Satan' trait for Goat!
impl Satan for Goat {
    //Self becomes Sheep, the type
    fn new(name: &'static str) -> Goat {
        Goat { name: name, is_demonic: false}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_demonic() {
            "666 All hail The Beast! 666"
        } else {
            "Baaaaaaaaah!"
        }
    }
    // Default methods can be overridden
    fn talk(&self) {
        println!("{} pauses briefly, glancing with its weird rectangular Goat
        pupils. . . {}", self.name, self.noise())
    }
}

fn main() {
    //Type annotation needed here
    let mut baphomet: Goat = Satan::new("Baphomet");
    baphomet.talk();
    baphomet.become_vessel();
    baphomet.talk();
}

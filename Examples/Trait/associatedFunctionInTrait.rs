trait Fly {
    fn new() -> Self;
   fn fly(&self);
   fn land(&self) {
       println!("Flyable Object now landing.");
   }
}

struct Kaftar ();

impl Fly for Kaftar {
   fn new() -> Self {
       return Kaftar{};
   }

   fn fly(&self) {
       println!("Kafter The Kakol Be Sar is flying");
   }
}


fn main() {
    let kakol_be_sar = Kaftar::new();
   kakol_be_sar.fly();
}


fn main() {

    struct Vehicle {
        make: String,
        model: String,
        year: i32,
        color: VehicleColor,
    }

    enum VehicleColor {
        Black,
        White,
        Red,
        Blue,
        Green,
    }

    impl Vehicle {
        fn paint (&self, color: VehicleColor) {
            self.color = color;
        }
    }
}
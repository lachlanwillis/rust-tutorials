// The newtype idiom gives compile time guarantees that the right type of value is supplied to a program.
// For example, an age verification function that checks age in years, must be given a value of type Years.
// It does not matter that the Days and Years type are both structs of i64
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}

// To obtain the newtype's value as the base type, you may use tuple syntax like so:
// struct Years(i64);

// fn main() {
//     let years = Years(42);
//     let years_as_primitive: i64 = years.0;
// }

// fn =>
// fn Once =>  this closure can only be called once => because you jmove into the closure.
// fn Mut => you can pass in a mutable reference into a closure also, which means
// you have a mutable reference to the value.

// Iterators : iterators are a way of processing series of elements
// just like for loops, you can use with or without method chaining.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn new() -> Self {
        Self { shirts: Vec::new() }
    }
    fn giveaway3(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        if let Some(value) = user_preference {
            value
        } else {
            self.most_stocked()
        }
    }

    fn giveaway2(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        match user_preference {
            Some(value) => value,
            None => self.most_stocked(),
        }
    }

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // user_preference.unwrap_or(self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // whenever you see this => || in your closure its synonymous to ()
    let add = |a, b| a + b;

    let res = add(2, 3);
    println!("Our response {res}");

    // this won't work because it infers the type from first usage of the closure
    // let res = add(2.0, 3.0);
    // println!("Our response {res}");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}",);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Rectangle {
    width: u8,
    height: u8,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_this() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 2).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }
}

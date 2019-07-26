use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, InputType, OutputType>
    where T: Fn(InputType) -> OutputType
{
    calculation: T,
    value: HashMap<InputType, OutputType>,
}

impl<T, InputType, OutputType> Cacher<T, InputType, OutputType>
    where T: Fn(InputType) -> OutputType,
          InputType: Eq + std::hash::Hash + Copy,
          OutputType: Eq + std::hash::Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, InputType, OutputType> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: InputType) -> OutputType {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                self.value.insert(arg, (self.calculation)(arg));
                self.value(arg)
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!", 
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 23;
    let simulated_random_number = 3;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
# TypeMapVec
TypeMapVec is a simple rust library that provides a HashMap for which the key is the type of the object to add and the value is a vector of objects of type T.
This is particularly useful for data-oriented Entity-Component Systems (ECS).

TypeMapVec is 100% written in safe Rust.

## How to use
```rs
use typemapvec::TypeMapVec;

let mut tmv = TypeMapVec::new();

// Add some u32
tmv.insert(2026 as u32);
tmv.insert(42 as u32);
tmv.insert(0 as u32);

// Add some i32
tmv.insert(2026 as i32);
tmv.insert::<i32>(-42);
tmv.insert::<i32>(0);

match tmv.get_vec_ref::<u32>() {
    Some(vec) => {
        println!("I've got {} u32s in the tmv", vec.iter().len());
    },
    None => {
        println!("It looks like no u32 was added in the tmv");
    }
}

match tmv.get_vec_mut::<u32>() {
    Some(vec) => {
        vec.push(89);
    },
    None => {
        println!("It looks like no u32 was added in the tmv");
    }
}

match tmv.get_vec_ref::<u32>() {
    Some(vec) => {
        println!("I've got {} u32s in the tmv", vec.iter().len());
    },
    None => {
        println!("It looks like no u32 was added in the tmv");
    }
}
```

## Technical limitations
A TypeMapVec can only hold types with a `'static` lifetime and thus cannot hold `&` or `&mut` references. You might want to consider `std::sync::Arc` or similar methods for references.

## Contributions + AI Policy
Any contributions is welcome as long as you are capable of explaining clearly your code. Please visit [the GitHub repository](https://github.com/Oxey405/typemapvec-rs) to contribute or file an issue.

While this code wasn't made by AI, I have used the help of an LLM on how to choose the type in the underlying HashMap.
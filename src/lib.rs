#![crate_name = "typemapvec"]
use std::{any::{Any, TypeId}, collections::HashMap};

type VecMap = HashMap<TypeId, Box<dyn Any + 'static>>;

pub struct TypeMapVec {
    /*Here we cannot write Box<Vec<dyn Any>> because dyn Any doesn't have a known size at compile time
    * however the methods implemented for TypeMapVec make it impossible to add anything that isn't a Vec<T>
    */
    vec_map: VecMap
}

///
/// TypeMapVec provides a specialized HashMap that can be used to store objects of the same type T in an Vec<T> while having N different types stored
/// 
impl TypeMapVec {
    /// Creates a new instance of a TypeMapVec
    pub fn new() -> Self {
        Self {
            vec_map: HashMap::new()
        }
    }
    /// Adds a new object of type T
    /// This creates a new Vec<T> if none exists
    /// If another object of type T is in the TypeMapVec, it gets added to the corresponding Vec<T>
    pub fn insert<T: 'static>(&mut self, obj: T) {
        let type_id = obj.type_id();
        if self.vec_map.contains_key(&type_id) {
            let vec= self.vec_map.get_mut(&type_id).unwrap();
            let vec = vec.downcast_mut::<Vec<T>>().unwrap();
            vec.push(obj);
        } else {
            let mut vec = Vec::<T>::new();
            vec.push(obj);
            self.vec_map.insert(type_id, Box::new(vec));
        }
    }

    /// Gets a reference to the Vec<T> if it exists
    pub fn get_vec_ref<T: 'static>(&self) -> Option<&Vec<T>> {
        let type_id = TypeId::of::<T>();
        match self.vec_map.get(&type_id) {
            Some(v) => {
                v.downcast_ref::<Vec<T>>()
            },
            None => {
                return None
            }
        }
    }

    /// Gets a mutable reference to the Vec<T> if it exits
    pub fn get_vec_mut<T: 'static>(&mut self) -> Option<&mut Vec<T>> {
        let type_id = TypeId::of::<T>();
        match self.vec_map.get_mut(&type_id) {
            Some(v) => {
                v.downcast_mut::<Vec<T>>()
            },
            None => {
                return None
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestStructA {
        a: u32,
        b: String
    }


    #[test]
    fn typemapvec_get_vec_ref() {
        let mut map: VecMap = HashMap::new();
        map.insert(TypeId::of::<usize>(), Box::new(vec![2026 as usize]));

        let tmv = TypeMapVec {
            vec_map: map
        };

        assert_eq!(tmv.get_vec_ref::<usize>().expect("TMV get_vec_ref should return Some here")[0], 2026);
    }

    #[test]
    fn typemapvec_add_one_simpletype() {
        let mut tmv = TypeMapVec::new();
        tmv.insert(42 as u32);

        assert_eq!(tmv.get_vec_ref::<u32>().expect("TMV get_vec_ref should return Some here")[0], 42);
    }

    #[test]
    fn typemapvec_add_many_simpletype() {
        let mut tmv = TypeMapVec::new();
        tmv.insert(24 as u32);
        tmv.insert(42 as u32);
        assert_eq!(tmv.get_vec_ref::<u32>().expect("TMV get_vec_ref should return Some here")[0], 24);
        assert_eq!(tmv.get_vec_ref::<u32>().expect("TMV get_vec_ref should return Some here")[1], 42);
    }

    #[test]
    fn typemapvec_add_one_complextype() {
        let mut tmv = TypeMapVec::new();
        tmv.insert(TestStructA {
            a: 42,
            b: String::from("The Answer to the Ultimate Question of Life, the Universe, and Everything")
        });

        let data = tmv.get_vec_ref::<TestStructA>();
        assert_ne!(data, None);
        let data = data.unwrap();
        assert!(&data[0].a == &42 && &data[0].b == "The Answer to the Ultimate Question of Life, the Universe, and Everything");
    }

    #[test]
    fn typemapvec_add_many_complextype() {
        let mut tmv = TypeMapVec::new();
        tmv.insert(TestStructA {
            a: 42,
            b: String::from("The Answer to the Ultimate Question of Life, the Universe, and Everything")
        });

        tmv.insert(TestStructA {
            a: 24,
            b: String::from("Well that doesn't seem quite right does it?")
        });

        let data = tmv.get_vec_ref::<TestStructA>();
        assert_ne!(data, None);
        let data = data.unwrap();
        assert!(&data[0].a == &42 && &data[0].b == "The Answer to the Ultimate Question of Life, the Universe, and Everything");
        assert!(&data[1].a == &24 && &data[1].b == "Well that doesn't seem quite right does it?");

    }

    #[test]
    fn typemapvec_add_many_different_types() {
        let mut tmv = TypeMapVec::new();
        
        tmv.insert(43 as u32);
        tmv.insert(24 as u32);

        tmv.insert(-8 as i32);
        tmv.insert(-167 as i32);
        
        tmv.insert(true);
        tmv.insert(false);
        tmv.insert(true);

        // Note: After the previous tests pass I assume the actual contents is correct
        let u32_vec_size = tmv.get_vec_ref::<u32>().unwrap().iter().len();
        let i32_vec_size = tmv.get_vec_ref::<i32>().unwrap().iter().len();
        let bool_vec_size = tmv.get_vec_ref::<bool>().unwrap().iter().len();

        assert_eq!(u32_vec_size, 2);
        assert_eq!(i32_vec_size, 2);
        assert_eq!(bool_vec_size, 3);
    }


    #[test]
    fn typemapvec_get_vec_ref_no_member_of_type() {
        let mut tmv = TypeMapVec::new();

        tmv.insert(true);
        tmv.insert(false);
        tmv.insert(true);

        assert!(tmv.get_vec_ref::<u32>().is_none());
    }

    #[test]
    fn typemapvec_get_vec_mut() {
        const CHANGED_VALUE: &str = "The value changed";
        let mut tmv = TypeMapVec::new();
        
        tmv.insert(43 as u32);
        tmv.insert(24 as u32);

        tmv.insert(String::from("This value will get replaced"));
        tmv.insert(String::from("This value will remain untouched"));

        // We drop the mut so we can access it statically later
        {
            let string_vec = tmv.get_vec_mut::<String>().unwrap();
            string_vec[0] = String::from(CHANGED_VALUE);

        }

        let string_vec = tmv.get_vec_ref::<String>().unwrap();
        assert_eq!(string_vec[0], CHANGED_VALUE);
    }

}

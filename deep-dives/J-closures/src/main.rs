#![allow(dead_code)]

fn main() {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    fn map_with_bare_function_with_args<U, V>(self, some_func: fn(T, U) -> V, args: U) -> Maybe<V> {
        match self {
            Maybe::Just(t) => Maybe::Just(some_func(t, args)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    fn map_with_bare_function_with_2_args<U, V, W>(
        self,
        some_func: fn(T, U, V) -> W,
        arg1: U,
        arg2: V,
    ) -> Maybe<W> {
        match self {
            Maybe::Just(t) => Maybe::Just(some_func(t, arg1, arg2)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    fn map_with_bare_function<U>(self, some_func: fn(T) -> U) -> Maybe<U> {
        match self {
            Maybe::Just(t) => Maybe::Just(some_func(t)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }

    fn map_with_closure<U, F>(self, some_closure: F) -> Maybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Maybe::Just(t) => Maybe::Just(some_closure(t)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MyVec<T> {
    values: Vec<T>,
}

impl<T> MyVec<T> {
    fn map_values<U, F>(self, mut f: F) -> MyVec<U>
    where
        F: FnMut(T) -> U,
    {
        let mut output = MyVec { values: Vec::new() };
        for value in self.values.into_iter() {
            let outval = f(value);
            output.values.push(outval);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_no_closure() {
        fn my_func(x: i32) -> i32 {
            x + 333
        }

        fn my_other_func(x: i32) -> i32 {
            x + 444
        }

        let mb = Maybe::Just(123);
        let actual = mb.map_with_bare_function(my_func);
        let expect = Maybe::Just(456);
        assert_eq!(actual, expect);

        let mb = Maybe::Just(123);
        let actual = mb.map_with_bare_function(my_other_func);
        let expect = Maybe::Just(567);
        assert_eq!(actual, expect);

        // closures can be coerced into function pointers
        // if they are 'simple'

        let my_closure = |x: i32| -> i32 { x + 333 };
        let f: fn(i32) -> i32 = my_closure;

        let mb = Maybe::Just(123);
        let actual = mb.map_with_bare_function(f);
        let expect = Maybe::Just(456);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_bare_function_with_args() {
        fn my_func(x: i32, y: i32) -> i32 {
            x + y
        }

        let mb = Maybe::Just(123);
        let actual = mb.map_with_bare_function_with_args(my_func, 444);
        let expect = Maybe::Just(567);
        assert_eq!(actual, expect);

        // // // How???
        // fn my_func2(x: i32, y: i32, z: i32) -> i32 {
        //     x + y + z
        // }

        // let mb = Maybe::Just(123);
        // let actual = mb.map_with_bare_function_with_args(my_func, 444);
        // let expect = Maybe::Just(567);
        // assert_eq!(actual, expect);

        struct MyArgs {
            y: i32,
            z: i32,
        }

        fn my_func3(x: i32, args: MyArgs) -> i32 {
            x + args.y + args.z
        }

        let mb = Maybe::Just(123);
        let y = 222;
        let z = 333;
        let actual = mb.map_with_bare_function_with_args(my_func3, MyArgs { y, z });
        let expect = Maybe::Just(678);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_map_with_closure() {
        let mb = Maybe::Just(123);
        let y = 333;
        let z = 111;
        let actual = mb.map_with_closure(|x| x + y + z);
        let expect = Maybe::Just(567);
        assert_eq!(actual, expect);

        fn my_func(x: i32) -> i32 {
            x + 333
        }
        let mb = Maybe::Just(123);
        let actual = mb.map_with_closure(my_func);
        let expect = Maybe::Just(456);
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_spawn() {
        let x = String::from("1234 is a big number");

        let handle = std::thread::spawn(move |/* no arguments */| {
            println!("hello {x}"); // is x still alive? might have been destroyed by now
            // returning 'unit' or () or just 'nothing' (in C: void)
        });

        // println!("hello {x}");   // <- will not work - has been moved into thread
        handle.join().unwrap();
    }

    #[test]
    fn test_spawn_ref() {
        let x = String::from("1234 is a big number");
        let y = &x;

        let handle = std::thread::spawn(move || {
            // println!("hello {y}"); // <- Bad! will not compile because although y is captured, y is a reference to x, therefore
            // the closure does NOT own all its values
        });

        // println!("hello {x}");   // <- will not work - has been moved into thread
        handle.join().unwrap();
    }

    #[test]
    fn test_map_values() {
        let mv = MyVec {
            values: vec![1, 2, 3, 4],
        };
        let actual = mv.map_values(|val: i32| val + 1);
        let expect = MyVec {
            values: vec![2, 3, 4, 5],
        };
        assert_eq!(actual, expect);
    }
}

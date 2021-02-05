/// A macro that makes creating [`rmpv`] values easier
///
/// Create an nil value by just calling value with no arguments:
/// ```
/// value!()
/// ```
///
/// Create any single Value by just calling `value!` with the expression that you want to convert.
/// This uses `Value::from` the hood
/// ```
/// value!(true)
/// value!(3)
/// ```
///
/// Create an array of Values using syntax similar to `vec!`:
/// ```
/// value!([]) // empty array
/// value!([3, 4, 2, 234, 213]);
/// value!([true, false, true true]);
/// value!([true; 10]); // repeat a value
/// ```
///
/// Create a map of Values using syntax similar to ruby
/// ```
/// value!([=>]) // empty map
/// value!(["Cat" => false, "Dog" => true, "Fish" => false]);
/// value!(["Dog" => true; 9]); // repeat a key value pair
/// ```
/// *Note*: Value::Map is just a `Vec<(Value, Value)>`, not a `HashMap`
///
/// [`rmpv`]: https://docs.rs/rmpv/0.4.7/rmpv/
#[macro_export]
macro_rules! value {
    () => {
        nvim_rs::Value::Nil
    };

    ($x:expr) => {
        nvim_rs::Value::from($x)
    };

    ([]) => {
        {
            use nvim_rs::Value;
            let v: Vec<Value> = Vec::new();
            v
        }
    };

    ([$elem:expr; $n:expr]) => {
        {
            use nvim_rs::Value;

            let n = $n;
            let value = Value::from($elem);
            let vec: Vec<Value> = std::vec::Vec::with_capacity(n);

            for _ in 0..n { v.push(value.clone()) }

            v
        }
    };

    ([$($x:expr),+ $(,)?]) => {
        {
            use nvim_rs::Value;
            let v: Vec<Value> = <[_]>::into_vec(std::box::Box::new([$(Value::from($x)),+]));
            v
        }
    };

    ([=>]) => {
        {
            use nvim_rs::Value;
            let v: Vec<(Value, Value)> = Vec::new();
            v
        }
    };

    ([$k:expr => $v:expr; $n:expr]) => {
        {
            use nvim_rs::Value;

            let n = $n;
            let tuple = (Value::from($k), Value::from($v));
            let vec: Vec<(Value, Value)> = std::vec::Vec::with_capacity(n);

            for _ in 0..n { v.push(tuple.clone()) }

            v
        }
    };

    ([$($k:expr => $v:expr),+ $(,)?]) => {
        {

            use nvim_rs::Value;
            let v: Vec<Value> = <[_]>::into_vec(std::box::Box::new([$((Value::from($k), Value::from($v))),+]));
            v
        }
    }
}

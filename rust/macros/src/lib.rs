#[macro_export]
macro_rules! hashmap {
    () => { crate::HashMap::new() };
    //Handle trailing comma
    ($($key:expr => $val:expr,)*) =>
    {
    {
        let mut temp = crate::HashMap::new();
        $(
            temp.insert($key,$val);
        )*
        temp
    }
    };
    ($($key:expr => $val:expr),*) =>
    {
    {
        let mut temp = crate::HashMap::new();
        $(
            temp.insert($key,$val);
        )*
        temp
    }
    };
}

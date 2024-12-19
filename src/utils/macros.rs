#[macro_export]
macro_rules! hashset {
    () => {
        ::std::collections::HashSet::new()
    };
    
    ($elem:expr; $n:expr) => {
        {
            let mut set = ::std::collections::HashSet::new();
            for _ in 0..$n {
                set.insert($elem);
            }
            set
        }
    };
    
    ($($x:expr),+ $(,)?) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(
                set.insert($x);
            )+
            set
        }
    };
}
trait Push<E>{
    type Output;
    fn push(self, value:E) -> Option<Self::Output>;
}

macro_rules! tuple_push {
    ($($x:ident,)*) => {
        impl<E, $ ( $ x ) , *> Push<E> for ($ ( $ x ,) *){
            type Output = ( $ ( $ x , )  *  E,);
            fn push(self, value:E) -> Option<Self::Output> {
                #[allow(non_snake_case)]
                let ($ ( $ x ,)  *) = self;
                Some(($ ( $ x ,) * value, ))
            }
        }
    }
}

tuple_push!();
tuple_push!(T1,);
tuple_push!(T1,T2,);
tuple_push!(T1,T2,T3,);

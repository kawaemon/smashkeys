pub fn eq<A, B>((a, b): (A, B)) -> bool
where
    A: PartialEq<B>,
{
    a == b
}

pub fn eq_ref<A, B>((a, b): &(A, B)) -> bool
where
    A: PartialEq<B>,
{
    a == b
}

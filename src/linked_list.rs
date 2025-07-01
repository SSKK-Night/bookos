pub struct ListItem<'a, T> {
    value T,
    next: Option<&'a mut ListItem<'a, T>>,
}

pub struct LinkedList<'a, T> {
    head: Option<&'a mut ListItem<'a, T>>,
    last: Option<&'a mut ListItem<'a, T>>,
}
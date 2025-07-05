#[cfg(test)]

use core::ptr::NonNull;
use core::marker::PhantomData;

pub struct ListItem<'a, T> {
    value: T,
    next: Option<NonNull<ListItem<'a, T>>>
    marker: PhantomData<&'a T>,
}

pub struct LinkedList<'a, T> {
    head: Option<NonNull<ListItem<'a, T>>>,
    last: Option<NonNull<ListItem<'a, T>>>,
    marker: PhantomData<&'a T>,
}

mod test {
    use ListItem;
    use LinkedList;

    #[test]
    fn test_list() {
        let mut item1 = ListItem::new(1);
        let mut item2 = ListItem::new(2);
        let mut item3 = ListItem::new(3);
        let mut list = LinkedList::new();

        list.push(&mut item1);
        list.push(&mut item2);
        list.push(&mut item3);

        assert_eq!(Some(&mut 1), list.head_mut());
        let result1: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 2), list.head_mut());
        let result2: &u32 = list.pop().unwrap();
        assert_eq!(Some(&mut 3), list.head_mut());
        let result3: &u32 = list.pop().unwrap();
        assert_eq!(1, *result1);
        assert_eq!(2, *result2);
        assert_eq!(3, *result3);

        assert!(list.is_empty());

        let mut item4 = ListItem::new(4);
        let mut item5 = ListItem::new(5);
        list.push(&mut item4);
        list.push(&mut item5);

        let result4: &u32 = list.pop().unwrap();
        let result5: &u32 = list.pop().unwrap();
        assert_eq!(4, *result4);
        assert_eq!(5, *result5);

        assert!(list.is_empty())
    }
}
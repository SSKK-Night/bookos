use crate::linked_list::{LinkedList, ListItem};
use crate::process::Process;

pub struct Scheduler<'a> {
    list: LinkedList<'a, Process<'a>>,
}
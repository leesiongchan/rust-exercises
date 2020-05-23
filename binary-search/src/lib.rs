pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: Ord,
{
    let mut slice = array.as_ref();
    let base = slice.len() >> 1;
    let (head, tail) = slice.split_at(base);

    if let Some(mid) = tail.first() {
        if *mid == key {
            return Some(base);
        } else if *mid < key {
            slice = &tail[1..];
            return find(slice, key).map(|v| v + base + 1);
        } else if *mid > key {
            slice = head;
            return find(slice, key);
        }
    }
    None
}

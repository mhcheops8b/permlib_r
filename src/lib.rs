// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

fn swap<T:Copy>(a:&mut[T], i:usize, j:usize)
{
    if i != j {
        let pom = a[i];
        a[i] = a[j];
        a[j] = pom;
    }
}

fn swap_vec<T:Copy>(a:&mut Vec<T>, i:usize, j:usize)
{
    if i != j {
        let pom = a[i];
        a[i] = a[j];
        a[j] = pom;
    }
}

fn reverse<T:Copy>(a:&mut[T], from:usize, to:usize)
{
    let mut idx = from;
    let mut to_idx = to;

    while idx < to_idx
    {
        swap(a, idx, to_idx);
        idx+=1;
        to_idx-=1;
    }
}

fn reverse_vec<T:Copy>(a:&mut Vec<T>, from:usize, to:usize)
{
    let mut idx = from;
    let mut to_idx = to;

    while idx < to_idx
    {
        swap_vec(a, idx, to_idx);
        idx+=1;
        to_idx-=1;
    }
}

pub fn next_perm<T:Copy+PartialOrd>(a:&mut[T], len:usize) -> bool
{
    // there no next_perm on empty array
    if len == 0 {
        return false;
    }

    let a_size = len - 1; //a.len() - 1;
    let mut i:isize = a_size as isize;
    while (i - 1) >= 0 && a[(i-1) as usize] > a[i as usize]
    {
        i-=1;
    }

    if i-1 < 0
    {
        return false;
    }

    let mut j:usize = a_size;

    while a[(i-1) as usize] > a[j] {
        j-=1
    }
    
    swap(a, (i-1) as usize, j);

    reverse(a, i as usize, a_size);
    
    return true;
}

pub fn next_perm_vec<T:Copy+PartialOrd>(a:&mut Vec<T>, len:usize) -> bool
{
    // there no next_perm on empty array
    if len == 0 {
        return false;
    }

    let a_size = len - 1; //a.len() - 1;
    let mut i:isize = a_size as isize;
    while (i - 1) >= 0 && a[(i-1) as usize] > a[i as usize]
    {
        i-=1;
    }

    if i-1 < 0
    {
        return false;
    }

    let mut j:usize = a_size;

    while a[(i-1) as usize] > a[j] {
        j-=1
    }
    
    swap_vec(a, (i-1) as usize, j);

    reverse_vec(a, i as usize, a_size);
    
    return true;
}

#[allow(dead_code)]
fn prev_perm<T:Copy+PartialOrd>(a:&mut[T], len:usize) -> bool
{
    // there no prev_perm on empty array
    if a.len() == 0 {
        return false;
    }

    let a_size = len - 1;
    let mut i:isize = a_size as isize;

    while (i - 1) >= 0 && a[(i-1) as usize] < a[i as usize]
    {
        i-=1;
    }

    if i-1 < 0
    {
        return false;
    }

    let mut j:usize = a_size;

    while a[(i-1) as usize] < a[j] {
        j-=1
    }
    
    swap(a, (i-1) as usize, j);

    reverse(a, i as usize, a_size);
    
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap()
    {
        let mut a = vec![3,8,4,1];

        swap(&mut a, 1, 2);

        assert_eq!(a, vec![3,4,8,1]);
    }

    #[test]
    fn test_next_perm() {
        let mut p = vec![0,2,4,1];

        let b = next_perm(&mut p, 4);

        assert_eq!(b, true);

        assert_eq!(p, vec![0,4,1,2]);
    }

    #[test]
    fn test_next_perm_end() {
        let mut p = vec![4,2,1,0];

        let b = next_perm(&mut p, 4);

        assert_eq!(b, false);
    }

    #[test]
    fn test_prev_perm() {
        let mut p = vec![0,2,4,1];

        let b = prev_perm(&mut p, 4);

        assert_eq!(b, true);

        assert_eq!(p, vec![0,2,1,4]);
    }

       #[test]
    fn test_prev_perm_begin() {
        let mut p = vec![0,1,2,4];

        let b = prev_perm(&mut p, 4);

        assert_eq!(b, false);

    }
}
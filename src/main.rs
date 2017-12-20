fn main() {
    let step = std::env::args()
        .nth(1)
        .unwrap_or("3".to_string())
        .parse()
        .unwrap();

    let sequence = spinlock_history(2018, step);
    let &(_, last) = sequence.last().unwrap();

    let result = resolve_value(&sequence, last + 1).unwrap();

    println!("Result = {}", result);

    let sequence = spinlock_history(50000001, step);
    let result = resolve_value(&sequence, 1).unwrap();

    println!("Huge = {}", result);
}

struct SpinLock {
    next: usize,
    size: usize,
    step: usize,
}

impl Iterator for SpinLock {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.size += 1;
        let next = self.next_position();
        Some(std::mem::replace(&mut self.next, next))
    }
}

impl SpinLock {
    fn new(step: usize) -> Self {
        SpinLock {
            next: 0,
            size: 0,
            step,
        }
    }

    fn next_position(&self) -> usize {
        ((self.next + self.step) % self.size) + 1
    }
}

fn spinlock_history(times: usize, step: usize) -> Vec<(usize, usize)> {
    SpinLock::new(step).enumerate().take(times).collect()
}

fn resolve_value<V: AsRef<[(usize, usize)]>>(sequence: V, mut pos: usize) -> Option<usize> {
    sequence.as_ref().iter().rev().filter_map(|&(n, v)|
        {
            match v {
                x if x == pos => Some(n),
                x if x < pos => {
                    pos -= 1;
                    None
                }
                x if x > pos => { None }
                _ => unreachable!()
            }
        }
    ).nth(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn short_sequence() {
        assert_eq!(vec![(0, 0), (1, 1), (2, 1), (3, 2)],
                   spinlock_history(4, 3))
    }

    #[test]
    fn medium_sequence() {
        assert_eq!(vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 1), (6, 5), (7, 2),
                        (8, 6), (9, 1)],
                   spinlock_history(10, 3))
    }

    #[test]
    fn test_resolve_value() {
        let sequence = vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 2), (5, 1), (6, 5), (7, 2),
                            (8, 6), (9, 1)];
        assert_eq!(Some(7), resolve_value(&sequence, 3))
    }

    #[test]
    fn integration() {
        let sequence = spinlock_history(2018, 3);
        let &(_, last) = sequence.last().unwrap();

        assert_eq!(638, resolve_value(&sequence, last + 1).unwrap());
    }

    #[test]
    fn try_to_create_a_50000000_sequence() {
        let sequence = spinlock_history(50000001, 3);

        assert_eq!(1222153, resolve_value(&sequence, 1).unwrap());
    }
}

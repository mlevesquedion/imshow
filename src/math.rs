#[macro_export]
macro_rules! linspace {
    ($from:expr, $to:expr, $count:expr) => {
        Linspace::new($from, $to, $count)
    };
}

#[derive(Debug)]
pub struct Linspace {
    current: f64,
    target: f64,
    step: f64,
}

impl Linspace {
    pub fn new(from: f64, to: f64, count: usize) -> Self {
        Self {
            current: from,
            target: to,
            step: to / count as f64,
        }
    }
}

impl Iterator for Linspace {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if (self.current + self.step) > self.target {
            None
        } else {
            let val = (self.current, self.current + self.step);
            self.current += self.step;
            Some(val)
        }
    }
}

pub fn weighted_indices(lo: f64, hi: f64) -> Vec<(usize, f64)> {
    indices(lo, hi)
        .into_iter()
        .zip(normalized_by_sum(weights(lo, hi)).into_iter())
        .collect()
}

fn normalized_by_sum(values: Vec<f64>) -> Vec<f64> {
    let sum: f64 = values.iter().sum();
    values.into_iter().map(|x| x / sum).collect()
}

fn weights(lo: f64, hi: f64) -> Vec<f64> {
    if lo + 1.0 > hi {
        return vec![hi - lo];
    }

    let mut weights = vec![lo.ceil() - lo];
    let mut lo = lo.ceil() + 1.0;

    while lo < hi {
        weights.push(1.0);
        lo += 1.0;
    }

    weights.push(1.0 - (lo - hi));

    weights
}

fn indices(lo: f64, hi: f64) -> Vec<usize> {
    let mut lo = lo.floor();
    let mut indices = Vec::new();
    while lo < hi {
        indices.push(lo as usize);
        lo += 1.0;
    }

    indices
}

#[test]
fn test_linspace() {
    let steps: Vec<(f64, f64)> = linspace!(0.0, 10.0, 4).collect();
    let expected = vec![(0.0, 2.5), (2.5, 5.0), (5.0, 7.5), (7.5, 10.0)];
    assert_eq!(expected, steps);
}

#[test]
fn test_normalized_by_sum() {
    let values = vec![10.0, 40.0];
    assert_eq!(vec![0.2, 0.8], normalized_by_sum(values));
}

#[cfg(test)]
mod weights_tests {
    use super::*;

    macro_rules! assert_float_eq {
        ($x:expr, $y:expr) => {
            assert!(($x - $y).abs() <= f64::EPSILON)
        };
        ($x:expr, $y:expr, $message:expr) => {
            assert!(($x - $y).abs() <= f64::EPSILON, $message)
        };
    }

    #[test]
    fn test_gap_between_lo_and_hi_less_than_1() {
        let bounds = (0.2, 0.7);
        let expected = vec![0.5];
        let actual = weights(bounds.0, bounds.1);
        assert_float_eq!(expected[0], actual[0]);
    }

    #[test]
    fn test_many_weights() {
        let bounds = (1.4, 4.2);
        let expected = vec![0.6, 1.0, 1.0, 0.2];
        let actual = weights(bounds.0, bounds.1);

        println!("{:?}", actual);

        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_float_eq!(e, a, format!("{} != {}", e, a));
        }
    }
}

#[cfg(test)]
mod indices_tests {
    use super::*;

    #[test]
    fn test_gap_between_lo_and_hi_less_than_1() {
        let bounds = (0.2, 0.7);
        let expected = vec![0];
        let actual = indices(bounds.0, bounds.1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_indices() {
        let bounds = (1.4, 4.2);
        let expected = vec![1, 2, 3, 4];
        let actual = indices(bounds.0, bounds.1);
        assert_eq!(expected, actual);
    }
}
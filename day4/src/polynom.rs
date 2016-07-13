
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{ Add };

#[derive(PartialEq, Eq)]
struct Element(i64);

impl Debug for Element {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}*x", self.0)
    }
}

impl<'a> Add for &'a Element {
    type Output = Element;

    fn add(self, rhs: &'a Element) -> Element {
        Element(self.0 + rhs.0)
    }
}

#[derive(PartialEq, Eq)]
struct Polynom(Vec<Element>);

impl Polynom {

    fn from_iter<I: Iterator<Item=Element>>(col: I) -> Polynom {
        Polynom(col.collect::<Vec<Element>>())
    }

    fn from_int_array(array: &[i64]) -> Polynom {
        Polynom::from_iter(array.iter().filter(|&x| *x > 0i64).map(|x| Element(*x)))
    }

    fn calculate(&self, mun: i64) -> i64 {
        self.0.iter().enumerate().fold(0i64, |acc, (idx, x)| acc + (x.0 * mun.pow(idx as u32)))
    }

}

impl ToString for Polynom {
    fn to_string(&self) -> String {
        self.0.iter()
            .enumerate()
            .map(|(idx, ref x)| format!("{:?}^{}", x, idx))
            .collect::<Vec<String>>()
            .as_slice()
            .join(" + ")
    }
}

impl Add for Polynom {
    type Output = Polynom;

    fn add(self, me: Polynom) -> Polynom {
        let mut iter = me.0.iter();
        Polynom::from_iter(self.0.iter().map(|e| e + iter.next().unwrap()))
    }
}

impl<'a> Add for &'a [Element] {
    type Output = Polynom;

    fn add(self, me: &'a [Element]) -> Polynom {
        let mut iter = me.iter();
        Polynom::from_iter(self.0.iter().map(|e| e + iter.next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    
    use super::Polynom;

    #[test]
    fn test_calculate() {
        let pol = Polynom::from_int_array(&[1, 2, 3]);
        assert_eq!(pol.calculate(3), 34i64);
    }

    #[test]
    fn test_to_string() {
        let pol = Polynom::from_int_array(&[1, 2, 3]);
        assert_eq!(pol.to_string(), "1*x^0 + 2*x^1 + 3*x^2");
    }

    #[test]
    fn test_polynom_sum() {
        let pol1 = Polynom::from_int_array(&[1, 2, 3]);
        let pol2 = Polynom::from_int_array(&[1, 2, 3]);

        assert_eq!((pol1 + pol2).to_string(), "2*x^0 + 4*x^1 + 6*x^2");
    }

}

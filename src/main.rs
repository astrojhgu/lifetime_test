use num_traits::float::Float;
use num_traits::Num;
use std::ops::{Add, Mul, Sub};


pub trait InnerProdSpace<'a, Scalar>
    where
        Scalar: Float,
        Self: 'a + Sized,
        &'a Self: Add<Output = Self>,
        &'a Self: Sub<Output = Self>,
        &'a Self: Mul<Scalar, Output = Self>,
{
    fn dot(&self, rhs: &Self) -> Scalar;

    fn distance_to(&'a self, rhs: &'a Self) -> Scalar
    {
        let d=self-rhs;
        d.dot(&d).sqrt()
    }
}


pub fn kmeans<'a, Scalar, T>(points:Vec<T>, mut seeds:Vec<T>)->Vec<Vec<T>>
    where T:InnerProdSpace<'a,Scalar>,
          &'a T: Add<Output = T>,
          &'a T: Sub<Output = T>,
          &'a T: Mul<Scalar, Output = T>,
          Scalar:Float
{
    let k=seeds.len();

    let mut cluster_id=Vec::new();
    {
        for p in &points
            {
                cluster_id.push(
                    seeds.iter().enumerate().map(|(i, s)| {
                        (i, s.distance_to(p))
                    }).min_by(|a, b| {
                        if a.1 < b.1 {
                            std::cmp::Ordering::Less
                        } else if a.1 > b.1 {
                            std::cmp::Ordering::Greater
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    }).expect("").0);
            }
    }

    //place an empty result here to make the compiler happy
    vec![vec![]]
}


fn main() {
}
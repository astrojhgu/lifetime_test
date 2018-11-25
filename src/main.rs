use num_traits::float::Float;
use num_traits::Num;
use std::ops::{Add, Mul, Sub};

pub trait LinearSpace<'a, Scalar>
    where
        Scalar: Num,
        Self: 'a + Sized,
        &'a Self: Add<Output = Self>,
        &'a Self: Sub<Output = Self>,
        &'a Self: Mul<Scalar, Output = Self>,
{
}

pub trait InnerProdSpace<'a, Scalar>: LinearSpace<'a, Scalar>
    where
        Scalar: Num,
        Self: 'a + Sized,
        &'a Self: Add<Output = Self>,
        &'a Self: Sub<Output = Self>,
        &'a Self: Mul<Scalar, Output = Self>,
{
    fn dot(&self, rhs: &Self) -> Scalar;
}

pub trait PDInnerProdSpace<'a, Scalar>: InnerProdSpace<'a, Scalar>
    where
        Scalar: Float,
        Self: 'a + Sized,
        &'a Self: Add<Output = Self>,
        &'a Self: Sub<Output = Self>,
        &'a Self: Mul<Scalar, Output = Self>,
{
    fn distance_to(&'a self, rhs: &'a Self) -> Scalar {
        let d=self-rhs;
        d.dot(&d).sqrt()
        //self.dot(rhs).sqrt()
    }
}


pub fn kmeans<'a, Scalar, T>(points:Vec<T>, mut seeds:Vec<T>)->Vec<Vec<T>>
    where T:PDInnerProdSpace<'a,Scalar>,
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
    //seeds.iter_mut().for_each(|i|{let a=(i as &T)*Scalar::zero();*i=a;});

    vec![vec![]]
}


fn main() {
}
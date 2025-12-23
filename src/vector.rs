use anyhow::Result;
use anyhow::anyhow;
use std::ops::AddAssign;
use std::ops::{Deref};
pub struct Vector<T> 
{
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        let data1 = data.into();
        Vector { data: data1 }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn dot_prosuct<T>(a: Vector<T>, b:Vector<T>) ->Result<T>
where T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Default + AddAssign
{
    if a.len() != b.len() {// a.len => a.data.len() (Deref trait)
        return Err(anyhow!("Vectors must be of the same length"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}
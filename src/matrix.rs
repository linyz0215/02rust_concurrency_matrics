use anyhow::{anyhow,Result};
use std::fmt::{self, Debug, Display};
use std::ops::{AddAssign, Mul};
use std::sync::mpsc;   
use std::thread;
use crate::vector::{Vector,dot_prosuct};
const NUM_THREADS: usize = 4;
pub struct Matrix<T> 
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}


pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + AddAssign + Copy + Default + Send + 'static
{
    if a.cols != b.rows {
        return Err(anyhow::anyhow!("Incompatible matrix dimensions for multiplication"));
    }
    let senders = (0..NUM_THREADS).map(|_| {
        let (tx, rx) = mpsc::channel::<Msg<T>>();//创建通道
        thread::spawn(move || {//创建线程
            for msg in rx {
                let value = dot_prosuct(msg.input.row, msg.input.col)?;
                if let Err(e) = msg.sender.send(MsgOutput { idx: msg.input.idx, value }) {
                    eprintln!("Failed to send result: {:?}", e);
                }
            }
            Ok::<(), anyhow::Error>(())
        });
        tx
    }).collect::<Vec<_>>();


    let matrix_len = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers:Vec<oneshot::Receiver<MsgOutput<T>>> = Vec::with_capacity(matrix_len);

    //map
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            let col = Vector::new(b.data[j..].iter().step_by(b.cols).copied().collect::<Vec<T>>());
            let idx = i * b.cols + j;
            let input = MsgInput::new(idx, row, col);
            let (tx,rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}",e);
            }
            receivers.push(rx);
        }
    }

    //receive
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
    }
    Ok(Matrix { data, rows: a.rows, cols: b.cols })
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        let data1 = data.into();
        //data.into() 是调用 std::convert::Into trait 的方法，把 data 转换（并移动）成 Vec<T>。
        //impl Into<Vec<T>> 表示函数接受任何能被转换为 Vec<T> 的类型（例如 Vec<T> 本身，或者实现了 From<Other> for Vec<T> 的类型；From 的实现会自动提供对应的 Into）。注意 .into() 会消费所有权——data 会被 move 进那个 Vec。
        Matrix { data: data1, rows, cols }
    }
}

impl<T> Mul for Matrix<T>
where T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + AddAssign + Copy + Default + Send + 'static
{
    type Output = Result<Matrix<T>>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        multiply(&self, &rhs)
    }
}


impl<T> fmt::Display for Matrix<T>
where T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, "; ")?;
            }
        }
        write!(f,"}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Matrix {{ rows: {}, cols: {}, data: {} }}", self.rows, self.cols, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![7, 8, 9, 10, 11, 12], 3, 2);
        let c = (a * b).expect("error");

        let expected = Matrix::new(vec![58, 64, 139, 154], 2, 2);
        assert_eq!(format!("{}", c), format!("{}", expected));
        Ok(())
    }
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, sender }
    }
}


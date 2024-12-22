use core::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;

use crate::*;

fn format_row<T, U> (values: &Vec<T>, filler: U, left_terminator: U, right_terminator: U) -> Result<String, std::fmt::Error>
where T: Display, U: Display {
    let mut out = String::new();
    write!(out, "{}", left_terminator)?;
    let mut middle = String::new();
    values
        .iter()
        .map(|item| write!(middle, "{}{}", filler, item))
        .collect::<Result<Vec<_>, _>>()?;
    write!(out, "{}{}", middle, filler)?;
    write!(out, "{}", right_terminator)?;
    Ok(out)
}

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Debug for Grid<T>
where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "{TLC}{HL}")?;
        let res: Result<Vec<_>, _> = self.0[0].iter().map(|_| write!(f, "{HL}{HL}")).collect();
        res?;
        writeln!(f, "{TRC}")?;

        // middle rows
        let _: Result<Vec<_>, _> = self.0.iter().map(|row| {
            write!(f, "{VL} ")?;
            let res: Result<Vec<_>, _> = row.iter().map(|item| {
                write!(f, "{:?} ", item)
            }).collect();
            res?;
            writeln!(f, "{VL}")
        }).collect();

        // bottom row
        write!(f, "{BLC}{HL}")?;
        let res: Result<Vec<_>, _> = self.0[0].iter().map(|_| write!(f, "{HL}{HL}")).collect();
        res?;
        write!(f, "{BRC}")?;

        Ok(())
    }
}

impl<T> Display for Grid<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty_row = vec![HL; self.0[0].len()];

        // top row
        let top_row = format_row(&empty_row, HL, TLC, TRC)?;
        writeln!(f, "{}", top_row)?;

        // middle rows
        let _: Result<Vec<_>, _> = self.0.iter().map(|row| {
            writeln!(f, "{}", format_row(row, ' ', VL, VL)?)
        }).collect::<Result<Vec<_>, _>>();

        // bottom row
        let bottom_row = format_row(&empty_row, HL, BLC, BRC)?;
        write!(f, "{}", bottom_row)?;

        Ok(())
    }
}

impl<T> Grid<T>
where T: Copy {
    pub fn get(&self, pos: (usize, usize)) -> Option<T> {
        if let Some(row) = self.0.get(pos.1) {
            if let Some(item) = row.get(pos.0) {
                return Some(*item)
            }
        }

        None
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
}

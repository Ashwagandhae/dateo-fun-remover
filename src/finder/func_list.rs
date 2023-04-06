use super::func::Func;
use std::iter::Iterator;
// use a u64 to store the functions
// each pair of bits represents one of the 3 functions, or none
// 00 = none
// 01 = Sqrt
// 10 = Factorial
// 11 = Summation

#[derive(Debug, Clone)]
pub struct FuncList {
    data: u64,
    len: usize,
}

impl FuncList {
    pub fn new() -> FuncList {
        FuncList { data: 0, len: 0 }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push(&mut self, func: Func) {
        match func {
            Func::SquareRoot => {
                self.set_bits(self.len, false, true);
            }
            Func::Factorial => {
                self.set_bits(self.len, true, false);
            }
            Func::Summation => {
                self.set_bits(self.len, true, true);
            }
        }
        self.len += 1;
    }
    pub fn get(&self, index: usize) -> Func {
        let (bit_1, bit_2) = self.get_bits(index);
        match (bit_1, bit_2) {
            (false, true) => Func::SquareRoot,
            (true, false) => Func::Factorial,
            (true, true) => Func::Summation,
            _ => panic!("invalid function"),
        }
    }
    pub fn pop(&mut self) -> Func {
        let func = self.get(self.len - 1);
        self.len -= 1;
        self.data >>= 2;
        func
    }

    fn set_bits(&mut self, index: usize, bit_1: bool, bit_2: bool) {
        self.set_bit(index * 2, bit_1);
        self.set_bit(index * 2 + 1, bit_2);
    }
    fn set_bit(&mut self, bit_index: usize, bit: bool) {
        let mut mask: u64 = 1;
        // shift left by index
        mask <<= bit_index;
        if bit {
            self.data |= mask;
        } else {
            self.data &= !mask;
        }
    }
    fn get_bits(&self, index: usize) -> (bool, bool) {
        (self.get_bit(index * 2), self.get_bit(index * 2 + 1))
    }
    fn get_bit(&self, bit_index: usize) -> bool {
        let mut mask: u64 = 1;
        // shift left by index
        mask <<= bit_index;
        self.data & mask != 0
    }
    pub fn reverse(&self) -> FuncList {
        let mut new = FuncList::new();
        for i in 0..self.len {
            let back_i = self.len - i - 1;
            new.push(self.get(back_i));
        }
        new
    }
    pub fn iter(&self) -> FuncListIter {
        FuncListIter {
            list: self,
            index: 0,
        }
    }
}

impl Iterator for FuncList {
    type Item = Func;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        Some(self.pop())
    }
}

pub struct FuncListIter<'a> {
    list: &'a FuncList,
    index: usize,
}

impl<'a> Iterator for FuncListIter<'a> {
    type Item = Func;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.len {
            return None;
        }
        let func = self.list.get(self.index);
        self.index += 1;
        Some(func)
    }
}

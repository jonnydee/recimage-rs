extern crate bit_vec;

use self::bit_vec::BitVec;
use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;


pub struct Canvas<'a> {
    pub width : usize,
    pub height : usize,
    pub pixmap : &'a Pixmap,
    pub pixel : &'a Pixmap,
    empty_pixel : Pixmap,
}

impl<'a> Canvas<'a> {
    pub fn new(pixmap : &'a Pixmap, pixel : &'a Pixmap) -> Canvas<'a> {
        let empty_pixel = Pixmap::new(pixel.width, pixel.height, false);
        let width = pixmap.width * pixel.width;
        let height = pixmap.height * pixel.height;
        Canvas {
            width : width,
            height : height,
            pixmap : &pixmap,
            pixel : &pixel,
            empty_pixel : empty_pixel,
        }
    }
    
    pub fn get(&self, x : usize, y : usize) -> bool {
        let pixel_at_x = x / self.pixel.width;
        let pixel_at_y = y / self.pixel.height;
        
        let pixel_x = x % self.pixel.width;
        let pixel_y = y % self.pixel.height;

        let dot = self.pixmap.get(pixel_at_x, pixel_at_y);
        let pixel = if dot { self.pixel } else { &self.empty_pixel };
        pixel.get(pixel_x, pixel_y)
    }
    
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let dot = if self.get(x, y) { '*' } else { ' ' };
                print!("{}", dot);
            }
            println!("");
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Pixmap {
    pub width : usize,
    pub height : usize,
    buffer : BitVec,
}

impl Pixmap {
    pub fn new(width : usize, height : usize, filled : bool) -> Pixmap {
        Pixmap {
            width : width,
            height : height,
            buffer : BitVec::from_elem(width*height, filled),
        }
    }
    
    pub fn set(&mut self, x : usize, y : usize, flag : bool) {
        let offset = y*self.width + x;
        self.buffer.set(offset, flag);
    }
    
    pub fn get(&self, x : usize, y : usize) -> bool {
        let offset = y*self.width + x;
        self.buffer.get(offset).unwrap()
    }

    pub fn from_file(file_name : &str) -> Result<Pixmap, Error> {
        let f = try!(File::open(file_name));
        let file = BufReader::new(&f);
        
        let mut matrix : Vec<Vec<char>> = vec![];
        let mut width = 0;
        for line in file.lines() {
            let l = line.unwrap();
            let row : Vec<_> = l.chars().collect();
            width = cmp::max(width, row.len());
            matrix.push(row);
        }
        let height = matrix.len();
        
        let mut pixmap = Pixmap::new(width, height, false);
        for (y, row) in matrix.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let flag = *c != ' ';
                pixmap.set(x, y, flag);
            }
        }
        Ok(pixmap)
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct RecursiveImage<'a> {
    pub brush : &'a Pixmap,
    pub buffer : Pixmap,
}

impl<'a> RecursiveImage<'a> {
    pub fn draw(brush : &'a Pixmap, depth : usize) -> Pixmap {
        let width = brush.width.pow(depth as u32);
        let height = brush.height.pow(depth as u32);
        let buffer = Pixmap::new(width, height, false);
        let mut image = RecursiveImage {
            brush : brush,
            buffer : buffer,
        };
        image.draw_pixmap(depth, 0, 0);
        image.buffer
    }
    
    fn draw_pixmap(&mut self, depth : usize, row : usize, col : usize) {
        if 1 == depth {
            for y in 0..(self.brush.height) {
                for x in 0..(self.brush.width) {
                    let draw = self.brush.get(x, y);
                    if draw { 
                        self.buffer.set(col + x, row + y, true);
                    }
                }
            }
        } else {
            let width = self.brush.width.pow(depth as u32);
            let height = self.brush.height.pow(depth as u32);
            let x_step = width / self.brush.width;
            let y_step = height / self.brush.height;
            for y in 0..(self.brush.height) {
                let new_row = row + y * y_step;
                for x in 0..(self.brush.width) {
                    let new_col = col + x * x_step;
                    let draw = self.brush.get(x, y);
                    if draw {
                        self.draw_pixmap(depth - 1, new_row, new_col);
                    }
                }
            }
        }
    }
}

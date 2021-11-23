use std::convert::From;
use std::iter::Iterator;
use std::io::{Write, BufWriter};
use std::fs::File;
use crate::Point;

use crate::stream::geo::GeoStream;

pub struct PointStream {
    input: GeoStream,
    buffer: Option<Vec<u8>>, //Used by Read impl for storing partial features
    errors: Option<BufWriter<File>>
}

impl PointStream {
    pub fn new(input: GeoStream, errors: Option<String>) -> Self {
        PointStream {
            input: input,
            buffer: None,
            errors: match errors {
                None => None,
                Some(path) => Some(BufWriter::new(File::create(path).unwrap()))
            }
        }
    }

}

impl Iterator for PointStream {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Result<Point, String> = Err(String::from(""));

        while next.is_err() {
            next = match self.input.next() {
                Some(potential) => match Point::new(potential) {
                    Ok(potential) => Ok(potential),
                    Err(err) => match self.errors {
                        None => Err(err),
                        Some(ref mut file) => {
                            file.write(format!("{}\n", err).as_bytes()).unwrap();

                            Err(err)
                        }
                    }
                },
                None => { return None; }
            };
        }

        Some(next.unwrap())
    }
}

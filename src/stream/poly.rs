use std::convert::From;
use std::iter::Iterator;
use std::io::{Write, BufWriter};
use std::fs::File;
use crate::Polygon;

use crate::stream::geo::GeoStream;

pub struct PolyStream {
    input: GeoStream,
    buffer: Option<Vec<u8>>, //Used by Read impl for storing partial features
    errors: Option<BufWriter<File>>
}

impl PolyStream {
    pub fn new(input: GeoStream, errors: Option<String>) -> Self {
        PolyStream {
            input: input,
            buffer: None,
            errors: match errors {
                None => None,
                Some(path) => Some(BufWriter::new(File::create(path).unwrap()))
            }
        }
    }
}

impl Iterator for PolyStream {
    type Item = Polygon;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Result<Polygon, String> = Err(String::from(""));

        while next.is_err() {
            next = match self.input.next() {
                Some(potential) => match Polygon::new(potential) {
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

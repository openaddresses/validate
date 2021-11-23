use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::iter::Iterator;
use std::fmt::Display;

pub struct OutStream {
    output: Output
}

pub enum Output {
    File(BufWriter<File>),
    StdOut(BufWriter<std::io::StdoutLock<'static>>),
}

impl OutStream {
    pub fn new(output: Option<impl ToString>) -> Self {
        let stream = match output {
            Some(outpath) => match File::open(outpath.to_string()) {
                Ok(file) => OutStream {
                    output: Output::File(BufWriter::new(file))
                },
                Err(err) => { panic!("Unable to open output file: {}", err); }
            },
            None => {
                OutStream {
                    output: Output::StdOut(BufWriter::new(Box::leak(Box::new(io::stdout())).lock()))
                }
            }
        };

        stream
    }

    pub fn stream(mut self, mut read: impl Iterator<Item = impl Display> ) {
        for feat in read {
            let fstr = feat.to_string();
            let fbytes = fstr.as_bytes();

            match &mut self.output {
                Output::File(f) => f.write(fbytes),
                Output::StdOut(s) => s.write(fbytes)
            }.unwrap();
        }
    }
}

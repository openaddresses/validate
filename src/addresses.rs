use crate::stream::{GeoStream, PointStream};

pub fn main(args: &clap::ArgMatches) {
    for pt in PointStream::new(GeoStream::new(args.value_of("INPUT")), None) {
        print!("{}", pt);
    }
}

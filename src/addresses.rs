use crate::stream::{GeoStream, PointStream};

pub fn main(args: &clap::ArgMatches) {
    PointStream::new(GeoStream::new(args.value_of("INPUT")), None);
}

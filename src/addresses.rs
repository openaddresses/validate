use crate::stream::{GeoStream, PointStream, OutStream};

pub fn main(args: &clap::ArgMatches) {
    let stream = PointStream::new(GeoStream::new(args.value_of("INPUT")), None);

    OutStream::new(args.value_of("OUTPUT"))
        .stream(stream);
}

#![crate_name = "geos"]
#![crate_type = "lib"]

extern crate c_vec;
extern crate libc;
extern crate num;
#[cfg(any(feature = "geo", feature = "dox"))]
extern crate geo_types;
#[cfg(any(feature = "geo", feature = "dox"))]
extern crate wkt;
extern crate geos_sys;

#[cfg(all(feature = "geo", test))]
#[macro_use]
extern crate doc_comment;

#[cfg(all(feature = "geo", test))]
doctest!("../README.md");

pub(crate) mod functions;

pub use context_handle::{
    ContextHandle,
};
pub use coord_seq::{
    CoordSeq,
};
pub use enums::{
    ByteOrder,
    CoordDimensions,
    Dimensions,
    GeometryTypes,
    Ordinate,
    Orientation,
    OutputDimension,
};
#[cfg(any(feature = "v3_6_0", feature = "dox"))]
pub use enums::{
    Precision,
};
pub use functions::{
    orientation_index,
    version,
};
#[cfg(any(feature = "v3_7_0", feature = "dox"))]
pub use functions::{
    segment_intersection,
};
pub use geometry::{
    Geometry,
};
pub use prepared_geometry::{
    PreparedGeometry,
};
pub use spatial_index:: {
    SpatialIndex,
    STRtree
};
pub use wkb_writer::{
    WKBWriter,
};
pub use wkt_writer::{
    WKTWriter,
};

mod context_handle;
mod coord_seq;
mod error;
#[cfg(any(feature = "geo", feature = "dox"))]
pub mod from_geo;
mod geometry;
mod prepared_geometry;
mod spatial_index;
#[cfg(any(feature = "geo", feature = "dox"))]
pub mod to_geo;
pub use error::{
    Error,
    GResult,
};
#[cfg(any(feature = "geo", feature = "dox"))]
mod voronoi;
#[cfg(any(feature = "geo", feature = "dox"))]
pub use voronoi::{
    compute_voronoi,
};
mod enums;
mod traits;
mod wkb_writer;
mod wkt_writer;

pub(crate) use traits::{
    AsRaw,
};
pub use traits::{
    ContextInteractions,
    ContextHandling
};

#[cfg(test)]
mod test;

mod vec;
mod error;
mod vertex;
mod edge;
mod limits;

use vec::Vec3;
use vertex::Vertex;
use edge::Segment;

fn main() {
    let pt1 = Vertex::new(Vec3::new(2., 3., 4.)).unwrap();
    let pt2 = Vertex::new(Vec3::new(8., 7., 6.)).unwrap();
    let seg = Segment::new(pt1, pt2);
    println!("Segment is: {:?}", seg);
}

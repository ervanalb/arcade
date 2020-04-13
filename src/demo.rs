use arcade::vec::Vec3;
use arcade::vertex::Vertex;
use arcade::edge::Segment;

fn main() {
    let pt1 = Vertex::new(Vec3::new(2., 3., 4.)).unwrap();
    let pt2 = Vertex::new(Vec3::new(8., 7., 6.)).unwrap();
    let seg = Segment::new(pt1, pt2);
    println!("Segment is: {:?}", seg);

    //let v1 = Vertex::new(Vec3::new(0., 0., 0.)).unwrap();
    //let v2 = Vertex::new(Vec3::new(1., 1., 0.)).unwrap();
    //let v3 = Vertex::new(Vec3::new(-0.3, 2., 0.)).unwrap();

    //let arc = Arc::from_three_points(&v1, &v2, &v3).unwrap();
    //println!("Arc is: {:?}", arc);
}

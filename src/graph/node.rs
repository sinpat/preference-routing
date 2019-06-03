#[derive(Debug)]
pub struct Node {
    pub id: usize,
    pub lat: f64,
    pub long: f64,
    pub height: f64,
    pub ch_level: usize
}

impl Node {
    pub fn new(id: usize, lat: f64, long: f64, height: f64, ch_level: usize) -> Node {
        Node { id, lat, long, height, ch_level }
    }
}
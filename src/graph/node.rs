#[derive(Debug)]
pub struct Node {
    id: usize,
    lat: f64,
    long: f64,
    height: f64,
    ch_level: usize
}

impl Node {
    pub fn new(id: usize, lat: f64, long: f64, height: f64, ch_level: usize) -> Node {
        Node { id, lat, long, height, ch_level }
    }

    pub fn get_ch_level(&self) -> usize {
        self.ch_level
    }
}
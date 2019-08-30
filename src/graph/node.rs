use crate::graph::NodeId;
use crate::helpers::Coordinate;

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub height: f64,
    pub ch_level: usize,
    pub location: Coordinate,
}

impl Node {
    pub fn new(id: NodeId, lat: f64, lng: f64, height: f64, ch_level: usize) -> Node {
        let location = Coordinate { lat, lng };
        Node {
            id,
            height,
            ch_level,
            location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

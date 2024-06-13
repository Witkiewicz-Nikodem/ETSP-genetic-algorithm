use rand::distributions::{Distribution, Uniform};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Node{
    x: f64,
    y: f64,
}


impl Node {
    pub fn new_random(start: u64, end: u64) -> Self{
        let between = Uniform::new(start, end);
        let mut rng = rand::thread_rng();

        let y: u64 = between.sample(&mut rng);
        let x: u64 = between.sample(&mut rng);

        Node{x: x as f64, y: y as f64}
    }

    
    pub fn new(x: f64, y: f64) -> Self{
        Node{x,y}
    }

    pub fn as_string(&self) -> String{
        "(".to_string() + &self.x.to_string() + " , " + &self.y.to_string() + ")"
    }

}



pub fn distance(p1: Node, p2: Node) -> f64{
    let base = (p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0);
    base.sqrt()
}


#[test]
pub fn test_new_random(){
    let point = Node::new_random(0,100);
    println!("node = {:?}", point);
}

#[test]
pub fn test_distance(){
    let point1 = Node{x: 10.0, y: 10.0};
    let point2 = Node{x: 20.0, y: 10.0};

    assert_eq!(distance(point1,point2), 10.0);
}

#[test]
pub fn test_distance2(){
    let point1 = Node{x: 10.0, y: 10.0};
    let point2 = Node{x: 13.0, y: 14.0};

    assert_eq!(distance(point1,point2), 5.0);
}

#[test]
pub fn test_compare(){
    let point1 = Node{x: 10.0, y: 10.0};
    let point2 = Node{x: 13.0, y: 14.0};

    assert_eq!(point1 == point2, false);
}

#[test]
pub fn test_compare2(){
    let point1 = Node{x: 10.0, y: 10.0};
    let point2 = Node{x: 10.0, y: 10.0};

    assert_eq!(point1 == point2, true);
}

#[test]
pub fn test_as_string(){
    let point1 = Node{x: 10.1, y: 10.1};

    assert_eq!(point1.as_string(), "(10.1 , 10.1)");
}
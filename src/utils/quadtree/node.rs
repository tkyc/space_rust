use crate::actors::Actor;
//TODO: Collison through hashmap with adjmtrx -- key is coord and values are actors
//collisions occur when map collisions occur?



pub struct QuadNode<'a, T> {
    pub tl: Option<Box<QuadNode<'a, T>>>, //top-left
    pub tr: Option<Box<QuadNode<'a, T>>>, //top-right
    pub bl: Option<Box<QuadNode<'a, T>>>, //bottom-left
    pub br: Option<Box<QuadNode<'a, T>>>, //bottom-right
    xrange: (f32, f32),
    yrange: (f32, f32),
    val: Option<&'a T>,
}



impl<'a, T> QuadNode<'a, T> {

    //Area of ship/enemy is 300px
    const LEAF_X_LIMIT: f32 = 25.0;
    const LEAF_Y_LIMIT: f32 = 18.75;

    pub fn new(xrange: (f32, f32), yrange: (f32, f32)) -> Box<QuadNode<'a, T>> {
        Box::new(QuadNode {
            tl: None,
            tr: None,
            bl: None,
            br: None,
            xrange: xrange,
            yrange: yrange,
            val: None,
        })
    }

    pub fn insert(&mut self, element: &'a T) -> bool
        where T: Actor {
    }

    fn is_leaf_node(&self) -> bool {

        !((self.xrange.1 - self.xrange.0) > QuadNode::<()>::LEAF_X_LIMIT) ||
        !((self.yrange.1 - self.yrange.0) > QuadNode::<()>::LEAF_Y_LIMIT)

    }

    fn set_value(&mut self, value: &'a T) -> bool
        where T: Actor {

            match self.val {

                Some(_val) => false,

                None => {

                    self.val = Some(value);

                    true

                },

            }

    }

    //Partition only if value is set
    fn partition(&mut self) -> bool {

        if self.is_leaf_node() {

            //Partition failed -- we are at leaf node therefore a collision
            return false

        }

        let x_mid: f32 = (self.xrange.1 - self.xrange.0) / 2.0 + self.xrange.0;
        let y_mid: f32 = (self.yrange.1 - self.yrange.0) / 2.0 + self.yrange.0;

        self.tl = Some(QuadNode::new((self.xrange.0, x_mid), (self.yrange.0, y_mid)));
        self.tr = Some(QuadNode::new((x_mid, self.xrange.1), (self.yrange.0, y_mid)));
        self.bl = Some(QuadNode::new((self.xrange.0, x_mid), (y_mid, self.yrange.1)));
        self.br = Some(QuadNode::new((x_mid, self.xrange.1), (y_mid, self.yrange.1)));

        //Partition success
        true

    }

}

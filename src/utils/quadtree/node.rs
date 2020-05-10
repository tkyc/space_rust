//TODO



pub struct QuadNode<'a, 'b, T> {
    pub tl: Option<&'a QuadNode<'a, 'b, T>>,
    pub tr: Option<&'a QuadNode<'a, 'b, T>>,
    pub bl: Option<&'a QuadNode<'a, 'b, T>>,
    pub br: Option<&'a QuadNode<'a, 'b, T>>,
    val: Option<&'b T>,
}



impl<'a, 'b, T> QuadNode<'a, 'b, T> {

}

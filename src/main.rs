//use rssudoku::Config;


fn main() {
    //let cfg = Config::default().finish();

    let mut b0 = rssudoku::Board::new();

    b0.solve(0);
    b0.print();
    println!("b0 {:?}",b0.traceback);


   let mut b1 = rssudoku::Board::new();

    b1.s(0,0,1);
    b1.s(1,1,2);
    b1.s(2,2,3);
    b1.s(3,3,4);
    b1.s(4,4,5);
    b1.s(5,5,6);
    b1.solve(0);
    b1.print();
    println!("b1 {:?}",b1.traceback);

  
    
    

    
 
}

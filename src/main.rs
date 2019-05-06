//use rssudoku::Config;


use std::fs;
use std::io;

fn main_solve<T>(f:T) ->Option<()> where T: io::Read {

    let mut b0 = rssudoku::Board::new();
    b0.read_from(f);
     println!("start board");
    b0.print();
    println!("solving...");
    b0.solve(0);
    println!("b0 {:?}",b0.traceback);
    b0.print();
    println!("b0 {:?}",b0.traceback);
    Some(())
} 


fn main() {
    //let cfg = Config::default().finish();
    let args:Vec<String> = std::env::args().collect();
    let args_len = args.len();
 
    if args_len>1 {
     for filename in &args[1..] {
        let f =  (fs::File::open(filename)).unwrap();
        main_solve(f);
    }
    }
    else {
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



     
    
    

    
 
}

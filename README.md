# rs-sudoku

simple brute force resolver first commit ....
it works....
but forn now you have to code the board:
    
    let mut b1 = rssudoku::Board::new();

    b1.s(0,0,1);
    b1.s(1,1,2);
    b1.s(2,2,3);
    b1.s(3,3,4);
    b1.s(4,4,5);
    b1.s(5,5,6);
    b1.Solve(0);
    b1.Print();
    println!("b1 {:?}",b1.traceback);

roadmap:
* input from file
* board generator
* benchmarks

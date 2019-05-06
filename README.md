# rs-sudoku

simple brute force resolver 
it works....
## quick start
cargo build
rssudoku board.txt

bardt.txt formats
packed 1->9 rows , missinng rows   are zeroed:

    800000000
    003600000
    070090200
    050007000
    000045700
    000100030
    001000068
    008500010
    090000400
    
packed single rows of 81 elements 0 for empty

    800000000003600000070090200050007000000045700000100030001000068008500010090000400
pretty printed: with # comments:

    # the father of all sudoku
    8 0 0 | 0 0 0 | 0 0 0
    0 0 3 | 6 0 0 | 0 0 0
    0 7 0 | 0 9 0 | 2 0 0
    ------+-------+------
    0 5 0 | 0 0 7 | 0 0 0
    0 0 0 | 0 4 5 | 7 0 0
    0 0 0 | 1 0 0 | 0 3 0
    ------+-------+------
    0 0 1 | 0 0 0 | 0 6 8
    0 0 8 | 5 0 0 | 0 1 0
    0 9 0 | 0 0 0 | 4 0 0



if you want create programmatically a board :
    
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
* ~~input from file~~ DONE
    - ~~text file 9 char 0-9 9 rows , #comment~~ DONE
    - ~~text file 21 chars as 1 2 3 | 4 5 6 | 7 8 9 ~~ DONE
    - ~~ prtty print ~~ DONE
* board generator
* benchmarks
* optimization ( ~~parallel? let's test~~ NAH)
* web ui react


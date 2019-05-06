#[macro_use]
extern crate criterion;

use criterion::Criterion;
use criterion::black_box;

 

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sdk empty", |b| b.iter(|| {  
        let mut b= rssudoku::Board::new(); 
        b.solve(0);  
        } ));
  c.bench_function("sdk diag", |b| b.iter(|| {  
        let mut b= rssudoku::Board::new(); 
        b.s(0,0,1);b.s(1,1,2);b.s(2,2,3);
        b.s(3,3,4);b.s(4,4,5);b.s(6,6,6);

        

        b.solve(0);  
        } ));

 c.bench_function("sdk everest", |b| b.iter(|| {  
        let mut b= rssudoku::Board::new(); 
       b.read_from("800000000003600000070090200050007000000045700000100030001000068008500010090000400".as_bytes());
        

        b.solve(0);  
        } ));


        
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
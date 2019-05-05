
use std::fmt;


#[derive(Debug)]
pub struct Config {  size: usize  }
impl Config { 
    pub fn default() -> Self
    {
      let res= Config { size: 9 };
       res
    }
    pub fn finish(self) -> Self { self } 
    }




//Assumption used to track assumption  un possible sulutions
#[derive(Debug,Clone,PartialEq)]
pub struct Assumption {
	I:usize,
	J:usize,
	V:u8,
}

pub struct Board { 
  cels : Vec<u8>,
  pub traceback: usize
}

impl Board {
    pub fn new() -> Self {  Board { cels: vec!(0;81) , traceback:0 }  }

    pub fn g(&self , r:usize ,c: usize) -> u8 {

        self.cels[c+r*9]

    }

    pub fn s(&mut self , r:usize ,c: usize,v: u8) {

        self.cels[c+r*9]=v;

    }
    

    //Print in a pretty format the currend board
    pub fn   Print(&self)  {

	for i in 0..9 {

		println!("{} {} {} | {} {} {} | {} {} {}",self.g(i,0), self.g(i,1), self.g(i,2),
                    self.g(i,3), self.g(i,4), self.g(i,5), 
                    self.g(i,6), self.g(i,7), self.g(i,8));
		if i == 2 || i == 5 {
			println!("------+-------+------")
		}
	}
   }


   pub fn is_valid(&self) -> (bool,bool){

let mut zeros=false;


   for r in 0..9 {
      let  mut seen = vec![ false;10];
       for i in 0..9 {
           let e= self.g(r,i) as usize;

           if e>0 {
               if seen[e] { return (false,zeros); }
               seen[e]=true;
           }
           else { zeros=true;}
       }       
   };
   for c in 0..9 {
      let  mut seen = vec![ false;10];
       for i in 0..9 {
           let e= self.g(i,c) as usize;
           if e>0 {
               if seen[e] { return (false,zeros); }
               seen[e]=true;
           } else
           { zeros=true;}
       }       
   };
   for s in 0..9 {
       let i0= (s / 3)*3;
       let j0= (s % 3)*3;
      let  mut seen = vec![ false;10];
      
       for i in i0..(i0+3) {
       for j in j0..(j0+3) {
           
           let e= self.g(i,j) as usize;
           if e>0 {
               if seen[e] { return (false,zeros); }
               seen[e]=true;
           }
           else { zeros=true;}
       }       
       }
   };


    (true,zeros)
   }





pub fn get_alternatives(&self,r:usize,c:usize) -> Vec<u8> {

   let  mut seen = vec![ false;10];
   let  mut res = vec![];
   //println!("get_alternatives ({},{})",r,c);
   let elem= self.g(r,c);
   if elem >0 {
       //res.push(elem);
       return res;
   }
   


   
       for i in 0..9 {

           if i!=c { 
           let e= self.g(r,i) as usize;
           if e>0  {
               if !seen[e] { seen[e]=true; }
           }
           }
       };       
    
    //println!("seen row: {:?}",seen );

    for i in 0..9 {
            if i!=r {
           let e= self.g(i,c) as usize;
           if e>0 {
               if !seen[e] { seen[e]=true; }
           }
            }
   };

//println!("seen cols: {:?}",seen );
    

       let i0= (r / 3)*3;
       let j0= (c / 3)*3;
   
       for i in i0..(i0+3) {
       for j in j0..(j0+3) {

           if i!=r && j!=c {

        //println!("sect: {},{} -> {},{}",i0,j0,i,j);
           
           let e= self.g(i,j) as usize;
           if e>0 {
               if !seen[e] { seen[e]=true; }
           }
       } 
       }      
       
   };

//println!("seen section: {:?}",seen );


     for i in 1..10 {
         if !seen[i] {
             res.push(i as u8);
         }
     }
    
    res
   }
 

 //MakeAssumptions make the assumptions on the possible solution
    pub fn   make_assumptions(&mut self,ix: usize, jx: usize, t: u8) ->  Vec<Assumption> {
    let mut ass =vec![];
	
    self.s(ix,jx, t);

	ass.push(Assumption{ I:ix, J:jx, V:t});

	for i  in 0..9   {
		for j in 0..9  {
			if !(ix == i && jx == j) {
				
                //let mut cl= |ik,jk| {
					let con =  self.get_alternatives(i, j);
					if  con.len() == 1 {
						self.s(i,j,con[0]);
						ass.push( Assumption{I: i, J:j, V:con[0]});
					};

				//};
                
                //cl(i, j);
			}
		}
	}

	ass
}


//UndoAssumptions revert [wrong] assumptions
pub fn  undo_assumptions(&mut self , ass: Vec<Assumption>) {


    self.traceback+=1;
    //println!("undo_assumptions {:?}",ass);

	//self.Traceback++
	for  a in ass {
		self.s(a.I,a.J, 0)

	}
	if self.traceback%1000 == 0 {

		println!("Tracebacks {}", self.traceback);
		self.Print()
	}
}

pub fn find_next_empty_cell(&self) -> Option<(usize,usize)>{

for i in 0..9 {
		for j in 0.. 9 {
			if self.g(i,j) == 0 {
				return Option::Some((i, j));

			}
		}
	}

Option::None
}

//Solve the board
pub fn  Solve(&mut self, d:usize) -> bool {

	//fmt.Printf("depth %d \n", d)
	//b.Print()
	loop {
	
        let next = self.find_next_empty_cell();

        //println!("next: {:?}",next);

        match next {
         Option::None  => {
			return false ;
		}
        Some(c)  =>  {
            let (i,j) =c;
          let con = self.get_alternatives(i, j);


		 //println!("constrains  {},{} {:?}", i, j, con);

		for  t in con {
			let ass = self.make_assumptions(i, j, t);

           //println!("assumptions  for {} -> {:?}", t, ass);

			let (val, zeros) = self.is_valid();

			if val {

                //println!("\n{:?}", self);

               

				if !zeros || self.Solve(d+1) {
					return true
				}
			}

			self.undo_assumptions(ass)

		}

        }
        }
		//fmt.Printf("traceback (%d,%d)\n", i, j)

		return false

	}
}
}





impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Board:\n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n  -  -  -++-  -  -++-  -  -  \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n  -  -  -++-  -  -++-  -  - \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n ", 
        &self.cels[0..3]  ,&self.cels[3..6],&self.cels[6..9], 
        &self.cels[9..12] ,&self.cels[12..15],&self.cels[15..18], 
        &self.cels[18..21],&self.cels[21..24],&self.cels[24..27], 
        &self.cels[27..30],&self.cels[30..33],&self.cels[33..36], 
        &self.cels[36..39],&self.cels[39..42],&self.cels[42..45], 
        &self.cels[45..48],&self.cels[48..51],&self.cels[51..54], 
        &self.cels[54..57],&self.cels[57..60],&self.cels[60..63], 
        &self.cels[63..66],&self.cels[66..69],&self.cels[69..72], 
        &self.cels[72..75],&self.cels[75..78],&self.cels[78..81]) 
           }
}

#[test]
fn  test_isvalid(){
 
    let mut b = Board::new();
    //empty
    b.Print();
    assert_eq!(b.is_valid(),(true,true));

    //single
    b.s(0,0,1);
    b.Print();   
    assert_eq!(b.is_valid(),(true,true));

    b.s(1,1,2);
    b.Print();
   
    assert_eq!(b.is_valid(),(true,true));
    b.s(0,3,3);
    b.Print();   
    assert_eq!(b.is_valid(),(true,true));

    //row violation
    b.s(1,3,2);
    b.Print(); 
    assert_eq!(b.is_valid(),(false,true));

    //section violation
    b.s(1,3,4);
    b.s(1,4,4);
    b.Print();   
    assert_eq!(b.is_valid(),(false,true));

    //col violation
    b.s(1,3,4);
    b.s(1,4,5);
    b.s(2,3,4);    
    b.Print();
    assert_eq!(b.is_valid(),(false,true));
   

}

#[test]
fn test_isvalid_block(){


    let mut b = Board::new();

    // r:0 -> [1, 2, 3][4, 5, 6][7, 8, 9]  
    for c in 0..9 {
        b.s(0,c,(c+1) as u8);
    }
    //empty
    b.Print();
    println!("{:?}",b );

    assert_eq!(b.is_valid(),(true,true));

    //r:1 -> [4, 0, 0][0, 0, 0][0, 0, 0]
    b.s(1,0,4);
    b.Print();
    println!("{:?}",b );
    assert_eq!(b.is_valid(),(true,true));

   let alt20 = b.get_alternatives(2, 0);
   assert_eq!(alt20,vec![5, 6, 7, 8, 9] );


   let alt80 = b.get_alternatives(8, 0);
    println!("{:?}",alt80);
 
   assert_eq!(alt80,vec![2, 3, 5, 6, 7, 8, 9]);

   
    let alt81 = b.get_alternatives(8, 1);
    println!("{:?}",alt81);
 
   assert_eq!(alt81,vec![1, 3, 4, 5, 6, 7, 8, 9]);

    let alt11 = b.get_alternatives(1, 1);
    println!("{:?}",alt11);
 
   assert_eq!(alt11,vec![5, 6, 7, 8, 9]);


   // assert_eq!(b.is_valid(),(true,false));
  
}
#[test]
fn  test_alternatives(){
 
    let mut b = Board::new();
    //empty
    b.Print();
    assert_eq!(b.get_alternatives(0, 0),vec![1,2,3,4,5,6,7,8,9]);
    
    b.s(0,1,1);
    b.s(0,0,2);
   
    b.Print();   
    assert_eq!(b.get_alternatives(0, 0),vec![]);

    assert_eq!(b.get_alternatives(0, 2),vec![3,4,5,6,7,8,9]);
    b.s(0,2,3);
   
    assert_eq!(b.get_alternatives(0, 3),vec![4,5,6,7,8,9]);

    b.Print();
    
    assert_eq!(b.get_alternatives(8, 8),vec![1,2,3,4,5,6,7,8,9]);
 

}
#[test]
fn  test_make_assumptions(){
 
    let mut b = Board::new();
    let a0 = b.make_assumptions(0, 0, 1);
    println!("0,0 -> 1  ASS {:?}",a0);
    println!("{:?}",b);
    assert_eq!(a0,vec![ Assumption{ I:0,J:0,V:1};1]);


    let a1 = b.make_assumptions(0, 1, 2);
    println!("0,1 -> 2  ASS {:?}",a1);
    println!("{:?}",b);
    assert_eq!(a1,vec![ Assumption{ I:0,J:1,V:2}]);

    let a2 = b.make_assumptions(0, 2, 3);
    println!("0,2 -> 3  ASS {:?}",a2);
    println!("{:?}",b);
    assert_eq!(a2,vec![ Assumption{ I:0,J:2,V:3}]);

}


//#[test]
fn  test_solve(){

    let mut b = Board::new();
    let a0 = b.Solve(0);

    println!("empty  ASS {:?}",a0);
    println!("{:?}",b);
    assert_eq!(a0,false);

 
}
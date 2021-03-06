use std::fmt;
use std::io;
use std::io::prelude::*;
 


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




//assumption used to track Assumption  un possible sulutions
#[derive(Debug,Clone,PartialEq)]
pub struct Assumption {
	i:usize,
	j:usize,
	v:u8,
}

pub struct Board { 
  cels : Vec<u8>,
  pub traceback: usize,
  pub trace_assumptions: usize,
  pub trace_main_assumptions: usize,
  
}

impl Board {
    pub fn new() -> Self {  Board { cels: vec!(0;81) , traceback:0 ,trace_assumptions:0,trace_main_assumptions:0 }  }


    
    pub fn read_from<T,>(&mut self,  input_reader : T) -> Result<(), io::Error>  where  T: io::Read,
    {
    let file = io::BufReader::new(input_reader);
    let mut i =0_usize;
     
    

    for line in file.lines() {
        let l = line.unwrap();
        if l.starts_with("#") { break;}
        if l.len()==81 {
           
             for  (p,v) in l.chars().enumerate() { self.s(p / 9 ,p % 9, v as u8 - ('0' as u8)); }
             i+=9;
        } 
        if l.len()==9 {
            for  (j,v) in l.chars().enumerate() { self.s(i,j, v as u8 - ('0' as u8)); }
         i+=1;
        }
        if l.len()==21 {
            let ll=l.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let ll_len =ll.len();

          
       

            for  (j,v) in ll.into_iter().enumerate() {

                self.s(i,j, v as u8 - ('0' as u8));

            }
            
           if ll_len==9 { i+=1; }
        }
        
         
    }       
    // yadda yadda...
    Ok(())
 }

    #[inline]
    pub fn g(&self , r:usize ,c: usize) -> u8 {

        self.cels[c+r*9]

    }

    #[inline]
    pub fn s(&mut self , r:usize ,c: usize,v: u8) {

        self.cels[c+r*9]=v;

    }
    
    
    

    //Print in a pretty format the currend board
    pub fn   print(&self)  {
    println!("# {} {} {}",self.traceback, self.trace_assumptions, self.trace_main_assumptions);

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




///get alternative values for a given position
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
 

///make_assumptions make the Assumptions on the possible solutions with some look forward 
/// as when for a cell in the board there is only one alternative, and cheking the "touched" positions in the board   
pub fn   make_assumptions(&mut self,ix: usize, jx: usize, t: u8) ->  Vec<Assumption> {
    let mut ass =vec![];
	
    self.s(ix,jx, t);

	ass.push(Assumption{ i:ix, j:jx, v:t});
    self.trace_assumptions+=1;
    self.trace_main_assumptions+=1;

    let mut touched=vec![false;81];
    let mut retouch=vec![false;81];
    let mut found =1;



     /* for i  in 0..9   {
		for j in 0..9  {
			if !(ix == i && jx == j) {
                 touched[j+i*9]=1;
            }
        }
      }*/   
    

      let i0= (ix / 3)*3;
      let j0= (jx / 3)*3;
   
       for z in 0..9 {   
                                   if z!=ix &&self.g(z,jx)==0 {  touched[jx+z*9]=true ;}
                                   if z!=jx &&self.g(ix,z)==0 { touched[z+ix*9]=true ;}     
                                   if !((i0+z/3)==ix && (j0+z%3)==jx)  &&self.g(i0+z/3,0+z%3)==0   { touched[j0+z%3+(i0+z/3)*9]=true;  }
        }

    loop {

     for (t,v) in touched.into_iter().enumerate() {
         if !v { continue;}
         let (i,j)=(t/9,t%9);
    			let con =  self.get_alternatives(i, j);
					if  con.len() == 1 {
						self.s(i,j,con[0]);
						ass.push( Assumption{i, j, v:con[0]});
                        //retouched.push((i,j));
                        let i0= (i / 3)*3;
                        let j0= (j / 3)*3;
   
                        for z in 0..9 {   
                                   if z!=i &&self.g(z,j)==0 { retouch[j+z*9]=true ;}
                                   if z!=j &&self.g(i,z)==0  { retouch[z+i*9]=true ;}     
                                   if !( (i0+z/3)==i && (j0+z%3)==j)  &&self.g(i0+z/3,0+z%3)==0  { retouch[j0+z%3+(i0+z/3)*9]=true;  }
                                }

                        self.trace_assumptions+=1;
					};
 
            }         

           if ass.len()<= found { return ass;  }

           found= ass.len();

           touched=retouch;
           retouch=vec![false;81];


    }
   

    
	ass
}
 


//UndoAssumptions revert [wrong] Assumptions
pub fn  undo_assumptions(&mut self , ass: Vec<Assumption>) {
    //println!("undo_assumptions {:?}",ass);

    self.traceback+=1;

	for  a in ass {
		self.s(a.i,a.j, 0)

	}
	//if self.traceback%1000 == 0 {

	//	println!("Tracebacks {}", self.traceback);
	//	self.print()
	//}
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

///solve the board
/// 
/// #Example
/// 
///     let mut b=rssudoku::Board::new();
///     b.solve(0);
///     b.print();
/// 
/// 
pub fn  solve(&mut self, d:usize) -> bool {

	//fmt.Printf("depth %d \n", d)
	//b.Print()
	loop {
	
        let next = self.find_next_empty_cell();

        //println!("next: {:?}",next);

        match next {
         Option::None     => { return false ; 	}
         Option::Some(c)  => {
            let (i,j) =c;
            let con = self.get_alternatives(i, j);
    		 //println!("constrains  {},{} {:?}", i, j, con);
    		for  t in con {
	    		let ass = self.make_assumptions(i, j, t);
               //println!("assumptions  for {} -> {:?}", t, ass);
    			let (val, zeros) = self.is_valid();
    			if val {
                    //println!("\n{:?}", self);
    				if !zeros || self.solve(d+1) {
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

        write!(f, "Board: {} {} {}\n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n  -  -  -++-  -  -++-  -  -  \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n  -  -  -++-  -  -++-  -  - \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n {:?}{:?}{:?} \n ", 
        &self.traceback, self.trace_assumptions, self.trace_main_assumptions,
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
    b.print();
    assert_eq!(b.is_valid(),(true,true));

    //single
    b.s(0,0,1);
    b.print();   
    assert_eq!(b.is_valid(),(true,true));

    b.s(1,1,2);
    b.print();
   
    assert_eq!(b.is_valid(),(true,true));
    b.s(0,3,3);
    b.print();   
    assert_eq!(b.is_valid(),(true,true));

    //row violation
    b.s(1,3,2);
    b.print(); 
    assert_eq!(b.is_valid(),(false,true));

    //section violation
    b.s(1,3,4);
    b.s(1,4,4);
    b.print();   
    assert_eq!(b.is_valid(),(false,true));

    //col violation
    b.s(1,3,4);
    b.s(1,4,5);
    b.s(2,3,4);    
    b.print();
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
    b.print();
    println!("{:?}",b );

    assert_eq!(b.is_valid(),(true,true));

    //r:1 -> [4, 0, 0][0, 0, 0][0, 0, 0]
    b.s(1,0,4);
    b.print();
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
    b.print();
    assert_eq!(b.get_alternatives(0, 0),vec![1,2,3,4,5,6,7,8,9]);
    
    b.s(0,1,1);
    b.s(0,0,2);   
    b.print();   
    assert_eq!(b.get_alternatives(0, 0),vec![]);

    assert_eq!(b.get_alternatives(0, 2),vec![3,4,5,6,7,8,9]);
    b.s(0,2,3);   
    assert_eq!(b.get_alternatives(0, 3),vec![4,5,6,7,8,9]);

    b.print();    
    assert_eq!(b.get_alternatives(8, 8),vec![1,2,3,4,5,6,7,8,9]);
 

}
#[test]
fn  test_make_assumptions(){
 
    let mut b = Board::new();
    let a0 = b.make_assumptions(0, 0, 1);
    println!("0,0 -> 1  ASS {:?}",a0);
    println!("{:?}",b);
    assert_eq!(a0,vec![ Assumption{ i:0,j:0,v:1};1]);


    let a1 = b.make_assumptions(0, 1, 2);
    println!("0,1 -> 2  ASS {:?}",a1);
    println!("{:?}",b);
    assert_eq!(a1,vec![ Assumption{ i:0,j:1,v:2}]);

    let a2 = b.make_assumptions(0, 2, 3);
    println!("0,2 -> 3  ASS {:?}",a2);
    println!("{:?}",b);
    assert_eq!(a2,vec![ Assumption{ i:0,j:2,v:3}]);

}


#[test]
fn  test_solve(){

    let mut b = Board::new();
    let a0 = b.solve(0);
    println!("empty  ASS {:?}",a0);
    println!("{:?}",b);
    assert_eq!(a0,true);

 
}
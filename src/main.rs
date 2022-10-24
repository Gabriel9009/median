fn main() {
 let mut vec = vec![0, 1];
 let arr = [1, 2, 5 , 3, 7];
 vec.extend(&arr);
 vec.sort();
 println!("{:?}", vec);
 
    if vec.len() % 2 == 1{
        let num: usize = vec.len() / 2;
         let save :usize = vec[num];
          println!("{:?}", save);
        }else{
            let new: usize = vec.len() / 2;
            let first :usize = vec[new];
            let second :usize = vec[new - 1];
            let median = (first + second) / 2;
            println!("{:?}", median);
        }
    }
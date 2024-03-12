use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

#[macro_use]
extern crate timeit;

use rb_tree::rbtree_mod::*;

fn main() {
    
    let i = timeit_loops!(1, {
        let mut tree = RedBlackTree::new();
        'l: loop {
            let mut vec = vec![];
            for i in 0..5000 {
                tree.add(i);
                vec.push(i)
            }
            // println!("{:?}", vec);
            let old_vec = vec.clone();

            let mut rng = thread_rng();
            vec.shuffle(&mut rng);
    
            
            let mut count = tree.len();
            for i in vec.clone() {
                tree.del(i as i32);
                if tree.len() == count /*|| tree.check_tree() == false*/ {
                    println!("{:?} ", old_vec);
                    println!("{:?} ", vec);
                    println!("---------{:#?} ", i);
                    break 'l;
                } else {
                    count = tree.len();
                }
            }
        }
        
  
        
        /*
        let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49]  ;
        for i in vec.clone() {
            tree.insert(i);
        }

        let vec = vec![44, 34, 45, 37, 7, 30, 17, 14, 48, 8, 12, 36, 0, 4, 40, 32, 33, 10, 15, 5, 20, 47, 9, 31, 18, 2, 11, 6, 13, 1, 16, 3, 41, 28, 19, 46, 24, 23, 49, 35, 21, 27, 29, 38, 26, 39, 22, 25, 42, 43] ;

        // println!("{:#?}", tree);
        let mut count = tree.len();
        for i in vec.clone() {
            
            tree.del(i as i32);
            /*
            if i == 13 {
                println!("{:#?}", tree);
            }
             */

            if tree.len() == count || tree.check_tree() == false {
                println!("{:?} ---- {:#?}", i, tree);
            } else {
                count = tree.len();
            }
            

            println!("{:#?} {:#?}", tree.len(), tree.check_tree());
        }
         */
    });

    println!("time: {:?}", i);
    
}

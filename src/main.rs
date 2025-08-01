fn main() {
    let x: i32 = 1;
    print!("x is : {} ", x);
    let is_male=true;
    print!("{} ",is_male); 
    let greeting: String =String::from("hello world");
    println!("{} ", greeting);

    let a = true;
    if a {
        print!("ok");
    } else {
        print!("not ok");
    }

    for i in 0..100 {
        print!("{} ", i);
    }
   
   let f = 10;
   let g = 20;
   let sum = do_sum(f, g);
   print!("sum is : {} ", sum);


    fn do_sum(f: i32, g: i32) -> i32 {
        return f + g;
    }
    
}
 
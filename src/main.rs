#[cfg(test)]
mod tests {
    use super::cc::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn include(){
        say_hello();
    
        let   template:&str =  include_str!("../templates/index.html");
        println!("{}",template);
        let   template:&str =  include_str!("../templates/index.html");
        println!("{}",template);
        assert_eq!(2 + 2, 4);
    }
} 


pub mod cc{
  pub  fn say_hello(){
        println!("hello");
    }
}


fn main (){
    let   template:&str =  include_str!("../templates/index.html");
    println!("{}",template);
    let   template:&str =  include_str!("../templates/index.html");
}
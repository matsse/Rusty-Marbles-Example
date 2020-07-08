use rand::Rng;
use std::borrow::BorrowMut;


fn Initialize() ->  u8 {
   let mut rng = rand::thread_rng();
   let num  = rng.gen_range(32, 125);
   num as u8
}



pub fn scamble_w_char_key<'a>(mut wcToScramble: &mut String, mut key : &mut String) -> (String, String) {

   // let wc_key = Initialize();
   let hexes = wcToScramble.as_bytes();
   let mut output : Vec<u8> = Vec::new();
   let mut wc_key : Vec<u8> = Vec::new();
   for input in hexes.iter() {

      wc_key.push(Initialize());
      let mut x = input + *wc_key.last().unwrap();
      if x > 126 {
         let xtemp = x % 126;
         x =  xtemp;
         output.push(x);
      } else {
         output.push(x);
      }
      //println!("{:?} {:?}\n", wc_key, wc_key.last().unwrap());


   }
   //*key = wc_key;
   *key = String::from_utf8(wc_key).unwrap();
   *wcToScramble = String::from_utf8(output).unwrap();
   (wcToScramble.clone(), key.clone())

}


pub fn scamble_wkey<'a>(mut wcToScramble: &mut String, mut key : &mut u8) -> String {

   // let wc_key = Initialize();
   let hexes = wcToScramble.as_bytes();
   let mut output : Vec<u8> = Vec::new();
   let mut wc_key : Vec<u8> = Vec::new();
   for input in hexes.iter() {

      wc_key.push(Initialize());
      let mut x = input + *key;
      if x > 126 {
         let xtemp = x % 126;
         x =  xtemp;
         output.push(x);
         //println!("[>126]Input:{} XOR Key:{} = {}",input, *key,  x);
      } else {
         output.push(x);
         //println!("[<126]Input:{} XOR Key:{} = {}",input, *key,  x);
      }
      //*wcToScramble.insert( as usize, x);
      //println!("{:?} {:?}\n", wc_key, wc_key.last().unwrap());


   }
   //*key = wc_key;
   *wcToScramble = String::from_utf8(output).unwrap();
   wcToScramble.clone()



}

pub fn Scramble<'a>(mut wcToScramble: &mut String, mut key : &mut u8)  -> String {

   let wc_key = Initialize();
   let hexes = wcToScramble.as_bytes();
   let mut output : Vec<u8> = Vec::new();

   for input in hexes.iter() {


      let mut x = input + wc_key;
      if x > 126 {
         let xtemp = x % 126;
         x =  xtemp;
         output.push(x);
         println!("[>126]Input:{} XOR Key:{} = {}",input, wc_key,  x);
      } else {
         output.push(x);
         println!("[<126]Input:{} XOR Key:{} = {}",input, wc_key,  x);
      }
      //*wcToScramble.insert( as usize, x);



   }
   *key = wc_key;
   *wcToScramble = String::from_utf8(output).unwrap();
    wcToScramble.clone()






}

pub fn GenerateInsert(mut wcUnScramble: &mut String, key: &mut u8 ) {


      let sImport = "Rusty-Marbles = {path = \"../Rusty_Marbles\"}";

      let sInsert = format!("let mut {} = \
         RMBL_CLASS_BUMP1D::uscrambled(String::from({:?}).borrow_mut(), {} )", "TestString", wcUnScramble.clone(), key.clone());

      println!("{}", sInsert)
}


pub fn Generate_WCInsert(mut wcUnScramble: &mut String, key: &mut String ) {


   let sImport = "Rusty-Marbles = {path = \"../Rusty_Marbles\"}";

   let sInsert = format!("let mut {} = \
         RMBL_CLASS_BUMP1D::uscramble_Wckey(String::from({:?}).borrow_mut(), {}.borrow_mut());", "TestString", wcUnScramble.clone(), key.clone());

   println!("{}", sInsert)
}
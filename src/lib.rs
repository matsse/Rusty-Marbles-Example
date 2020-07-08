mod RMBL_CLASS_BUMP1;
mod RMBL_CLASS_BUMP1D;




#[cfg(test)]
mod tests {
    use crate::I_scramble_Q;
    use std::borrow::{BorrowMut, Borrow};
    use crate::RMBL_CLASS_BUMP1D::uscramble;
    use crate::RMBL_CLASS_BUMP1::Scramble;
    use crate::RMBL_CLASS_BUMP1::scamble_wkey;
    use crate::RMBL_CLASS_BUMP1::scamble_w_char_key;
    use crate::RMBL_CLASS_BUMP1D::uscramble_Wckey;
    use crate::RMBL_CLASS_BUMP1::Generate_WCInsert;

    #[test]
    fn AssignSingle() {
        let x = I_scramble_Q::new();
    }
    #[test]
    fn assign_Hash_Map<'a>() {
        let mut  x = I_scramble_Q::new();
        let mut  string  = String::from("abcdefghijklmnopqrstuvwxyz");
        let mut xy  = (0 as u8);
        //let xxx = ((x.scramble_a)(string.borrow_mut(), xy) as String);
        println!("{:?} {:?} {:?}", string.len(), ((x.scramble_a)(string.borrow_mut(), xy.borrow_mut()) as String), string.len());
        println!("{:?}", string.len());

        (x.generate_insert_a)(string.borrow_mut(), xy.borrow_mut());
        println!("{:?}", (uscramble(string.borrow_mut(), xy) as String));
    }
    #[test]
    fn assign_New_Literal<'a>() {
        let mut  x = I_scramble_Q::new();
        let mut  string  = String::from("This is a test string");
        let mut  string2  = String::from("This is a test string");
        for i in 32..126 {
            if scamble_wkey(string.borrow_mut(), (i as u8).borrow_mut()).len() != string2.len() {
                //panic!("ERROR");
            } else if string2 == string {
                //panic!("Same String");
            } else {
                println!("{:?} - {:?}", i, string)
            }
        }
    }
    #[test]
    fn with_Char<'a>() {
        //let mut  x = I_scramble_Q::new();
        let mut wc_key = String::from("");
        let mut  string  = String::from("Every normal man must be tempted at times. To spit upon his hands. Hoist the black flag, and begin slitting throats");
        let mut Obfusc = scamble_w_char_key(string.borrow_mut(), wc_key.borrow_mut());

        println!("Obfuscated Output: [{:?}]   \nKey: [{}]", Obfusc.0, Obfusc.1);

        let  DeObfusc = uscramble_Wckey(Obfusc.0.borrow_mut() as &mut String, Obfusc.1.borrow_mut() as &mut String);
        println!("Deobfuscated Output: [{}] ", DeObfusc);
        Generate_WCInsert(Obfusc.0.borrow_mut() as &mut String, Obfusc.1.borrow_mut() as &mut String);

    }


    #[test]
    fn with_char_write<'a>() {
        //let mut  x = I_scramble_Q::new();
        let mut wc_key = String::from("");
        let mut  string  = String::from("Every normal man must be tempted at times. To spit upon his hands. Hoist the black flag, and begin slitting throats");

        let mut Obfusc = scamble_w_char_key(string.borrow_mut(), wc_key.borrow_mut());
        println!("Obfuscated Output: [{:?}]   \nKey: [{}]", Obfusc.0, Obfusc.1);

        let  DeObfusc = uscramble_Wckey(Obfusc.0.borrow_mut() as &mut String, Obfusc.1.borrow_mut() as &mut String);
        println!("Deobfuscated Output: [{}] ", DeObfusc);

    }

}


pub struct I_scramble_Q<'a> {
    pub scramble_a: Box<dyn FnMut(&mut String, &mut u8) -> String + 'a>,
    pub generate_insert_a: Box<dyn FnMut( &mut String, &mut u8) + 'a>,
}



impl <'a> I_scramble_Q <'a>{
    pub fn new() -> I_scramble_Q<'a> {
        I_scramble_Q{
            scramble_a: Box::new(RMBL_CLASS_BUMP1::Scramble),
            generate_insert_a: Box::new(RMBL_CLASS_BUMP1::GenerateInsert),
        }
    }

}














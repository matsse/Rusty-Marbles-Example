
pub fn uscramble(mut wcUnScramble: &mut String, key: u8 ) -> String{
    //let hexes = wcUnScramble.as_bytes();
    let mut output : Vec<u8> = Vec::new();
    let mut ikey = key as i8;
    println!("{:?}", wcUnScramble.as_bytes());
    for input in wcUnScramble.as_bytes().iter() {
        let iInput : i8 = *input as i8;
        let mut x: i8 =  iInput - ikey ;

        if x < 0 {
            let mut base = -126;
            output.push(((x) - (base as i8 )) as u8);
        } else {
            output.push(x as u8)
        }
    }

    String::from_utf8(output).unwrap()
}

pub fn uscramble_Wckey(mut wcUnScramble: &mut String, key: &mut String ) -> String{
    //let hexes = wcUnScramble.as_bytes();
    let mut output : Vec<u8> = Vec::new();

    //println!("{:?}", wcUnScramble.as_bytes());
    let mut counter = 0;
    let keys = key.as_bytes();
    for input in wcUnScramble.as_bytes().iter() {

        let mut ikey = *((*keys).get(counter)).unwrap() as i8;
        let iInput : i8 = *input as i8;
        let mut x: i8 =  iInput - ikey ;

        if x < 0 {
            let mut base = -126;
            output.push(((x) - (base as i8 )) as u8);
        } else {
            output.push(x as u8)
        }
        counter+=1;
    }

    String::from_utf8(output).unwrap()
}



// pub fn uscramble(mut wcUnScramble: &mut String, key: u8 ) -> String{
//     //let hexes = wcUnScramble.as_bytes();
//     let mut output : Vec<u8> = Vec::new();
//     let mut ikey = key as i8;
//
//     for input in wcUnScramble.as_bytes().iter() {
//         let iInput : i8 = *input as i8;
//         let mut x: i8 =  iInput - ikey ;
//
//         if x < 0 {
//             let mut base = -128;
//             output.push(((x- 16) - (base as i8 )) as u8);
//         } else {
//             output.push(x as u8)
//         }
//     }
//     String::from_utf8(output).unwrap()
//
// }
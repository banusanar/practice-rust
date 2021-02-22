/*
FInd the min and max of 3 numbers
*/

use std::process::exit;
pub fn min_max(num1: i64, num2: i64, num3: i64) -> (i64, i64) {
    if num1 == 0 || num2 == 0 || num3 == 0 {
        exit(1);
    }

    let max = if num1 < num2 && num3 < num2 {
        num2
    } else if num2 < num1 && num3 < num1 {
        num1
    } else {
        num3
    };
    let min = if num1 > num2 && num3 > num2 {
        num2
    } else if num2 > num3 && num1 > num3 {
        num3
    } else {
        num1
    };
    (max, min)
    //println!("The Max # is [{}] and the Min # is [{}]", max, min);
}

#[test]
fn t_min_max() {
    let tp = (-1i64, -2i64, -3i64, -1i64, -3i64);
    let mmv = min_max(tp.0, tp.1, tp.2);
    assert_eq!(mmv.0, tp.3);
    assert_eq!(mmv.1, tp.4);

    //let tp1 = (0i64,1i64,2i64, 2i64, 0i64);
    //let mmva = min_max(tp1.0, tp1.1, tp1.2);
    //assert_eq!(mmva.0, tp1.3);
    //assert_eq!(mmva.1, tp1.4);
    /*
    /* 3 input variables, MAX, MIN in that order */
    let inputs : Vec<(i64, i64, i64, i64, i64)> = vec![
        (0i64,1i64,2i64, 2i64, 0i64), (10i64, 15i64, 10i64, 15i64, 10i64),
        (-1i64, -2i64, -3i64, -1i64, -3i64), (-1i64,0,1i64, 1i64, -1i64),
    ];

    for idx in 0 ..inputs.len() {
        let tp = inputs[idx];
        println!("{:?}", tp);
        let mmv = min_max(tp.0, tp.1, tp.2);
        assert_eq!(mmv.0, tp.3);
        assert_eq!(mmv.1, tp.4);
    }
    */
}

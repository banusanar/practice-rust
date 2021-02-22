extern crate expressions;
//use expressions::math::calc_expression;

#[test]
fn calculator_test() {
    assert_eq!(
        expressions::math::calc_expression::<i32>(10i32, 20i32, &"+"),
        30i32
    );
    assert_eq!(
        expressions::math::calc_expression::<i8>(127i8, 100i8, &"-"),
        27i8
    );
    assert_eq!(
        expressions::math::calc_expression::<i32>(10i32, 20i32, &"*"),
        200i32
    );
    assert_eq!(
        expressions::math::calc_expression::<f32>(127.0f32, 100.0f32, &"-"),
        27.0f32
    );
    assert_eq!(
        expressions::math::calc_expression::<i64>(10000i64, 20i64, &"/"),
        500i64
    );
    assert_eq!(
        expressions::math::calc_expression::<f32>(127.0f32, 100.0f32, &"%"),
        27.0f32
    );
}

#[test]
fn is_a_prime_test() {
    assert_eq!(expressions::math::is_a_prime(7i64), true);
    assert_eq!(expressions::math::is_a_prime(17i64), true);
    assert_eq!(expressions::math::is_a_prime(39i64), false);
    assert_eq!(expressions::math::is_a_prime(47i64), true);
    assert_eq!(expressions::math::is_a_prime(73i64), true);
}

#[test]
fn sum_of_digits_test() {
    assert_eq!(expressions::math::sum_of_digits(123u64), 6i8);
    assert_eq!(expressions::math::sum_of_digits(100001000u64), 2i8);
}

#[test]
fn gen_fibonacci_test() {
    assert_eq!(expressions::math::gen_fibonnaci(2usize), vec![0,1]);
    assert_eq!(expressions::math::gen_fibonnaci(3usize), vec![0,1,1]);
    assert_eq!(expressions::math::gen_fibonnaci(6usize), vec![0,1,1,2,3,5]);
}
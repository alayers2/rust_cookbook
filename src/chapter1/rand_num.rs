extern crate rand;
extern crate rand_distr;

use rand::Rng;
use rand::distributions::{Distribution, Uniform, Alphanumeric};
use rand_distr::{Normal, Standard};

/// Generates random numbers with help of random-number generator rand::Rng 
pub fn gen_random_nums() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

/// Generates random numbers within a half-open range
pub fn gen_random_nums_within_range() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0, 10));
    println!("Float: {}", rng.gen_range(0.0, 10.0));    
}

/// Generates random numbers within a half-open range and a uniform distribution
pub fn gen_uniform_random_nums_within_range() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    // Roll until we get a 6
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

/// Generate random numbers with a given distribution
pub fn gen_random_nums_within_range_and_distribution() {
    let mut rng = rand::thread_rng();
    let normal_dist = Normal::new(2.0, 3.0).unwrap();
    let v = normal_dist.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) =rng.gen();
        Point {
            x: rand_x,
            y: rand_y
        }
    }
}

/// Generate Random Values of a Custom Type
pub fn gen_random_values_of_custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random point: {:?}", rand_point);
}

/// Generate random passwords from a set of alphanumeric characters
pub fn gen_random_password_from_set_of_chars() {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_string);
}

/// Generate random passwords from a set of user-defined characters
pub fn gen_random_password_from_defined_chars() {
    const CHARSET: &[u8] = b"SUPERCOOLPASSWORDSET";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            char::from(unsafe { *CHARSET.get_unchecked(idx)})
        })
        .collect();

    println!("{:?}", password)
}
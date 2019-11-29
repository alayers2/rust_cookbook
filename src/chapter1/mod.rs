
pub mod rand_num;
pub mod vec_sort;

pub fn chapter_1_random_nums() {
    println!("Generate Random Numbers");
    rand_num::gen_random_nums();
    println!("");
    println!("Generate Random Numbers Within a Range");
    rand_num::gen_random_nums_within_range();
    println!("");
    println!("Generate Uniform Random Numbers Within a Range");
    rand_num::gen_uniform_random_nums_within_range();
    println!("");
    println!("Generate Random Numbers Within a Range and Distribution");
    rand_num::gen_random_nums_within_range_and_distribution();
    println!("");
    println!("Generate Random Numbers Of Custom Type");
    rand_num::gen_random_values_of_custom_type();
    println!("");
    println!("Generate Random Password");
    rand_num::gen_random_password_from_set_of_chars();
    println!("");
    println!("Generate Random Password from custom set");
    rand_num::gen_random_password_from_defined_chars();
    println!("");

}

pub fn chapter_1_vec_sort() {
    println!("Integer Vector Sort");
    vec_sort::int_vec_sort();
    println!("");
    println!("Float Vector Sort");
    vec_sort::float_vec_sort();
    println!("");
    println!("Sorting Structs (People)");
    vec_sort::sort_people();
    println!("");

}

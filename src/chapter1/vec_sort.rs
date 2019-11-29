

pub fn int_vec_sort() {
    let mut vec = vec![1, 5, 10, 2, 15];

    // Basic vector sort only works for ints.
    vec.sort();

    assert_eq!(vec, vec![1,2, 5, 10, 15]);
    println!("Original Vec {:?} -> Sorted Vec {:?}", vec![1, 5, 10, 2, 15], vec)
}

pub fn float_vec_sort() {
    let mut vec = vec![1.0, 5.0, 10.0, 2.5, 15.5];

    // This is cool, it uses a closure to implement a comparison.
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.0 ,2.5, 5.0, 10.0, 15.5]);
    println!("Original Vec {:?} -> Sorted Vec {:?}", vec![1, 5, 10, 2, 15], vec)

}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name:String, age: u32) -> Self {
        Person {
            name,
            age
        }
    }
}

pub fn sort_people() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    // Sort with derived sort impl
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25)
        ]
    );

    // Sort by age with custom comparator
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}
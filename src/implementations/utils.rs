use rand::Rng;

pub fn random_array<const SIZE : usize>(value_min : i32, value_max : i32) -> [i32; SIZE] {
    let mut rng = rand::thread_rng();
    let mut array = [0; SIZE];

    for index in 0..SIZE {
        array[index] = rng.gen_range(value_min..value_max);
    }
    array
}

pub fn dot_product<const COUNT: usize>(array0 : [i32; COUNT], array1 : [i32; COUNT]) -> i32 {
    array0.iter().zip(array1.iter()).map(|(x, y)| x * y).sum()
}
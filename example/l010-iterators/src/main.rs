fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // consuming the collection (into_iter())
    // for number in numbers.into_iter() { // at this time variable numbers already moved
    //     println!("Number: {}", number);
    // }

    // preserving the collection (iter())
    for &num in numbers.iter() {
        println!("Number: {}", num);
    }

    // modifying the collection (iter_mut())
    for number in numbers.iter_mut() {
        *number *= 2;
    }
    println!("modified numbers: {:?}", numbers);

    // iterator adapter:
    // doubling each number (map())
    let double_numbers = numbers.iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    println!("double numbers: {:?}", double_numbers);

    // filtering even numbers (filter())
    let even_numbers = numbers.iter().filter(|&x| x % 2 == 0).collect::<Vec<&i32>>();
    println!("even numbers: {:?}", even_numbers);

    // print each number (for_each())
    numbers.iter().for_each(|x| println!("numbers: {}", x));

    // sum the numbers
    let sum = numbers.iter().sum::<i32>(); // specify type for generic sum
    println!("Sum: {}",sum);

    // collecting into a vector (collect())
    let collected_numbers = numbers.iter().collect::<Vec<&i32>>();
    println!("collected numbers: {:?}", collected_numbers);

    // turbofish syntax
    let num2 = vec![10, 20, 30];
    // specify type for generic sum()
    let sum = num2.iter().sum::<i32>(); // ::<> look like a swimming fish with bobble behind
    println!("sum (i32): {}",sum);

    // drain all elements and collection them (drain())
    let mut num3 = vec![1, 2, 3, 4, 5];
    let removed_num3 = num3.drain(..).collect::<Vec<i32>>();
    println!("removed numbers: {:?}", removed_num3);
    println!("remaining numbers: {:?}", num3);
}

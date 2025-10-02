fn main() {
    let nums: Vec<usize> = vec![5, 1, 4, 2];
    let target = 7;
    let mut pointer = 1;

    for (index, element) in nums.iter().enumerate()  {
        if *element < target {
            let sum = nums[pointer] + *element;

            if sum == target {
                println!("{:?}", vec![index, pointer]);
                return;
            } 

            if sum < target {
                pointer = index;
            }
            else {
                pointer += 1;
            }
        }
    }
}

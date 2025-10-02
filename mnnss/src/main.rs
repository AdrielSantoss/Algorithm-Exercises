fn main() {
    let arr: Vec<isize> = vec![1, 2, 0, -3, 4, 5, -1, 2];
    let mut result_sum = 0;
    let mut prev_sum = 0;

    for (index, element) in arr.iter().enumerate() {
        if *element > 0 {
            prev_sum += element;
        }

        if *element < 0 || index + 1 == arr.len() {            
            if prev_sum > result_sum {
                result_sum = prev_sum.clone();
            }        

            prev_sum = 0;
        } 
    }

    assert_eq!(result_sum, 9);
}

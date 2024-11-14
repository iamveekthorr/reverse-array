fn main() {
    // Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.
    // let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;

    rotate_array(&mut nums, k);
}

fn rotate_array(nums: &mut Vec<i32>, k: i32) {
    let n = nums.len();
    if n == 0 || k == 0 {
        return;
    }

    // Calculate the effective number of rotations
    // If k (the number of rotation steps) is greater than the length of the array n, rotating the array by k steps will give the same result as rotating it by k % n steps.
    // For example, rotating an array of length 7 by 10 steps is equivalent to rotating it by 10 % 7 = 3 steps. This simplifies the problem to a smaller number of steps without changing the outcome.
    let k = k as usize % n;
    if k == 0 {
        return;
    }

    let mut left = 0;
    let mut right = n - 1;
    // Reverse the entire array
    reverse(&mut right, &mut left, nums);

    left = 0;
    right = k - 1;
    // Reverse the first k elements
    reverse(&mut right, &mut left, nums);

    left = k;
    right = n - 1;
    // Reverse the remaining elements
    reverse(&mut right, &mut left, nums);
}

fn reverse(right: &mut usize, left: &mut usize, nums: &mut Vec<i32>) {
    while *left < *right {
        let temp = nums[*left];
        nums[*left] = nums[*right];
        nums[*right] = temp;
        *left += 1;
        *right -= 1;

        println!("{:?}", nums)
    }
}

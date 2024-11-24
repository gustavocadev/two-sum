fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];

  for (idx, &num) in nums.iter().enumerate() {
    if let Some(&next_num) = nums.get(idx + 1) {
      let sum = num + next_num;
      if sum == target {
        result.push(idx as i32);
        result.push((idx + 1) as i32);
      }
    }
  }

  result
}

fn main() {
  let vectors = vec![2, 7, 11, 15];
  let target = 9;

  let result = two_sum(vectors, target);

  println!("{:?}", result);
}

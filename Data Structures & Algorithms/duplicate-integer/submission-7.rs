impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut counter: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            if counter.contains_key(&n) { return true; }
            counter.insert(n, 1);
        }
        return false;
    }
}

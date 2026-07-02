fn traversal(node: Option<Box<Node>>, low: i32, high: i32, mut output: &mut Vec<i32>) {
    if !node.is_none() {
        if node.as_ref().unwrap().val >= low && node.as_ref().unwrap().val <= high {
            output.push(node.as_ref().unwrap().val);
        }
        if node.as_ref().unwrap().val >= low {
            traversal(node.as_ref().unwrap().left.clone(), low, high, &mut output);
        }
        if node.as_ref().unwrap().val <= high {
            traversal(node.as_ref().unwrap().right.clone(), low, high, &mut output);
        }
    }
}
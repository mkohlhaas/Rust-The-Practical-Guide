fn sorting_lists(vec_list: &mut Vec<Linklist<i32>>) -> Linklist<i32> {
    let mut sorted_list: Linklist<i32> = Linklist::create_empty_list();
    let mut values: Vec<i32> = Vec::new();
    while true {
        let values = vec_list
            .into_iter()
            .map(|x| x.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>();

        let min_val = *values.iter().min().unwrap();
        let min_index = values.iter().position(|x| *x == min_val).unwrap();

        sorted_list.add(min_val);
        vec_list[min_index].remove();

        if vec_list[min_index].head.is_none() {
            vec_list.remove(min_index);
        }
        if vec_list.len() == 0 {
            break;
        }
    }
    sorted_list
}
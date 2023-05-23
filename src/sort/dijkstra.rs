/// * 将所有节点的到起点最小距离设置成Max，起点自身为0
/// * 迭代
///   * 找到到起点最小距离最小的点(第一次是起点)，为其相邻节点更新到起点最小距离
/// * 迭代完成既得最小距离

#[test]
fn test_dijkstra() {
    use std::collections::{HashMap, VecDeque};

    let node_map: HashMap<u32, HashMap<u32, u32>> = HashMap::from([
        // (节点号，[(连接节点，距离),..])
        (1, [(4, 7), (2, 13), (3, 9)].into_iter().collect()),
        (2, [(6, 5)].into_iter().collect()),
        (3, [(2, 3), (5, 12), (6, 9)].into_iter().collect()),
        (4, [(3, 1), (5, 14)].into_iter().collect()),
        (5, [(7, 20)].into_iter().collect()),
        (6, [(5, 2), (7, 30)].into_iter().collect()),
        (7, [].into_iter().collect()),
    ]);
    let (start_idx, end_idx) = (1, 7);

    // 初期化节点距离map，key：节点号，value：(是否检查过，从起点到此最小距离(默认Max))
    let mut node_dis_map: HashMap<_, _> = node_map
        .iter()
        .map(|(&k, _v)| (k, (false, u32::MAX)))
        .collect();
    node_dis_map.insert(start_idx, (false, 0));

    // 每次找到未检查的最小距离节点，更新其相邻节点的最小距离
    for _ in 0..node_map.len() {
        let (idx, min_dis) = node_dis_map
            .iter_mut()
            .filter(|(_, &mut (checked, _))| !checked)
            .min_by_key(|(_, &mut (_, min_dis))| min_dis)
            .map(|(idx, (checked, min_dis))| {
                *checked = true;
                (idx, *min_dis)
            })
            .unwrap();
        let neighbor_nodes = node_map.get(idx).unwrap();
        neighbor_nodes.iter().for_each(|(nb_idx, nb_dis)| {
            let (_, origin_nb_dis) = node_dis_map.get_mut(nb_idx).unwrap();
            if *origin_nb_dis > nb_dis + min_dis {
                *origin_nb_dis = nb_dis + min_dis;
            }
        });
        // dbg!(&node_dis_map);
    }
    // 至此最短距离已算出
    dbg!(&node_dis_map);

    // 反向生成各节点来源节点距离map，用于显示最短路径
    let mut back_map = HashMap::new(); //(to_idx,[from_idx])
    node_map.iter().for_each(|(from_idx, v)| {
        v.into_iter().for_each(|(to_idx, _)| {
            back_map
                .entry(to_idx)
                .and_modify(|v: &mut Vec<_>| {
                    v.push(from_idx);
                })
                .or_insert(vec![from_idx]);
        })
    });
    // dbg!(&back_map);
    let mut best_path = VecDeque::new();
    let mut i = end_idx;
    // 从最终节点开始根据最短距离map得出最短路径
    while i != start_idx {
        let front_idxs = back_map.get(&i).expect(&format!("No node can go to {}", i));
        let (min_idx, _) = front_idxs
            .iter()
            .filter_map(|idx| node_dis_map.get(idx).map(|(_, dis)| (**idx, *dis)))
            .min_by_key(|(idx, dis)| *dis + node_map.get(idx).unwrap().get(&i).unwrap())
            .unwrap();
        best_path.push_front(min_idx);
        i = min_idx;
    }
    dbg!(best_path);
}

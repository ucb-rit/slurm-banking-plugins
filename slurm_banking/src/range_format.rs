fn parse_range(range: &str) -> Vec<String> {
    let range_parts: Vec<&str> = range.split("-").collect();
    if range_parts.len() != 2 {
        return Vec::new();
    }

    let start_str = range_parts.get(0).unwrap();
    let end_str = range_parts.get(1).unwrap();
    let start = start_str.parse::<u32>().unwrap();
    let end = end_str.parse::<u32>().unwrap();
    let mut result = Vec::new();
    for i in start..(end + 1) {
        result.push(format!("{1:0.0$}", end_str.len(), i));
    }
    result
}

struct CartesianProduct<T: Clone> {
    xs: Box<Iterator<Item = T>>,
    ys: Box<Vec<T>>,
    curr_x: Option<T>,
    ys_idx: usize
}

impl<T: Clone> CartesianProduct<T> {
    fn new(mut xs: Box<Iterator<Item = T>>, ys: Box<Iterator<Item = T>>) -> CartesianProduct<T>{
        let curr_x = xs.next();
        CartesianProduct {
            xs: xs,
            curr_x: curr_x,
            ys: Box::new(ys.collect()),
            ys_idx: 0
        }
    }
}

impl<T: Clone> Iterator for CartesianProduct<T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<(T, T)> {
        if self.ys_idx >= self.ys.len() {
            self.curr_x = self.xs.next();
            self.ys_idx = 0;
        }
        match &self.curr_x {
            Some(x) => {
                let result = (x.clone(), self.ys.get(self.ys_idx).unwrap().clone());
                self.ys_idx = self.ys_idx + 1;
                Some(result)
            },
            None => None
        }
    }
}

fn cartesian_product<T: Clone>(xs: Vec<T>, ys: Vec<T>) -> Vec<(T, T)> {
    let mut result = Vec::new();
    for x in xs {
        for y in &ys {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

fn expand_node_single_group(group: &str) -> Vec<String> {
    // "n00[01-20].savio[0-1]"

    let parts = group.split("[");
    // [ "n00", "01-20].savio", "0-1]" ]

    let parts = parts.flat_map(|x| x.split("]"));
    // [ "n00", "01-20", ".savio", "0-1", "" ]

    let parts = parts.enumerate();
    // [ (0, "n00"), (1, "01-20"), (2, ".savio"), (3, "0-1"), (4, "") ]

    let mut results = Vec::new();
    results.push("".to_string());
    for (i, val) in parts {
        if i % 2 == 0 {
            // it's a constant
            results = results.into_iter().map(|x| x + val).collect();
        } else {
            // it's a range
            let range = parse_range(val);
            results = CartesianProduct::new(Box::new(results.into_iter()), Box::new(range.into_iter())).into_iter().map(|(x,y)| x + &y).collect();
        }
    }
    results
}

// Expand hostname ranges, for example n000[0-2].savio1 becomes ["n0000", "n0001", "n0002"]
pub fn expand_node_hostnames(names: &str) -> Vec<String> {
    let groups = names.split(",");
    let mut result = Vec::new();
    for group in groups {
        result.extend(expand_node_single_group(group));
    }
    result
}

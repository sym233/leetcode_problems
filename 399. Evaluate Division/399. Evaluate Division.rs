use std::collections::{ HashMap, VecDeque };
use std::iter::FromIterator;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut variables = HashMap::<String, usize>::new();
        let mut var_count = 0;
        for eq in &equations {
            for var in eq {
                if !variables.contains_key(var) {
                    variables.insert(var.clone(), var_count);
                    var_count += 1;
                }
            }
        }
        
        // divs[a][b] = f -> a / b = f
        let mut divs: Vec<Vec<f64>> = vec![vec![-1.0; var_count]; var_count];
        let mut values = values.into_iter();
        for eq in equations {
            let a = *variables.get(&eq[0]).unwrap();
            let b = *variables.get(&eq[1]).unwrap();
            let value = values.next().unwrap();
            divs[a][b] = value;
            divs[b][a] = 1.0 / value;
            divs[a][a] = 1.0;
            divs[b][b] = 1.0;
            
        }
        
        
        for i in 0..var_count {
            let mut neighbors = VecDeque::from_iter(0..var_count);
            while let Some(j) = neighbors.pop_front() {
                if i == j || divs[i][j] == -1.0 {
                    continue;
                }
                for k in 0..var_count {
                    if k == i || k == j {
                        continue;
                    }
                    if divs[i][k] == -1.0 && divs[j][k] != -1.0 {
                        divs[i][k] = divs[i][j] * divs[j][k];
                        neighbors.push_back(k);
                    }

                }
            }
        }        
        
        let mut res: Vec<f64> = Vec::new();
        
        for q in queries {
            if let Some(&c) = variables.get(&q[0]) {
                if let Some(&d) = variables.get(&q[1]) {
                    if divs[c][d] != -1.0 {
                        res.push(divs[c][d]);
                        continue;
                    }                    
                }
            }
            res.push(-1.0);
        }
        
        return res;
    }
}

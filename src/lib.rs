use num::Float;
use std::error::Error;
mod parse;
mod types;
mod util;
use types::{BinaryOperator, Expression, Node};

fn priorized_indices<T: Float>(bin_ops: &Vec<BinaryOperator<T>>) -> Vec<usize> {
    let mut indices: Vec<_> = (0..bin_ops.len()).collect();
    indices.sort_by(|i1, i2| {
        bin_ops[*i2]
            .priority
            .partial_cmp(&bin_ops[*i1].priority)
            .unwrap()
    });
    indices
}

fn eval_expression<T: Float + std::fmt::Debug>(exp: &Expression<T>) -> T {
    let indices = priorized_indices(&exp.bin_ops);
    let mut numbers = exp
        .nodes
        .iter()
        .map(|n| match n {
            Node::Expr(e) => eval_expression(e),
            Node::Num(n) => *n,
        })
        .collect::<Vec<T>>();
    let mut num_inds = indices.clone();
    for (i, &bin_op_idx) in indices.iter().enumerate() {
        let num_idx = num_inds[i];
        let num_1 = numbers[num_idx];
        let num_2 = numbers[num_idx + 1];
        numbers[num_idx] = (exp.bin_ops[bin_op_idx].f)(num_1, num_2);
        numbers.remove(num_idx + 1);
        for j in num_inds.iter_mut() {
            if *j > num_idx {
                *j = *j - 1;
            }
        }
    }
    numbers[0]
}

type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn eval(text: &str) -> BoxResult<f32> {
    let exp = parse::parse(text)?;
    Ok(eval_expression(&exp))
}

#[cfg(test)]
mod tests {

    use crate::{eval, priorized_indices, types::BinaryOperator, util::assert_float_eq};
    #[test]
    fn test_prio() {
        assert_eq!(
            priorized_indices(&vec![
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 0
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 1
                }
            ]),
            vec![1, 0]
        );
        assert_eq!(
            priorized_indices(&vec![
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 0
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 1
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 0
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 1
                }
            ]),
            vec![1, 3, 0, 2]
        );
        assert_eq!(
            priorized_indices(&vec![
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 1
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 1
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 0
                },
                BinaryOperator {
                    f: |_, _| 0.0,
                    priority: 0
                }
            ]),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn test_eval() {
        assert_float_eq(eval(&"11.3").unwrap(), 11.3);
        assert_float_eq(eval(&"11.3+0.7").unwrap(), 12.0);
        assert_float_eq(eval(&"31.3+0.7*2").unwrap(), 32.7);
        assert_float_eq(eval(&"1.3+0.7*2-1").unwrap(), 1.7);
        assert_float_eq(eval(&"1.3+0.7*2-1/10").unwrap(), 2.6);
        assert_float_eq(eval(&"(1.3+0.7)*2-1/10").unwrap(), 3.9);
        assert_float_eq(eval(&"1.3+(0.7*2)-1/10").unwrap(), 2.6);
        assert_float_eq(eval(&"1.3+0.7*(2-1)/10").unwrap(), 1.37);
        assert_float_eq(eval(&"1.3+0.7*(2-1/10)").unwrap(), 2.63);
        assert_float_eq(eval(&"0-1*(1.3+0.7*(2-1/10))").unwrap(), -2.63);
        assert_float_eq(eval(&"0-1*((1.3+0.7)*(2-1/10))").unwrap(), -3.8);
    }

    #[test]
    fn test_error_handling() {
        assert!(eval(&"").is_err());
        assert!(eval(&"5+5-(").is_err());
        assert!(eval(&")2*(5+5)*3-2)*2").is_err());
        assert!(eval(&"2*(5+5))").is_err());
    }
}

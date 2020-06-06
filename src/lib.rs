use std::collections::VecDeque;

static base: u8 = 10;

pub struct ProductResult {
    base: i8,
    carry: i8,
}

// fn mult_by_base(array:&mut[i8], pow: i16) {
//     let power = pow;
//     while power > 0 {
//         array.append(0);
//         power -= 1;
//     }
//     return array
// }

fn even_pad(vector: &mut VecDeque<i8>) {
    if vector.len() % 2 == 1 {
        // If the array is odd-length, then
        vector.push_front(0);
    }
}

fn trim_leading_zeroes(vec: &mut Vec<i8>) {
    while vec.len() > 1 && vec[0] == 0 {
        // make sure there are at least 2 elements and the first is zero
        vec.remove(0); // remove the 0th element
    }
}

// fn add(x_array:&mut[&i8], y_array:&mut[&i8], sub_factor: i8){
//     # x_array.reverse()
//     # y_array.reverse()
//     ### Create return araray with all zeroes to simplify carry logic
//     return_length = max(len(x_array), len(y_array)) + 1

//     return_array = [0] * return_length

//     for i in range(0,return_length-1):   # We don't have to loop over the carry byte
//         if i + 1 > len(y_array):  ## x array is longer
//             x = x_array[-1 -i]  ## Get ith element from the end
//             z = return_array[-1 -i]  # Add in current value
//             sum = z + x
//         elif i + 1 > len(x_array):   ### y array is longer
//             y = y_array[-1 -i]  ## Get ith element from the end
//             z = return_array[-1 -i]  # Add in current value
//             sum = z + sub_factor * y
//         else:
//             x = x_array[-1 -i]  ## Get ith element from the end
//             y = y_array[-1 -i]  ## Get ith element from the end
//             z = return_array[-1 -i]  # Add in current value
//             sum = z + x + sub_factor * y
//         # if sum >= 0:
//         return_array[-1 -i] = sum % base
//         if sum < 0:
//             borrow = 1
//             return_array[-1 -i - 1] -=  borrow  # borrow
//         else:
//             return_array[-1 -i - 1] += (sum // base)  # carry
//         # else:
//         #     return_array[-1 -i] = -1* ((-1 * sum) % base)
//         #     if i != 0:
//         #         return_array[-1 -i - 1] = sum // base  # borrow

//     return_array = normalize_array(return_array)
//     return return_array

// fn sub(x_array:&mut[&i8], y_array:&mut[&i8]){
//     return add(x_array, y_array, sub_factor=-1)
// }

// Indicate if leading digit is positive
fn is_positive(vec: &Vec<i8>) -> bool {
    // assert_ne!(vec[0], 0); // vector that has not been trimmed
    if vec.len() > 0 {
        vec[0] >= 0
    } else {
        true
    }
}

// Clean up borrows
// In the end, there should be no leading zeroes and
// all digits should be of the same sign
// If the leadinig digit is positive, then execute negative borrows up the line
// If the leading digit is negative, then execute positive borrows up the line
fn execute_borrows(vec: &mut Vec<i8>) {
    trim_leading_zeroes(vec);
    let positive_number = is_positive(vec);
    let borrow_bit = match is_positive(vec) {
        true => 1,
        false => -1,
    };

    for i in (1..vec.len()).rev() {
        while positive_number && vec[i] < 0 {
            vec[i - 1] -= 1;
            vec[i] += base as i8;
        }
        while !positive_number && vec[i] > 0 {
            vec[i - 1] += 1;
            vec[i] -= base as i8;
        }
    }
}

fn normalize_vector(vec: &mut Vec<i8>) {
    execute_borrows(vec);
    trim_leading_zeroes(vec);
}

pub fn mult_1x1(x: &[i8], y: &[i8]) -> Result<ProductResult, String> {
    assert_eq!(x.len(), 1);
    assert_eq!(y.len(), 1);
    let product = x[0] * y[0];
    Ok(ProductResult {
        base: product % base as i8,
        carry: product / base as i8,
    })
}

pub fn mult_slices(x: &[i8], y: &[i8]) -> Result<Vec<i8>, String> {
    let mut return_vector: Vec<i8> = vec![];

    if x.len() < 1 || y.len() < 1 {
        return Err(String::from("Oops!"));
    }

    if x.len() == 1 && y.len() == 1 {
        if let Ok(p_result) = mult_1x1(x, y) {
            return_vector = vec![p_result.carry, p_result.base];
        }
    }

    if x.len() >= 2 && y.len() >= 2 {
        // even_pad(x);
        // even_pad(y);
        let power = x.len();

        let x_mid = x.len() / 2;
        let y_mid = y.len() / 2;

        let x_high = &x[..x_mid];
        let x_low = &x[x_mid..];
        let y_high = &y[..y_mid];
        let y_low = &y[y_mid..];

        // ### e = (xh + xl)*(yh+yl) - a - d
        // ### product = ab^2 + eb + d
        // let a = mult(x_high, y_high);
        // let d = mult(x_low, y_low);
        // let add1 = add(x_high, x_low);
        // let add2 = add(y_high,y_low);
        // let prod = mult(add1, add2);
        // let e = sub(sub(mult(add(x_high, x_low),  add(y_high,y_low)),a), d);
        // let return_array = add(add(mult_by_base(a,power), mult_by_base(e,power/2)), d)
    }
    // if return_array == []:
    //     println!("panic")
    // normalize_array(return_array)
    // return return_vector
    normalize_vector(&mut return_vector);
    Ok(return_vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_leading_zeroes() {
        let mut vec = vec![0, 0, 5, -1];
        trim_leading_zeroes(&mut vec);
        assert_eq!(vec, vec![5, -1]);
    }

    #[test]
    fn test_execute_borrows() {
        let mut vec = vec![1, 5];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![1, 5]);

        let mut vec = vec![0, 1, 5];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![1, 5]);

        let mut vec = vec![0, -1, 5];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![-5]);

        let mut vec = vec![0, 5, -1, 5];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![4, 9, 5]);

        let mut vec = vec![0, 1, 5, -1, 5];
        normalize_vector(&mut vec);
        assert_eq!(vec, vec![1, 4, 9, 5]);
    }

    #[test]
    fn test_mult_slices_null_slices() {
        let x = vec![];
        let y = vec![1];

        let result = mult_slices(&x[..], &y[..]);
        assert_eq!(result, Err(String::from("Oops!")))
    }
    #[test]
    fn test_mult_slices_2x1_digit_numbers() {
        let x = vec![4];
        let y = vec![2];

        let result = mult_slices(&x[..], &y[..]);
        assert_eq!(result, Ok(vec![8]));
    }

    #[test]
    fn test_mult_slices_2x1_digit_numbers_with_carry() {
        let x = vec![4];
        let y = vec![8];

        let result = mult_slices(&x[..], &y[..]);
        assert_eq!(result, Ok(vec![3, 2]));
    }

    #[test]
    fn test_mult_slices_2x2_digit_numbers_with_carry() {
        let x = vec![1, 2];
        let y = vec![2, 1];

        let result = mult_slices(&x[..], &y[..]);
        // assert_eq!(result, Ok(vec![2, 5, 2]));
    }

    #[test]
    fn test_even_pad() {
        let mut deque: VecDeque<i8> = VecDeque::new();
        for i in 3..8 {
            deque.push_back(i);
        }
        even_pad(&mut deque);
        // let slice = deque[..];
        assert_eq!(
            deque.into_iter().collect::<Vec<i8>>(),
            vec![0, 3, 4, 5, 6, 7]
        );
    }
}

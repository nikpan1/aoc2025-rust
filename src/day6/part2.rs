use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).expect("Reading file failed.")
}

struct Number {
    number: u8,
    number_index: u128,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_part2() {
        let mut values: Vec<String> = read_file("src/day6/input.txt").lines()
            .map(|x| x.to_string())
            .collect();

        let operations: String = values.pop().unwrap();
        let operation_indexes: Vec<usize> = operations.chars()
            .enumerate()
            .filter(|(_, op)| *op != ' ')
            .map(|(i, _)| i)
            .collect();

        let mut ops: HashMap<usize, Vec<Number>> = HashMap::new();
        for mut value in values {
            value  += " ";
            let mut line = value.chars();

            let mut op_index = 0;
            for i in 0..value.len() {
                let current = line.nth(i).unwrap();
                if current != ' ' {
                    ops.entry(op_index).or_insert_with(Vec::new).push(Number {
                        number: current as u8,
                        number_index: i as u128,
                    });
                    let next = line.nth(i + 1 );
                    match next {
                        None => println!("end"),
                        Some(n) => if n == ' ' {
                            op_index += 1;
                        },
                    }
                }
            }
        }



        // iteruj znak po znaku i patrz
        // jak znak != " " to znaczy ze chcemy dodac wartosc do mapy
        // jesli current == " " oraz previous != " " to znaczy ze inkrementujemy wartosc klucza
        //
        // w sumie to nie musi byc mapa, po otrzymaniu operaions.len() mozemy prealokacje zrobic Vec<Vec<Number>>
        //
        // po tym wszystkim liczymy max index i od niego zaczynamy (bierzmey wszystkie o tym indeksie i sortujemy po linen umber i otrzymujemy nasza liczbe

        assert_eq!(1, 1);
    }
}

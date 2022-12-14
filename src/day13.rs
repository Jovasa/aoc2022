use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use serde_json::Value;

fn compare(a: &Value, b: &Value) -> Ordering {
    if a.is_number() && b.is_number() {
        if a.as_i64().unwrap() < b.as_i64().unwrap() {
            return Less;
        } else if a.as_i64().unwrap() > b.as_i64().unwrap() {
            return Greater;
        }
        return Equal;
    } else if a.is_array() && b.is_number() {
        if a.as_array().unwrap().len() == 0 {
            return Less;
        }
        let x = a.as_array().unwrap().iter().next().unwrap();
        return if x.is_array() {
            compare(x, b)
        } else {
            if x.as_i64().unwrap() < b.as_i64().unwrap() { Less } else { Greater }
        };
    } else if a.is_number() && b.is_array() {
        if b.as_array().unwrap().len() == 0 {
            return Greater;
        }
        let x = b.as_array().unwrap().iter().next().unwrap();
        return if x.is_array() {
            compare(a, x)
        } else {
            if a.as_i64().unwrap() < x.as_i64().unwrap() { Less } else { Greater }
        };
    } else if a.is_array() && b.is_array() {
        if a.as_array().unwrap().len() == 0 && b.as_array().unwrap().len() == 0 {
            return Equal;
        }
        if a.as_array().unwrap().len() == 0 {
            return Less;
        }
        for (i, j) in a.as_array().unwrap().iter().zip(b.as_array().unwrap().iter()) {
            let i1 = if i.is_array() && b.is_array() {
                compare(i, j)
            } else if i.is_array() {
                compare(i, b)
            } else if j.is_array() {
                compare(a, j)
            } else {
                if i.as_i64().unwrap() < j.as_i64().unwrap() { Less } else if i.as_i64().unwrap() == j.as_i64().unwrap() {Equal} else { Greater }
            };
            if i1 != Equal {
                return i1;
            }
        }
        return if a.as_array().unwrap().len() > b.as_array().unwrap().len() {
            Greater
        } else { Less };
    }
    unreachable!("{:?} {:?}", a, b);
}

fn main() {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();

    let lines = input.lines().collect::<Vec<&str>>();
    let mut iter = lines.iter();
    let mut total = 0;

    let mut packets = Vec::new();

    for i in 1.. {
        let first = iter.next();
        let second = iter.next();
        if first == None || second == None {
            break;
        }
        let first = first.unwrap();
        let second = second.unwrap();
        let first = serde_json::from_str::<Value>(first).unwrap();
        let second = serde_json::from_str::<Value>(second).unwrap();
        if compare(&first, &second) == Less {
            total += i;
        }
        packets.push(first);
        packets.push(second);
        iter.next();
    }
    println!("{}", total);

    let temp1 = serde_json::from_str::<Value>("[[2]]").unwrap();
    packets.push(temp1.clone());
    let temp2 = serde_json::from_str::<Value>("[[6]]").unwrap();
    packets.push(temp2.clone());
    packets.sort_by(|a, b| compare(a, b));
    println!("{:?}", (packets.iter().position(|x| x == &temp1).unwrap() + 1) * (packets.iter().position(|x| x == &temp2).unwrap() + 1));

}
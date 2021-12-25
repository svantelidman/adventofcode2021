use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Literal {
        value: usize
    },
    Comma,
    LeftBracket,
    RightBracket
}

impl Token {
    fn add(self, val: usize) -> Token {
        match self {
            Token::Literal{value} => Token::Literal{value: value + val},
            _ => self
        }
    }
}

fn tokenize(chars: &mut std::iter::Peekable<&mut std::str::Chars>) -> Vec<Token> {
    fn parse_usize(chars: &mut std::iter::Peekable<&mut std::str::Chars>) -> usize {
        let mut num_chars: Vec<char> = vec!();
        loop {
            match chars.peek() {
                Some(c) => {
                    if c.is_numeric() {
                        num_chars.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                },
                None => panic!("Unexpected end of stream when parsing literal")
            }
        }
        let num_chars: String = num_chars.into_iter().collect();
        num_chars.parse().unwrap()
    }
    let mut tokens: Vec<Token> = vec!();
    loop {
        match chars.peek() {
            Some('[') => {tokens.push(Token::LeftBracket); chars.next();}, 
            Some(']') => {tokens.push(Token::RightBracket); chars.next();}, 
            Some(',') => {tokens.push(Token::Comma); chars.next();}, 
            Some(_) => tokens.push(Token::Literal{ value: parse_usize(chars)}),
            None => break
        }
    }
    tokens
}

fn add(first: Vec<Token>, second: Vec<Token>) -> Vec<Token> {
    let mut sum = vec!();
    sum.push(Token::LeftBracket);
    sum.append(&mut first.clone());
    sum.push(Token::Comma);
    sum.append(&mut second.clone());
    sum.push(Token::RightBracket);
    reduce(sum)
}

fn explode(input: &Vec<Token>) -> Option<Vec<Token>> {
    let mut exploded: Vec<Token> = vec!();
    let mut it = input.into_iter();
    let did_explode = false;
    let mut level = 0;
    loop {
        match it.next(){
            Some(Token::LeftBracket) => {
                level +=1; 
                if level == 5 {
                    let first_value = match it.next() {
                        Some(Token::Literal{value}) => value,
                        Some(t) => panic!("First element was not a literal: {:?}", t),
                        None => panic!("Missing first element")
                    };
                    assert_eq!(it.next(), Some(&Token::Comma));
                    let second_value = if let Some(Token::Literal{value}) = it.next() {
                        value
                    } else {
                        panic!("First element was not a literal")
                    };
                    exploded.reverse();
                    if let Some(first_lit_pos) = exploded.iter().position(|t| match t { Token::Literal{value: _} => true, _ => false }) {
                        exploded[first_lit_pos] = exploded[first_lit_pos].clone().add(*first_value);
                    }
                    exploded.reverse();
                    exploded.push(Token::Literal{value: 0});

                    assert_eq!(it.next(), Some(&Token::RightBracket));
                    let mut added = false;
                    loop {
                        match it.next() {
                            Some(Token::Literal{value}) => {
                                exploded.push(
                                    if added {
                                        Token::Literal{value: *value}
                                    } else {
                                        added = true;
                                        Token::Literal{value: value + second_value}
                                    }
                                )

                            },
                            Some(t) => exploded.push(t.clone()),
                            None => return Some(exploded)
                        }
                    }
                } else {
                    exploded.push(Token::LeftBracket);
                }
            },
            Some(Token::RightBracket) => {level -=1; exploded.push(Token::RightBracket)},
            Some(t) => exploded.push(t.clone()),
            None => break
        }
    }
    if did_explode {
        Some(exploded)
    } else {
        None
    }
}

fn split(input: &Vec<Token>) -> Option<Vec<Token>> {
    let mut splitted: Vec<Token> = vec!();
    let mut it = input.into_iter();
    let mut did_split = false;
    loop {
        match it.next() {
            Some(Token::Literal{value}) => {
                if !did_split && *value >= 10 {
                    let val_1 = value / 2;
                    let val_2 = value - val_1;
                    splitted.push(Token::LeftBracket);
                    splitted.push(Token::Literal{value: val_1});
                    splitted.push(Token::Comma);
                    splitted.push(Token::Literal{value: val_2});
                    splitted.push(Token::RightBracket);
                    did_split = true;
                } else {
                    splitted.push(Token::Literal{value: *value})
                }
            }
            Some(t) => splitted.push(t.clone()),
            None => break
        }
    }
    if did_split {
        Some(splitted)
    } else {
        None
    }
}

fn reduce(input: Vec<Token>) -> Vec<Token> {
    let mut reduced = input.clone();
    loop {
        if let Some(exploded) = explode(&reduced) {
            reduced = exploded
        } else {
            if let Some(splitted) = split(&reduced) {
                reduced = splitted
            } else {
                break
            }
        }
    }
    reduced
}

fn sum(rows: &Vec<Vec<Token>>) -> Vec<Token> {
    let mut it = rows.iter();
    let mut result = it.next().unwrap().clone();
    loop {
        match it.next() {
            Some(row) => {
                result = add(result, row.clone())
            }
            None => break
        }
    }
    result
}

fn magnitude(token_stream: &mut std::slice::Iter<Token>) -> usize {
    match token_stream.next() {
        Some(Token::Literal{value}) => {*value}
        Some(Token::LeftBracket)  => {
            let left_value = magnitude(token_stream);
            token_stream.next();
            let right_value = magnitude(token_stream);
            token_stream.next();
            3 * left_value + 2 * right_value
        }
        _ => panic!("Unexpected token when calculating magnitude")
    }
}

fn do_home_work_2(rows: &Vec<Vec<Token>>) -> usize {
    let row_pairs: Vec<_> = rows.iter().combinations(2).map(|v| vec!((v[0].clone(), v[1].clone()), (v[1].clone(), v[0].clone()))).flatten().collect();
    row_pairs.into_iter().map(|(r1, r2)| magnitude(&mut add(r1, r2).iter())).max().unwrap()
}

fn main() {
    let rows: Vec<_> = include_str!("../input").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
    let the_sum = sum(&rows);
    println!("Answer part 1: {}", magnitude(&mut the_sum.iter()));
    println!("Answer part 2: {}", do_home_work_2(&rows));
}

mod test {
    use super::*;

    #[test]
    fn test_explode_1() {
        let input = tokenize(&mut (&mut "[[[[[9,8],1],2],3],4]".chars()).peekable());
        println!("{:?}", input);
        let exploded = explode(&input);
        if let Some(tokens) = exploded {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[[[[0,9],2],3],4]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_explode_2() {
        let input = tokenize(&mut (&mut "[7,[6,[5,[4,[3,2]]]]]".chars()).peekable());
        println!("{:?}", input);
        let exploded = explode(&input);
        if let Some(tokens) = exploded {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[7,[6,[5,[7,0]]]]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_explode_3() {
        let input = tokenize(&mut (&mut "[[6,[5,[4,[3,2]]]],1]".chars()).peekable());
        println!("{:?}", input);
        let exploded = explode(&input);
        if let Some(tokens) = exploded {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[[6,[5,[7,0]]],3]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_explode_4() {
        let input = tokenize(&mut (&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars()).peekable());
        println!("{:?}", input);
        let exploded = explode(&input);
        if let Some(tokens) = exploded {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_explode_5() {
        let input = tokenize(&mut (&mut "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars()).peekable());
        println!("{:?}", input);
        let exploded = explode(&input);
        if let Some(tokens) = exploded {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_split() {
        let input = tokenize(&mut (&mut "[11,1]".chars()).peekable());
        println!("{:?}", input);
        let splitted = split(&input);
        if let Some(tokens) = splitted {
            assert_eq!(
                tokens,
                tokenize(&mut (&mut "[[5,6],1]".chars()).peekable())
            )
        } else {
            panic!("No tokens returned")
        }
    }

    #[test]
    fn test_reduce() {
        let input = tokenize(&mut (&mut "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".chars()).peekable());
        println!("{:?}", input);
        let reduced = reduce(input.clone());
        assert_eq!(
            reduced,
            tokenize(&mut (&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars()).peekable())
        )
    }

    #[test]
    fn test_add() {
        let inp1 = tokenize(&mut (&mut "[[[[4,3],4],4],[7,[[8,4],9]]]".chars()).peekable());
        let inp2 = tokenize(&mut (&mut "[1,1]".chars()).peekable());
        let reduced = add(inp1, inp2);
        assert_eq!(
            reduced,
            tokenize(&mut (&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars()).peekable())
        )
    }

    #[test]
    fn test_sum_1() {
        let rows: Vec<_> = include_str!("../sum1").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        let the_sum = sum(&rows);
        assert_eq!(
            the_sum,
            tokenize(&mut (&mut "[[[[1,1],[2,2]],[3,3]],[4,4]]".chars()).peekable())
        )
    }
    #[test]
    fn test_sum_2() {
        let rows: Vec<_> = include_str!("../sum2").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        let the_sum = sum(&rows);
        assert_eq!(
            the_sum,
            tokenize(&mut (&mut "[[[[3,0],[5,3]],[4,4]],[5,5]]".chars()).peekable())
        )
    }
    #[test]
    fn test_sum_3() {
        let rows: Vec<_> = include_str!("../sum3").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        let the_sum = sum(&rows);
        assert_eq!(
            the_sum,
            tokenize(&mut (&mut "[[[[5,0],[7,4]],[5,5]],[6,6]]".chars()).peekable())
        )
    }
    #[test]
    fn test_sum_4() {
        let rows: Vec<_> = include_str!("../sum4").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        let the_sum = sum(&rows);
        assert_eq!(
            the_sum,
            tokenize(&mut (&mut "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".chars()).peekable())
        )
    }

    #[test]
    fn test_magnitude_1() {
        let input = tokenize(&mut (&mut "[[1,2],[[3,4],5]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            143
        )
    }
    #[test]
    fn test_magnitude_2() {
        let input = tokenize(&mut (&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            1384
        )
    }
    #[test]
    fn test_magnitude_3() {
        let input = tokenize(&mut (&mut "[[[[1,1],[2,2]],[3,3]],[4,4]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            445
        )
    }
    #[test]
    fn test_magnitude_4() {
        let input = tokenize(&mut (&mut "[[[[3,0],[5,3]],[4,4]],[5,5]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            791
        )
    }
    #[test]
    fn test_magnitude_5() {
        let input = tokenize(&mut (&mut "[[[[5,0],[7,4]],[5,5]],[6,6]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            1137
        )
    }
    #[test]
    fn test_magnitude_6() {
        let input = tokenize(&mut (&mut "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".chars()).peekable());
        assert_eq!(
            magnitude(&mut input.iter()),
            3488
        )
    }

    #[test]
    fn test_homework() {
        let rows: Vec<_> = include_str!("../testhw").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        let the_sum = sum(&rows);
        assert_eq!(
            magnitude(&mut the_sum.iter()),
            4140
        )
    }

    #[test]
    fn test_homework_2() {
        let rows: Vec<_> = include_str!("../testhw").lines().map(|line| tokenize(&mut (&mut line.chars()).peekable())).collect();
        assert_eq!(
            do_home_work_2(&rows),
            3993
        )
    }


}

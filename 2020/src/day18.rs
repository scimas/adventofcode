use parsing::{
    self, char_parse_builder, choose, combination, left, map, right, whitespace_wrap, whole_number,
    zero_or_more, ParseResult, Parser,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_expressions() -> Vec<String> {
    let f = File::open("res/input18.txt").expect("Couldn't open day 18 input");
    let reader = BufReader::new(f);

    reader
        .lines()
        .map(|line| line.expect("Couldn't read line from day 18 input"))
        .collect()
}

fn open_bracket(s: &str) -> ParseResult<char> {
    whitespace_wrap(char_parse_builder('(')).parse(s)
}

fn close_bracket(s: &str) -> ParseResult<char> {
    whitespace_wrap(char_parse_builder(')')).parse(s)
}

fn plus_sign(s: &str) -> ParseResult<char> {
    whitespace_wrap(char_parse_builder('+')).parse(s)
}

fn times_sign(s: &str) -> ParseResult<char> {
    whitespace_wrap(char_parse_builder('*')).parse(s)
}

fn bracketed_expr_p1(s: &str) -> ParseResult<u64> {
    right(open_bracket, left(expression_p1, close_bracket)).parse(s)
}

fn binary_operator(s: &str) -> ParseResult<char> {
    choose(plus_sign, times_sign).parse(s)
}

fn p_term_p1(s: &str) -> ParseResult<u64> {
    choose(map(whole_number, |n| Some(n as u64)), bracketed_expr_p1).parse(s)
}

fn expression_p1(s: &str) -> ParseResult<u64> {
    map(
        combination(
            p_term_p1,
            zero_or_more(combination(binary_operator, p_term_p1)),
        ),
        |(a, b)| {
            let mut acc = a as u64;
            for (op, n) in b {
                if op == '+' {
                    acc += n as u64;
                } else if op == '*' {
                    acc *= n as u64;
                }
            }
            Some(acc)
        },
    )
    .parse(s)
}

pub fn part1() -> u64 {
    let exprs = load_expressions();
    exprs
        .iter()
        .map(|exp| expression_p1(exp).unwrap().0 as u64)
        .sum()
}

fn bracketed_expr_p2(s: &str) -> ParseResult<u64> {
    right(open_bracket, left(expression_p2, close_bracket)).parse(s)
}

fn p_term_p2(s: &str) -> ParseResult<u64> {
    choose(map(whole_number, |n| Some(n as u64)), bracketed_expr_p2).parse(s)
}

fn precedence(op1: char, _op2: char) -> bool {
    op1 == '+'
}

fn binop(op: char) -> impl Fn(u64, u64) -> u64 {
    if op == '+' {
        |a, b| a + b
    } else if op == '*' {
        |a, b| a * b
    } else {
        unreachable!()
    }
}

fn expression_p2(s: &str) -> ParseResult<u64> {
    map(
        combination(
            p_term_p2,
            zero_or_more(combination(binary_operator, p_term_p2)),
        ),
        |(a, b)| {
            let mut oprt_stack: Vec<char> = Vec::new();
            let mut oprn_stack: Vec<u64> = Vec::new();
            oprn_stack.push(a);
            for (op, n) in b {
                let mut pushed = false;
                while let Some(top_op) = oprt_stack.pop() {
                    if precedence(op, top_op) {
                        oprt_stack.push(top_op);
                        oprt_stack.push(op);
                        oprn_stack.push(n);
                        pushed = true;
                        break;
                    } else {
                        let oprn2 = oprn_stack.pop().unwrap();
                        let oprn1 = oprn_stack.pop().unwrap();
                        let res = binop(top_op)(oprn1, oprn2);
                        oprn_stack.push(res);
                    }
                }
                if !pushed {
                    oprt_stack.push(op);
                    oprn_stack.push(n);
                }
            }
            while let Some(op) = oprt_stack.pop() {
                let oprn2 = oprn_stack.pop().unwrap();
                let oprn1 = oprn_stack.pop().unwrap();
                let res = binop(op)(oprn1, oprn2);
                oprn_stack.push(res);
            }
            Some(oprn_stack.pop().unwrap())
        },
    )
    .parse(s)
}

pub fn part2() -> u64 {
    let exprs = load_expressions();
    exprs
        .iter()
        .map(|exp| expression_p2(exp).unwrap().0 as u64)
        .sum()
}

#[test]
fn expression_evaluates_p1() {
    assert_eq!(Ok((71, "")), expression_p1.parse("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(
        Ok((51, "")),
        expression_p1.parse("1 + (2 * 3) + (4 * (5 + 6))")
    );
    assert_eq!(Ok((26, "")), expression_p1.parse("2 * 3 + (4 * 5)"));
    assert_eq!(
        Ok((437, "")),
        expression_p1.parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    );
    assert_eq!(
        Ok((12240, "")),
        expression_p1.parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    );
    assert_eq!(
        Ok((13632, "")),
        expression_p1.parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    );
}

#[test]
fn expression_evaluates_p2() {
    assert_eq!(Ok((12, "")), expression_p2("3 * 2 + 2"));
    assert_eq!(Ok((231, "")), expression_p2.parse("1 + 2 * 3 + 4 * 5 + 6"));
    assert_eq!(
        Ok((51, "")),
        expression_p2.parse("1 + (2 * 3) + (4 * (5 + 6))")
    );
    assert_eq!(Ok((46, "")), expression_p2.parse("2 * 3 + (4 * 5)"));
    assert_eq!(
        Ok((1445, "")),
        expression_p2.parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    );
    assert_eq!(
        Ok((669060, "")),
        expression_p2.parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    );
    assert_eq!(
        Ok((23340, "")),
        expression_p2.parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    );
}

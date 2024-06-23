/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/*
 * Copyright (c) 2024 David Jackson
 */

use super::compile_error::{CompileError, CompileErrorType};
use super::token::{Token, TokenType};

const SINGLE_LINE_COMMENT_CHAR: char = '#';

struct Lexer {
    i: usize,
    chars: Vec<char>,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
}

impl Lexer {
    fn analyze(mut self) -> Result<Vec<Token>, CompileError> {
        while self.i < self.chars.len() {
            let ch = self.chars[self.i];
            if ch.is_alphabetic() {
                let token = self.lex_identifier()?;
                self.tokens.push(token);
            } else if ch.is_ascii_digit() {
                let token = self.lex_number()?;
                self.tokens.push(token);
            } else if ch == ',' {
                self.add_token(TokenType::Comma, self.line, self.col);
                self.i += 1;
                self.col += 1;
            } else if ch == '\n' {
                self.line += 1;
                self.col = 1;
                self.i += 1;
            } else if ch.is_whitespace() {
                // Skip whitespace
                self.i += 1;
                self.col += 1;
            } else if ch == SINGLE_LINE_COMMENT_CHAR {
                self.lex_single_line_comment();
            } else {
                return Err(CompileError::new(
                    CompileErrorType::InvalidCharacter,
                    self.line,
                    self.col,
                ));
            }
        }
        Ok(self.tokens)
    }

    fn lex_identifier(&mut self) -> Result<Token, CompileError> {
        let col = self.col;
        let identifier = self.lex_while(|ch| ch.is_alphabetic());
        let keywords = [
            ("at", TokenType::At),
            ("circle", TokenType::Circle),
            ("square", TokenType::Square),
            ("entity", TokenType::Entity),
            ("grid", TokenType::Grid),
            ("height", TokenType::Height),
            ("rect", TokenType::Rect),
            ("width", TokenType::Width),
            ("within", TokenType::Within),
            ("xor", TokenType::Xor),
            ("radius", TokenType::Radius),
            ("line", TokenType::Line),
            ("along", TokenType::Along),
            ("left", TokenType::Left),
            ("right", TokenType::Right),
            ("top", TokenType::Top),
            ("bottom", TokenType::Bottom),
            ("from", TokenType::From),
            ("length", TokenType::Length),
            ("stair", TokenType::Stair),
        ];
        if let Some(index) = keywords
            .iter()
            .position(|(keyword, _tok)| *keyword == identifier)
        {
            Ok(Token::new(keywords[index].1, self.line, col))
        } else {
            Err(CompileError::new(
                CompileErrorType::UnrecognizedKeyword,
                self.line,
                col,
            ))
        }
    }

    fn lex_number(&mut self) -> Result<Token, CompileError> {
        let col = self.col;
        let s = self.lex_while(|ch| ch.is_ascii_digit());
        match s.parse::<u32>() {
            Ok(n) => Ok(Token::new(TokenType::Number(n), self.line, col)),
            Err(_) => Err(CompileError::new(
                CompileErrorType::InvalidNumber,
                self.line,
                col,
            )),
        }
    }

    fn lex_single_line_comment(&mut self) {
        while self.i < self.chars.len() && self.chars[self.i] != '\n' {
            self.i += 1;
            self.col += 1;
        }
    }

    fn lex_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut s = String::new();
        while self.i < self.chars.len() && predicate(self.chars[self.i]) {
            let ch = self.chars[self.i];
            s.push(ch);
            self.i += 1;
            self.col += 1;
        }
        s
    }

    fn add_token(&mut self, token_type: TokenType, line: usize, col: usize) {
        let token = Token::new(token_type, line, col);
        self.tokens.push(token);
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, CompileError> {
    let lexer = Lexer {
        chars: input.chars().collect(),
        i: 0,
        tokens: Vec::new(),
        line: 1,
        col: 1,
    };
    lexer.analyze()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_keyword() {
        let input = "grid";
        let tokens = lex(input).expect("bad lex");
        assert_eq!(tokens.len(), 1);

        let token = &tokens[0];
        assert!(matches!(token.token_type, TokenType::Grid));
    }

    #[test]
    fn test_lex_number() {
        let input = "100";
        let tokens = lex(input).expect("bad lex");
        assert_eq!(tokens.len(), 1);
        let token = &tokens[0];
        if let TokenType::Number(n) = token.token_type {
            assert_eq!(n, 100);
        } else {
            panic!("Wrong token type");
        }
    }

    #[test]
    fn test_lex_comma() {
        let input = ",";
        let tokens = lex(input).expect("bad lex");
        assert_eq!(tokens.len(), 1);

        let token = &tokens[0];
        assert!(matches!(token.token_type, TokenType::Comma));
    }

    #[test]
    fn test_ignore_spaces() {
        let input = "grid 10, 10";
        let tokens = lex(input).expect("bad lex");
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_bad_keyword_error() {
        let input = "badkeyword";
        if let Err(err) = lex(input) {
            assert!(matches!(
                err.error_type,
                CompileErrorType::UnrecognizedKeyword
            ));
            assert_eq!(err.position.line, 1);
            assert_eq!(err.position.col, 1);
        } else {
            panic!("Should fail");
        }
    }

    #[test]
    fn test_lex_keywords() {
        let input = "grid at width height rect xor square stair";
        let correct_token_types = vec![
            TokenType::Grid,
            TokenType::At,
            TokenType::Width,
            TokenType::Height,
            TokenType::Rect,
            TokenType::Xor,
            TokenType::Square,
            TokenType::Stair,
        ];
        test_lex(input, &correct_token_types);
    }

    #[test]
    fn test_comments_are_ignored() {
        let input = "grid 10, 10 # a ten-by-ten grid";
        let tokens = lex(input).expect("bad lex");
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_line_number() {
        let input = "grid 10, 10\nrect at 1, 1 width 2 height 2";
        let tokens = lex(input).expect("bad lex");
        let at_token = &tokens[5];
        assert!(matches!(at_token.token_type, TokenType::At));
        assert_eq!(at_token.position.line, 2);
        assert_eq!(at_token.position.col, 6);
    }

    #[test]
    fn test_lex_entity() {
        let input = "entity circle within 5, 5";
        let correct_token_types = vec![
            TokenType::Entity,
            TokenType::Circle,
            TokenType::Within,
            TokenType::Number(5),
            TokenType::Comma,
            TokenType::Number(5),
        ];
        test_lex(input, &correct_token_types);
    }

    #[test]
    fn test_circular_entity_at_point() {
        let input = "entity circle at 5, 5 radius 2";
        let correct_token_types = vec![
            TokenType::Entity,
            TokenType::Circle,
            TokenType::At,
            TokenType::Number(5),
            TokenType::Comma,
            TokenType::Number(5),
            TokenType::Radius,
            TokenType::Number(2),
        ];
        test_lex(input, &correct_token_types);
    }

    #[test]
    fn test_lex_line() {
        let input = "line along left from 1, 2 length 3";
        let correct_token_types = vec![
            TokenType::Line,
            TokenType::Along,
            TokenType::Left,
            TokenType::From,
            TokenType::Number(1),
            TokenType::Comma,
            TokenType::Number(2),
            TokenType::Length,
            TokenType::Number(3),
        ];
        test_lex(input, &correct_token_types);
    }

    fn test_lex(input: &str, expected: &[TokenType]) {
        let tokens = lex(input).expect("Bad lex");
        assert_eq!(tokens.len(), expected.len());
        for (tok, tok_type) in tokens.iter().zip(expected) {
            assert_eq!(
                std::mem::discriminant(&tok.token_type),
                std::mem::discriminant(tok_type)
            );
        }
    }
}

 use crate::lexer::{C1Lexer, C1Token};
 use crate::ParseResult;
 use std::ops::{Deref, DerefMut};

 pub struct C1Parser<'a>(C1Lexer<'a>);
 // Implement Deref and DerefMut to enable the direct use of the lexer's methods
 impl<'a> Deref for C1Parser<'a> {
   type Target = C1Lexer<'a>;

   fn deref(&self) -> &Self::Target {
       &self.0
   }
 }

 impl<'a> DerefMut for C1Parser<'a> {
   fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.0
   }
 }

 impl<'a> C1Parser<'a> {
   pub fn parse(text: &str) -> ParseResult {
       let mut parser = Self::initialize_parser(text);
       parser.program()
   }

    fn initialize_parser(text: &str) -> C1Parser {
        C1Parser(C1Lexer::new(text))
    }

    /// program ::= ( functiondefinition )* <EOF>
    fn program(&mut self) -> ParseResult {
        if self.current_token() == None
        {
            return Ok(());
        }
        else if self.any_match_current(&[C1Token::KwBoolean, C1Token::KwFloat, C1Token::KwInt, C1Token::KwVoid])
        {
            match self.functiondefinition() {
                Ok(()) => { 
                    return self.program();
                },
                Err(text)=>return Err(text),
            }
        }
        else
        {
            return Err(["Expected: KwBoolean or KwFloat or KwInt or KwVoid or EOF in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn functiondefinition(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwBoolean,C1Token::KwFloat,C1Token::KwInt,C1Token::KwVoid])
        {
            match self.return_type(){
                Ok(()) =>{
                    match self.check_and_eat_tokens(&[C1Token::Identifier,C1Token::LeftParenthesis,C1Token::RightParenthesis,C1Token::LeftBrace],["Expected: ID () { in Line:".to_string(),self.get_line()].join("").as_str()){
                        Ok(()) => {
                            match self.statement_list(){
                                Ok(()) => {
                                    return self.check_and_eat_token(&C1Token::RightBrace,["Expected: } in Line:".to_string(),self.get_line()].join("").as_str());
                                },
                                Err(text)=>return Err(text),
                            }
                        },
                        Err(text)=>return Err(text),
                    }
                },
                Err(text)=>return Err(text),
            }
        }

        else
        {
            return Err(["Expected: KwBoolean or KwFloat or KwInt or KwVoid in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn function_call(&mut self) -> ParseResult{
        match self.check_and_eat_token(&C1Token::Identifier,["Expected: ID in Line:".to_string(),self.get_line()].join("").as_str()){
            Ok(()) =>{
                return self.check_and_eat_tokens(&[C1Token::LeftParenthesis,C1Token::RightParenthesis],["Expected: () in Line:".to_string(),self.get_line()].join("").as_str());
            }
            Err(text)=>return Err(text),
        
        }   
    }

    fn function_call2(&mut self) -> ParseResult{
        return self.check_and_eat_tokens(&[C1Token::LeftParenthesis,C1Token::RightParenthesis],["Expected: () in Line:".to_string(),self.get_line()].join("").as_str());
                
    }

    fn statement_list(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::LeftBrace, C1Token::Identifier, C1Token::KwIf, C1Token::KwReturn, C1Token::KwPrintf])
        {
            match self.block(){
                Ok(()) => {
                    return self.statement_list();
                },
                
                Err(text)=>return Err(text),
            }
        }

        else if self.any_match_current(&[C1Token::RightBrace]) || self.current_token() == None
        {
            return Ok(());
        }

        return Err(["Expected: { or ID or KwIf or KwReturn or KwPrintf or } in Line:".to_string(),self.get_line()].join(""));
    
    }

    fn block(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::LeftBrace])
        {
            self.eat();
            match self.statement_list(){
                Ok(()) => {
                    return self.check_and_eat_token(&C1Token::RightBrace, ["Expected: } in Line:".to_string(),self.get_line()].join("").as_str());
                },
                Err(text)=>return Err(text),
            }
        }

        else if self.any_match_current(&[C1Token::Identifier, C1Token::KwIf, C1Token::KwReturn, C1Token::KwPrintf])
        {
            return self.statement();
        }

        else
        {
            return Err(["Expected: { or ID or KwIf or KwReturn or KwPrintf in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn statement(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwIf])
        {
            return self.if_statement();
        }

        else  if self.any_match_current(&[C1Token::KwReturn])
        {
            match self.return_statement(){
                Ok(()) =>{
                    return self.check_and_eat_token(&C1Token::Semicolon,["Expected: ; in Line:".to_string(),self.get_line()].join("").as_str())
                },
                Err(text)=>return Err(text),
            }
        }

        else if self.any_match_current(&[C1Token::KwPrintf])
        {
            match self.printf(){
                Ok(()) =>{
                    return self.check_and_eat_token(&C1Token::Semicolon,["Expected: ; in Line:".to_string(),self.get_line()].join("").as_str())
                },
                Err(text)=>return Err(text),
            }
        }

        else if self.any_match_current(&[C1Token::Identifier])
        {
            self.eat();
            match self.statement2(){
                Ok(()) =>{
                    return self.check_and_eat_token(&C1Token::Semicolon,["Expected: ; in Line:".to_string(),self.get_line()].join("").as_str())
                },
                Err(text)=>return Err(text),
            }
                
        }

        else
        {
            return Err(["Expected: KwIf or KwReturn or KwPrintf or ID in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn statement2(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::LeftParenthesis])
        {
            return self.function_call2();
        }

        else  if self.any_match_current(&[C1Token::Assign])
        {
            return self.stat_assignment2();
        }

        else
        {
            return Err(["Expected: ( or = in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn if_statement(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwIf])
        {
            self.eat();
            match self.check_and_eat_token(&C1Token::LeftParenthesis, ["Expected: ( in Line:".to_string(),self.get_line()].join("").as_str()) {
                Ok(()) => {
                    match self.assignment() {
                        Ok(()) => {
                            match self.check_and_eat_token(&C1Token::RightParenthesis,["Expected: ) in Line:".to_string(),self.get_line()].join("").as_str()) {
                                Ok(()) => {
                                    return self.block();
                                },
                                Err(text)=>return Err(text), 
                            }
                        },
                        Err(text)=>return Err(text), 
                    }
                },
                Err(text)=>return Err(text), 
            }
            
        }

        else 
        {
            return Err(["Expected: KwIf in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn return_statement(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwReturn])
        {
            self.eat();
            return self.return_statement2();
        }

        else 
        {
            return Err(["Expected: KwReturn in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn return_statement2(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Identifier, C1Token::Minus, C1Token::ConstInt, C1Token::ConstFloat, C1Token::ConstBoolean, C1Token::LeftParenthesis])
        {
            return self.assignment();
        }
        
        else if self.any_match_current(&[C1Token::Semicolon]) || self.current_token() == None
        {
            return Ok(());
        }

        else 
        {
            return Err(["Expected: ID or - or ConstInt or ConstFloat or ConstBoolean or ( or ;".to_string(),self.get_line()].join(""));
        }
    }

    fn printf(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwPrintf])
        {
            self.eat();
            match self.check_and_eat_token(&C1Token::LeftParenthesis,["Expected: ( in Line:".to_string(),self.get_line()].join("").as_str())
            {
                Ok(()) => {
                    match self.assignment() {
                        Ok(()) => {
                            return self.check_and_eat_token(&C1Token::RightParenthesis, ["Expected: ) in Line:".to_string(),self.get_line()].join("").as_str())
                        },
                        Err(text)=>return Err(text),
                    }
                },
                Err(text)=>return Err(text),  
            }
        }

        else 
        {
            return Err(["Expected: KwPrintf in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn return_type(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::KwBoolean, C1Token::KwFloat, C1Token::KwInt, C1Token::KwVoid])
        {
            self.eat();
            return Ok(());
        }

        else 
        {
            return Err(["Expected: KwBoolean or KwFloat or KwInt or KwVoid in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn stat_assignment(&mut self) -> ParseResult{
        
        match self.check_and_eat_tokens(&[C1Token::Identifier, C1Token::Assign],["Expected: ID = in Line:".to_string(),self.get_line()].join("").as_str()){
            Ok(()) => {
                return self.assignment();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn stat_assignment2(&mut self) -> ParseResult{
        match self.check_and_eat_token(&C1Token::Assign, ["Expected: = in Line:".to_string(),self.get_line()].join("").as_str()){
            Ok(()) => {
                return self.assignment();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn assignment(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Identifier])
        {
            self.eat();
            return self.assignment2();
        }

        else if self.any_match_current(&[C1Token::Minus, C1Token::ConstInt, C1Token::ConstFloat, C1Token::ConstBoolean, C1Token::Identifier, C1Token::LeftParenthesis])
        {
            return self.exproi();
        }

        else 
        {
            return Err(["Expected: <ID> or - or ConstInt or ConstFloat or ConstBoolean or ( in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn assignment2(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Assign])
        {
            self.eat();
            return self.assignment();
        }

        else if self.any_match_current(&[C1Token::Plus, C1Token::Minus, C1Token::Or, C1Token::Less, C1Token::Greater, C1Token::LessEqual, C1Token::GreaterEqual, C1Token::Equal, C1Token::NotEqual, C1Token::LeftParenthesis, C1Token::Asterisk, C1Token::Slash, C1Token::And]) || self.current_token() == None
        {
            return self.exprmi();
        }

        else if self.any_match_current(&[C1Token::RightParenthesis, C1Token::Semicolon]) || self.current_token() == None
        {
            return Ok(());
        }

        else 
        {
            return Err(["Expected: = or + or - or * or / or || or && or > or >= or < or <= or == or != or ( or ) or ; in Line:".to_string(),self.get_line()].join(""));
        }

    }

    fn expr(&mut self) -> ParseResult{ // !!!
        match self.simpexpr() {
            Ok(()) => {
                return self.expr2();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn expr2(&mut self) -> ParseResult{ // nullable
        if self.any_match_current(&[C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual])
        {
            match self.operator() {
                Ok(()) => {
                    return self.simpexpr();
                },
                Err(text)=>return Err(text), 
            }
        }

        else if self.any_match_current(&[C1Token::RightParenthesis, C1Token::Semicolon]) || self.current_token() == None
        {
            return Ok(());
        }

        else 
        {
            return Err(["Expected: > or >= or < or <= or == or != or ) or ; in Line:".to_string(),self.get_line()].join(""));
        }        
    }

    fn exprmi(&mut self) -> ParseResult{ // !!! nullable
        if self.any_match_current(&[C1Token::RightParenthesis, C1Token::Semicolon]) || self.current_token() == None
        {
            return Ok(());
        }
        else
        {
            match self.simpexprmi() {
                Ok(()) => {
                    return self.expr2();
                },
                Err(text)=>return Err(text), 
            }
        }

    }

    fn exproi(&mut self) -> ParseResult{ // !!!
        match self.simpexproi() {
            Ok(()) => {
                return self.expr2();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn operator(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual])
        {
            self.eat();
            return Ok(());
        }

        else 
        {
            return Err(["Expected: > or >= or < or <= or == or != in Line:".to_string(),self.get_line()].join(""));
        }    

    }

    fn simpexpr(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Minus])
        {
            self.eat()
        }

        match self.term() { // !!!
            Ok(()) => {
                return self.simpexpr2();
            }
            Err(text)=>return Err(text), 
        }

    }

    fn simpexpr2(&mut self) -> ParseResult{ //nullable
        if self.any_match_current(&[C1Token::Plus, C1Token::Minus, C1Token::Or])
        {
            self.eat();
            match self.term() {
                Ok(()) => {
                    return self.simpexpr2();
                },
                Err(text)=>return Err(text), 
            }
        }

        else if self.any_match_current(&[C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual, C1Token::RightParenthesis, C1Token::Semicolon]) || self.current_token() == None
        {
           return Ok(());
        }
        else
        {
            return Err(["Expected: + or - or || or > or >= or < or <= or == or != or ) or ; in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn simpexprmi(&mut self) -> ParseResult{ //nullable
        if self.any_match_current(&[C1Token::RightParenthesis, C1Token::Semicolon, C1Token::Greater, C1Token::GreaterEqual, C1Token::Less, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual]) || self.current_token() == None
        {
            return Ok(());
        }
        else 
        {
            match self.termmi() { // !!!
                Ok(()) => {
                    return self.simpexpr2();
                }
                Err(text)=>return Err(text), 
            }
        }
    }

    fn simpexproi(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::Minus])
        {
            self.eat();
            match self.term() {
                Ok(()) => {
                    return self.simpexpr2();
                },
                Err(text)=>return Err(text), 
            }
        }

        else if self.any_match_current(&[C1Token::ConstInt, C1Token::ConstFloat, C1Token::ConstBoolean, C1Token::Identifier, C1Token::LeftParenthesis])
        {
            match self.termoi() {
                Ok(()) => {
                    return self.simpexpr2();
                },
                Err(text)=>return Err(text), 
            }
        }

        else
        {
            return Err(["Expected: ConstInt or ConstFloat or ConstBoolean or Identifier or ( in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn term(&mut self) -> ParseResult{
        match self.factor() {
            Ok(()) => {
                return self.term2();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn term2(&mut self) -> ParseResult{ //nullable
        if self.any_match_current(&[C1Token::Asterisk, C1Token::Slash, C1Token::And])
        {
            self.eat();
            match self.factor()
            {
                Ok(()) => {
                    return self.term2();
                },
                Err(text)=>return Err(text), 
            }
        }

        else if self.any_match_current(&[C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual, C1Token::RightParenthesis, C1Token::Semicolon, C1Token::Plus, C1Token::Minus, C1Token::Or]) || self.current_token() == None
        {
            return Ok(());
        }

        else 
        {
            return Err(["Expected: * or / or && or > or < or >= or <= or == or != or ) or ; or + or - or || in Line:".to_string(),self.get_line()].join(""));
        }

    }

    fn termmi(&mut self) -> ParseResult{ //nullable
        if self.any_match_current(&[C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual, C1Token::RightParenthesis, C1Token::Semicolon, C1Token::Plus, C1Token::Minus, C1Token::Or]) || self.current_token() == None
        {
            return Ok(());
        }
        else
        {
            match self.factor2() {
                Ok(()) => {
                    return self.term2();
                },
                Err(text)=>return Err(text), 
            }
        }
    }

    fn termoi(&mut self) -> ParseResult{
        match self.factoroi() {
            Ok(()) => {
                return self.term2();
            },
            Err(text)=>return Err(text), 
        }
    }

    fn factor(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::ConstInt])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::ConstFloat])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::ConstBoolean])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::Identifier])
        {
            self.eat();
            return self.factor2();
        }

        else if self.any_match_current(&[C1Token::LeftParenthesis])
        {
            self.eat();
            match self.assignment() {
                Ok(()) => {
                    return self.check_and_eat_token(&C1Token::RightParenthesis,["Expected: ) in Line:".to_string(),self.get_line()].join("").as_str());
                },
                Err(text)=>return Err(text), 
            }
        }

        else 
        {
            return Err(["Expected: ConstInt or ConstFloat or ConstBoolean or Identifier or ( in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn factor2(&mut self) -> ParseResult{ //nullable
        if self.any_match_current(&[C1Token::LeftParenthesis])
        {
            return self.function_call2();
        }

        else if self.any_match_current(&[C1Token::RightParenthesis, C1Token::Semicolon, C1Token::Greater, C1Token::Less, C1Token::GreaterEqual, C1Token::LessEqual, C1Token::Equal, C1Token::NotEqual, C1Token::Plus, C1Token::Minus, C1Token::Or, C1Token::Asterisk, C1Token::Slash, C1Token::And]) || self.current_token() == None
        {
            return Ok(());
        }

        else 
        {
            return Err(["Expected: ( or ) or ; or > or < or >= or <= or == or != or + or - or || or * or / or && in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn factoroi(&mut self) -> ParseResult{
        if self.any_match_current(&[C1Token::ConstInt])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::ConstFloat])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::ConstBoolean])
        {
            self.eat();
            return Ok(());
        }

        else if self.any_match_current(&[C1Token::LeftParenthesis])
        {
            self.eat();
            match self.assignment() {
                Ok(()) => {
                    return self.check_and_eat_token(&C1Token::RightParenthesis,["Expected: ) in Line:".to_string(),self.get_line()].join("").as_str());
                },
                Err(text)=>return Err(text), 
            }
        }

        else 
        {
            return Err(["Expected: ConstInt or ConstFloat or ConstBoolean or ( in Line:".to_string(),self.get_line()].join(""));
        }
    }

    fn get_line(&self) -> String {
        match self.current_line_number(){
            Some(number) => return number.to_string(),
            None => return "Unknown".to_string(),
        }
    }
   // TODO: implement remaining grammar

   /// Check whether the current token is equal to the given token. If yes, consume it, otherwise
   /// return an error with the given error message
   fn check_and_eat_token(&mut self, token: &C1Token, error_message: &str) -> ParseResult {
  
    if self.current_matches(token) {
           self.eat();
           Ok(())
       } else {
           Err(String::from(error_message))
       }
   }

   /// For each token in the given slice, check whether the token is equal to the current token,
   /// consume the current token, and check the next token in the slice against the next token
   /// provided by the lexer.
   fn check_and_eat_tokens(&mut self, token: &[C1Token], error_message: &str) -> ParseResult {  
    match token
           .iter()
           .map(|t| self.check_and_eat_token(t, error_message))
           .filter(ParseResult::is_err)
           .last()
       {
           None => Ok(()),
           Some(err) => err,
       }
   }

   /// Check whether the given token matches the current token
   fn current_matches(&self, token: &C1Token) -> bool {
       match &self.current_token() {
           None => false,
           Some(current) => current == token,
       }
   }

//   /// Check whether the given token matches the next token
//   fn next_matches(&self, token: &C1Token) -> bool {
//       match &self.peek_token() {
//           None => false,
//           Some(next) => next == token,
//       }
//   }

   /// Check whether any of the tokens matches the current token.
   fn any_match_current(&self, token: &[C1Token]) -> bool {
       token.iter().any(|t| self.current_matches(t))
   }

//   /// Check whether any of the tokens matches the current token, then consume it
//    fn any_match_and_eat(&mut self, token: &[C1Token], error_message: &String) -> ParseResult {
//        if token
//            .iter()
//            .any(|t| self.check_and_eat_token(t, "").is_ok())
//        {
//            Ok(())
//        } else {
//            Err(String::from(error_message))
//        }
//    }

//    fn error_message_current(&self, reason: &'static str) -> String {
//        match self.current_token() {
//            None => format!("{}. Reached EOF", reason),
//            Some(_) => format!(
//                "{} at line {:?} with text: '{}'",
//                reason,
//                self.current_line_number().unwrap(),
//                self.current_text().unwrap()
//            ),
//        }
//    }

//    fn error_message_peek(&mut self, reason: &'static str) -> String {
//        match self.peek_token() {
//              None => format!("{}. Reached EOF", reason),
//            Some(_) => format!(
//                "{} at line {:?} with text: '{}'",
//                reason,
//                self.peek_line_number().unwrap(),
//                self.peek_text().unwrap()
//            ),
//        }
//    }
 }

 #[cfg(test)]
 mod tests {
   use crate::parser::{C1Parser, ParseResult};

   fn call_method<'a, F>(parse_method: F, text: &'static str) -> ParseResult
   where
       F: Fn(&mut C1Parser<'a>) -> ParseResult,
   {
       let mut parser = C1Parser::initialize_parser(text);
       if let Err(message) = parse_method(&mut parser) {
           eprintln!("Parse Error: {}", message);
           Err(message)
       } else {
           Ok(())
       }
   }

   #[test]
   fn parse_empty_program() {
       let result = C1Parser::parse("");
       assert_eq!(result, Ok(()));

       let result = C1Parser::parse("   ");
       assert_eq!(result, Ok(()));

       let result = C1Parser::parse("// This is a valid comment!");
       assert_eq!(result, Ok(()));

       let result = C1Parser::parse("/* This is a valid comment!\nIn two lines!*/\n");
       assert_eq!(result, Ok(()));

       let result = C1Parser::parse("  \n ");
       assert_eq!(result, Ok(()));
   }

   #[test]
   fn fail_invalid_program() {
       let result = C1Parser::parse("  bool  ");
       println!("{:?}", result);
       assert!(result.is_err());

       let result = C1Parser::parse("int x = 0;");
       println!("{:?}", result);
       assert!(result.is_err());

       let result = C1Parser::parse("// A valid comment\nInvalid line.");
       println!("{:?}", result);
       assert!(result.is_err());
   }

   #[test]
   fn valid_function() {
       let result = C1Parser::parse("  void foo() {}  ");
       assert!(result.is_ok());

       let result = C1Parser::parse("int bar() {return 0;}");
       assert!(result.is_ok());

       let result = C1Parser::parse(
           "float calc() {\n\
       x = 1.0;
       y = 2.2;
       return x + y;
       \n\
       }",
       );
       assert!(result.is_ok());
   }

   #[test]
   fn fail_invalid_function() {
       let result = C1Parser::parse("  void foo()) {}  ");
       println!("{:?}", result);
       assert!(result.is_err());

       let result = C1Parser::parse("const bar() {return 0;}");
       println!("{:?}", result);
       assert!(result.is_err());

       let result = C1Parser::parse(
           "int bar() {
                                                         return 0;
                                                    int foo() {}",
       );
       println!("{:?}", result);
       assert!(result.is_err());

       let result = C1Parser::parse(
           "float calc(int invalid) {\n\
       int x = 1.0;
       int y = 2.2;
       return x + y;
       \n\
       }",
       );
       println!("{:?}", result);
       assert!(result.is_err());
   }

   #[test]
   fn valid_function_call() {
       assert!(call_method(C1Parser::function_call, "foo()").is_ok());
       assert!(call_method(C1Parser::function_call, "foo( )").is_ok());
       assert!(call_method(C1Parser::function_call, "bar23( )").is_ok());
   }

   #[test]
   fn fail_invalid_function_call() {
       assert!(call_method(C1Parser::function_call, "foo)").is_err());
       assert!(call_method(C1Parser::function_call, "foo{ )").is_err());
       assert!(call_method(C1Parser::function_call, "bar _foo( )").is_err());
   }

   #[test]
   fn valid_statement_list() {
       assert!(call_method(C1Parser::statement_list, "x = 4;").is_ok());//CHANGED EXPECTED FROM OK TO ERR
       assert!(call_method(
           C1Parser::statement_list,
           "x = 4;\n\
       y = 2.1;"
       )
       .is_ok());//CHANGED EXPECTED FROM OK TO ERR
       assert!(call_method(
           C1Parser::statement_list,
           "x = 4;\n\
       {\
       foo();\n\
       }"
       )
       .is_ok());
       assert!(call_method(C1Parser::statement_list, "{x = 4;}\ny = 1;\nfoo();\n{}").is_ok());//CHANGED EXPECTED FROM OK TO ERR
   }

   #[test]
   fn fail_invalid_statement_list() {
       assert!(call_method(
           C1Parser::statement_list,
           "x = 4\n\
       y = 2.1;"
       )
       .is_err());
       assert!(call_method(
           C1Parser::statement_list,
           "x = 4;\n\
       {\
       foo();"
       )
       .is_err());
       assert!(call_method(C1Parser::statement_list, "{x = 4;\ny = 1;\nfoo;\n{}").is_err());
   }

   #[test]
   fn valid_if_statement() {
       assert!(call_method(C1Parser::if_statement, "if(x == 1) {}").is_ok());
       assert!(call_method(C1Parser::if_statement, "if(x == y) {}").is_ok());
       assert!(call_method(C1Parser::if_statement, "if(z) {}").is_ok());
       assert!(call_method(C1Parser::if_statement, "if(true) {}").is_ok());
       assert!(call_method(C1Parser::if_statement, "if(false) {}").is_ok());
   }

   #[test]
   fn fail_invalid_if_statement() {
       assert!(call_method(C1Parser::if_statement, "if(x == ) {}").is_err());
       assert!(call_method(C1Parser::if_statement, "if( == y) {}").is_err());
       assert!(call_method(C1Parser::if_statement, "if(> z) {}").is_err());
       assert!(call_method(C1Parser::if_statement, "if( {}").is_err());
       assert!(call_method(C1Parser::if_statement, "if(false) }").is_err());
   }

   #[test]
   fn valid_return_statement() {
       assert!(call_method(C1Parser::return_statement, "return x").is_ok());
       assert!(call_method(C1Parser::return_statement, "return 1").is_ok());
       assert!(call_method(C1Parser::return_statement, "return").is_ok());
   }

   #[test]
   fn fail_invalid_return_statement() {
       assert!(call_method(C1Parser::return_statement, "1").is_err());
   }

   #[test]
   fn valid_printf_statement() {
       assert!(call_method(C1Parser::printf, " printf(a+b)").is_ok());
       assert!(call_method(C1Parser::printf, "printf( 1)").is_ok());
       assert!(call_method(C1Parser::printf, "printf(a - c)").is_ok());
   }

   #[test]
   fn fail_invalid_printf_statement() {
       assert!(call_method(C1Parser::printf, "printf( ").is_err());
       assert!(call_method(C1Parser::printf, "printf(printf)").is_err());
       assert!(call_method(C1Parser::printf, "Printf()").is_err());
   }

   #[test]
   fn valid_return_type() {
       assert!(call_method(C1Parser::return_type, "void").is_ok());
       assert!(call_method(C1Parser::return_type, "bool").is_ok());
       assert!(call_method(C1Parser::return_type, "int").is_ok());
       assert!(call_method(C1Parser::return_type, "float").is_ok());
   }

   #[test]
   fn valid_assignment() {
       assert!(call_method(C1Parser::assignment, "x = y").is_ok());
       assert!(call_method(C1Parser::assignment, "x =y").is_ok());
       assert!(call_method(C1Parser::assignment, "1 + 2").is_ok());
   }

   #[test]
   fn valid_stat_assignment() {
       assert!(call_method(C1Parser::stat_assignment, "x = y").is_ok());
       assert!(call_method(C1Parser::stat_assignment, "x =y").is_ok());
       assert!(call_method(C1Parser::stat_assignment, "x =y + t").is_ok());
   }

   #[test]
   fn valid_factor() {
       assert!(call_method(C1Parser::factor, "4").is_ok());
       assert!(call_method(C1Parser::factor, "1.2").is_ok());
       assert!(call_method(C1Parser::factor, "true").is_ok());
       assert!(call_method(C1Parser::factor, "foo()").is_ok());
       assert!(call_method(C1Parser::factor, "x").is_ok());
       assert!(call_method(C1Parser::factor, "(x + y)").is_ok());
   }

   #[test]
   fn fail_invalid_factor() {
       assert!(call_method(C1Parser::factor, "if").is_err());
       assert!(call_method(C1Parser::factor, "(4").is_err());
       assert!(call_method(C1Parser::factor, "bool").is_err());
   }

   #[test]
   fn multiple_functions() {
       assert!(call_method(
           C1Parser::program,
           "void main() { hello();}\nfloat bar() {return 1.0;}"
       )
       .is_ok());
   }
 }

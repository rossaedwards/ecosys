//! Yezion Frontend Parser
//! 
//! Parses .yez meta-script files

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "yez_frontend/grammar.pest"]
pub struct YezParser;

pub fn parse_yez(source: &str) -> Result<Vec<YezNode>, String> {
    let pairs = YezParser::parse(Rule::program, source)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let mut ast = Vec::new();
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::oracle_decl => {
                ast.push(parse_oracle(pair));
            }
            Rule::query_decl => {
                ast.push(parse_query(pair));
            }
            Rule::schema_decl => {
                ast.push(parse_schema(pair));
            }
            Rule::ritual_decl => {
                ast.push(parse_ritual(pair));
            }
            _ => {}
        }
    }
    
    Ok(ast)
}

#[derive(Debug, Clone)]
pub enum YezNode {
    Oracle {
        name: String,
        properties: Vec<(String, OracleValue)>,
    },
    Query {
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
        divine_block: DivineBlock,
    },
    Schema {
        entities: Vec<Entity>,
    },
    Ritual {
        name: String,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum OracleValue {
    String(String),
    Boolean(bool),
    Integer(i64),
}

#[derive(Debug, Clone)]
pub struct DivineBlock {
    pub oracle: String,
    pub properties: Vec<(String, Expression)>,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: String,
    pub fields: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let {
        name: String,
        value: Expression,
    },
    Echo {
        expr: Expression,
    },
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

// Parser helper functions
fn parse_oracle(pair: pest::iterators::Pair<Rule>) -> YezNode {
    // TODO: Implement oracle parsing
    YezNode::Oracle {
        name: "placeholder".to_string(),
        properties: vec![],
    }
}

fn parse_query(pair: pest::iterators::Pair<Rule>) -> YezNode {
    // TODO: Implement query parsing
    YezNode::Query {
        name: "placeholder".to_string(),
        params: vec![],
        return_type: "Any".to_string(),
        divine_block: DivineBlock {
            oracle: "placeholder".to_string(),
            properties: vec![],
        },
    }
}

fn parse_schema(pair: pest::iterators::Pair<Rule>) -> YezNode {
    // TODO: Implement schema parsing
    YezNode::Schema {
        entities: vec![],
    }
}

fn parse_ritual(pair: pest::iterators::Pair<Rule>) -> YezNode {
    // TODO: Implement ritual parsing
    YezNode::Ritual {
        name: "placeholder".to_string(),
        body: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_oracle() {
        let source = r#"
            oracle WeatherApi {
                source: "https://api.weather.com"
                cache: true
            }
        "#;
        
        let result = parse_yez(source);
        assert!(result.is_ok());
    }
}

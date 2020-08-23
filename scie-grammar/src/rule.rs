use crate::inter::{IRawRepository, IRawGrammar, ILocation, IRawRule, IRawCaptures};
use dyn_clone::{clone_trait_object, DynClone};
use std::borrow::Borrow;
use crate::grammar::grammar::Grammar;
use core::fmt;

pub struct RuleFactory {}

impl RuleFactory {
    fn _compile_captures(captures: Option<Box<IRawCaptures>>, helper: &mut Grammar, repository: IRawRepository) -> Option<CaptureRule> {
        let r = None;

        r
    }
    pub fn get_compiled_rule_id(mut desc: IRawRule, helper: &mut Grammar, repository: IRawRepository) -> i32 {
        match desc.id {
            None => {
                let id = helper.register_id();
                desc.id = Some(id.clone());

                match desc.match_s {
                    None => {}
                    Some(match_s) => {
                        MatchRule::new(
                            desc.location.clone(),
                            id.clone(),
                            desc.name.clone(),
                            match_s.clone(),
                            RuleFactory::_compile_captures(desc.captures.clone(), helper, repository.clone()),
                        );
                    }
                };
                // helper.register_rule();
            }
            Some(_) => {}
        }

        desc.id.unwrap()
    }

    pub fn create_capture_rule() {}
}


#[derive(Clone, Debug)]
pub struct Rule {
    pub location: Option<ILocation>,
    pub id: i32,
    pub name: Option<String>,
    pub content_name: Option<String>,
}

impl Rule {
    pub fn new(location: ILocation, id: i32, name: Option<String>, content_name: Option<String>) -> Self {
        Rule { location: Some(location), id, name, content_name }
    }
}

#[derive(Clone, Debug)]
pub struct RegExpSource {}

pub trait AbstractRule: DynClone {}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AbstractRule")
    }
}

clone_trait_object!(AbstractRule);

#[derive(Clone, Debug)]
pub struct IncludeOnlyRule {
    pub rule: Rule
}

impl AbstractRule for IncludeOnlyRule {}

#[derive(Clone, Debug)]
pub struct BeginWhileRule {
    pub rule: Rule
}

impl AbstractRule for BeginWhileRule {}

#[derive(Clone, Debug)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Option<CaptureRule>,
}

impl MatchRule {
    pub fn new(location: Option<ILocation>, id: i32, name: Option<String>, match_s: String, captures: Option<CaptureRule>) -> Self {
        MatchRule {
            rule: Rule {
                location,
                id,
                name,
                content_name: None,
            },
            _match: RegExpSource {},
            captures,
        }
    }
}

impl AbstractRule for MatchRule {}

#[derive(Clone, Debug)]
pub struct BeginEndRule {
    pub rule: Rule
}

impl AbstractRule for BeginEndRule {}

#[derive(Clone, Debug)]
pub struct CaptureRule {
    pub rule: Rule
}

impl AbstractRule for CaptureRule {}


// todo: trait with types
// https://users.rust-lang.org/t/impl-trait-with-generic-function-for-generic-struct/27083/2
pub trait IRuleRegistry {
    // type Output;
    // fn method(&self) -> Self::Output;

    fn register_id(&mut self) -> i32;
    fn get_rule(&self, pattern_id: i32) -> Rule;
    fn register_rule(&mut self, result: Box<dyn AbstractRule>) -> Box<dyn AbstractRule>;
}

pub trait IGrammarRegistry {
    fn get_external_grammar(&self, scope_name: String, repository: IRawRepository) -> Option<IRawGrammar>;
}

pub trait IRuleFactoryHelper: IGrammarRegistry + IRuleRegistry {}
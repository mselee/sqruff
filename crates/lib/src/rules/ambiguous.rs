use crate::core::rules::base::ErasedRule;

pub mod AM01;
pub mod AM06;

pub fn rules() -> Vec<ErasedRule> {
    use crate::core::rules::base::Erased as _;

    vec![AM01::RuleAM01.erased(), AM06::RuleAM06::default().erased()]
}

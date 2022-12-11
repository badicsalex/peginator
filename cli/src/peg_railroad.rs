// Copyright (C) 2022, Alex Badics
// This file is part of peginator
// Licensed under the MIT license. See LICENSE file in the project root for details.

use peginator_codegen::{
    grammar::{
        CharRangePart, CharRule, CharRulePart, CharacterRange, Choice, Closure,
        DelimitedExpression, DirectiveExpression, EndOfInput, ExternRule, Field, Grammar_rules,
        Group, IncludeRule, NegativeLookahead, Optional, OverrideField, PositiveLookahead, Rule,
        Sequence, StringLiteral,
    },
    Grammar,
};
use railroad::RailroadNode;

pub fn print_railroad_svg(grammar: &Grammar) {
    let mut dia = railroad::Diagram::new(grammar.generate_railroad());

    dia.add_default_css();

    println!("{dia}");
}

trait GenerateRailroad {
    fn generate_railroad(&self) -> Box<dyn RailroadNode>;
}

impl GenerateRailroad for Grammar {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Diagram::new(railroad::VerticalGrid::new(
            self.rules.iter().map(|r| r.generate_railroad()).collect(),
        )))
    }
}

impl GenerateRailroad for Grammar_rules {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        match self {
            Grammar_rules::CharRule(x) => x.generate_railroad(),
            Grammar_rules::ExternRule(x) => x.generate_railroad(),
            Grammar_rules::Rule(x) => x.generate_railroad(),
        }
    }
}

impl GenerateRailroad for Rule {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        let mut seq = railroad::Sequence::default();
        let is_exported = self
            .directives
            .iter()
            .any(|d| matches!(d, DirectiveExpression::ExportDirective(_)));
        if is_exported {
            seq.push(Box::new(railroad::Start));
        } else {
            seq.push(Box::new(railroad::SimpleStart));
        }
        seq.push(Box::new(railroad::Comment::new(self.name.clone())));
        seq.push(Box::new(self.definition.generate_railroad()));
        if is_exported {
            seq.push(Box::new(railroad::End));
        } else {
            seq.push(Box::new(railroad::SimpleEnd));
        }
        Box::new(seq)
    }
}

impl GenerateRailroad for CharRule {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        let mut seq = railroad::Sequence::default();
        seq.push(Box::new(railroad::SimpleStart));
        seq.push(Box::new(railroad::Comment::new(self.name.clone())));
        seq.push(Box::new(railroad::Choice::new(
            self.choices.iter().map(|c| c.generate_railroad()).collect(),
        )));
        seq.push(Box::new(railroad::SimpleEnd));
        Box::new(seq)
    }
}

impl GenerateRailroad for ExternRule {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        let mut seq = railroad::Sequence::default();
        seq.push(Box::new(railroad::SimpleStart));
        seq.push(Box::new(railroad::Comment::new(self.name.clone())));
        seq.push(Box::new(railroad::NonTerminal::new(
            self.directive.function.last().unwrap().clone(),
        )));
        seq.push(Box::new(railroad::SimpleEnd));
        Box::new(seq)
    }
}

impl GenerateRailroad for CharRulePart {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        match self {
            CharRulePart::CharRangePart(x) => x.generate_railroad(),
            CharRulePart::CharacterRange(x) => x.generate_railroad(),
            CharRulePart::Identifier(x) => Box::new(railroad::NonTerminal::new(x.clone())),
        }
    }
}

impl GenerateRailroad for CharRangePart {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Terminal::new(
            char::try_from(self).unwrap().escape_debug().to_string(),
        ))
    }
}

impl GenerateRailroad for CharacterRange {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Terminal::new(format!(
            "{}..{}",
            char::try_from(&self.from).unwrap().escape_debug(),
            char::try_from(&self.to).unwrap().escape_debug(),
        )))
    }
}

impl GenerateRailroad for Choice {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Choice::new(
            self.choices.iter().map(|c| c.generate_railroad()).collect(),
        ))
    }
}

impl GenerateRailroad for Sequence {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Sequence::new(
            self.parts.iter().map(|c| c.generate_railroad()).collect(),
        ))
    }
}

impl GenerateRailroad for DelimitedExpression {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        match self {
            DelimitedExpression::CharacterRange(x) => x.generate_railroad(),
            DelimitedExpression::Closure(x) => x.generate_railroad(),
            DelimitedExpression::EndOfInput(x) => x.generate_railroad(),
            DelimitedExpression::Field(x) => x.generate_railroad(),
            DelimitedExpression::Group(x) => x.generate_railroad(),
            DelimitedExpression::IncludeRule(x) => x.generate_railroad(),
            DelimitedExpression::NegativeLookahead(x) => x.generate_railroad(),
            DelimitedExpression::Optional(x) => x.generate_railroad(),
            DelimitedExpression::OverrideField(x) => x.generate_railroad(),
            DelimitedExpression::PositiveLookahead(x) => x.generate_railroad(),
            DelimitedExpression::StringLiteral(x) => x.generate_railroad(),
        }
    }
}

impl GenerateRailroad for Closure {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        let closure = Box::new(railroad::Repeat::new(
            self.body.generate_railroad(),
            railroad::Empty,
        ));
        if self.at_least_one.is_some() {
            closure
        } else {
            Box::new(railroad::Optional::new(closure))
        }
    }
}

impl GenerateRailroad for EndOfInput {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::NonTerminal::new("EOI".to_string()))
    }
}

impl GenerateRailroad for Field {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        let nonterminal = Box::new(railroad::NonTerminal::new(self.typ.clone()));
        if let Some(name) = &self.name {
            Box::new(railroad::LabeledBox::new(
                nonterminal,
                railroad::Comment::new(name.clone()),
            ))
        } else {
            nonterminal
        }
    }
}

impl GenerateRailroad for OverrideField {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::NonTerminal::new(self.typ.clone()))
    }
}

impl GenerateRailroad for Group {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        self.body.generate_railroad()
    }
}

impl GenerateRailroad for IncludeRule {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::NonTerminal::new(self.rule.clone()))
    }
}

impl GenerateRailroad for NegativeLookahead {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::LabeledBox::new(
            railroad::NonTerminal::new("Inverse of the above".to_string()),
            self.expr.generate_railroad(),
        ))
    }
}

impl GenerateRailroad for PositiveLookahead {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::LabeledBox::new(
            railroad::NonTerminal::new("Lookahead of the above".to_string()),
            self.expr.generate_railroad(),
        ))
    }
}

impl GenerateRailroad for Optional {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Optional::new(self.body.generate_railroad()))
    }
}

impl GenerateRailroad for StringLiteral {
    fn generate_railroad(&self) -> Box<dyn RailroadNode> {
        Box::new(railroad::Terminal::new(
            String::try_from(self).unwrap().escape_debug().collect(),
        ))
    }
}

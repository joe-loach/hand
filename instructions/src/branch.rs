use crate::*;

/// Branch causes a branch to a target address.
#[derive(Pattern)]
#[name = "B"]
pub struct B(Condition, Label);

impl Encodable for B {
    fn schema(&self, obj: &[CIR]) -> Schema {
        let (label, negative) = label(3, obj);
        let label = label / 4;
        let label = if negative {
            label.wrapping_neg()
        } else {
            label
        };

        Schema::new()
            .set(Variable::Condition, cond(2, obj), 32, 28)
            .one(27)
            .one(25)
            .set(Variable::Label, label, 24, 0)
    }
}
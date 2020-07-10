use crate::internal::*;
use itertools::Itertools;

#[derive(Debug, Clone, new, Default, Hash)]
pub struct FiniteReshape {
    pub shape: TVec<usize>,
}

impl Op for FiniteReshape {
    fn name(&self) -> Cow<str> {
        "Reshape".into()
    }

    fn info(&self) -> TractResult<Vec<String>> {
        Ok(vec![format!("to shape: {}", self.shape.iter().join("x"))])
    }

    op_core_lir!();
    op_as_typed_op!();
    not_a_pulsed_op!();
}

tract_linalg::impl_dyn_hash!(FiniteReshape);

impl StatelessOp for FiniteReshape {
    fn eval(&self, mut inputs: TVec<Arc<Tensor>>) -> TractResult<TVec<Arc<Tensor>>> {
        let input = args_1!(inputs);
        let o = unsafe { input.into_tensor().into_shape(&*self.shape)?.into_arc_tensor() };
        Ok(tvec!(o))
    }
}

impl TypedOp for FiniteReshape {
    fn output_facts(&self, inputs: &[&TypedFact]) -> TractResult<TVec<TypedFact>> {
        Ok(tvec!(TypedFact::dt_shape(inputs[0].datum_type, &*self.shape)?))
    }

    as_op!();
}

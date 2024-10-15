use super::observation::Observation;
use super::permutation::Permutation;

/// because of the equivalence of Suit,
/// many Observations are strategically equivalent !
/// so we can reduce the index space of learned
/// Abstractions by de-symmetrizing over the
/// 4! = 24 Suit Permutation group elements. in other words,
/// canonicalization.
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct Equivalence(Observation);

impl From<Observation> for Equivalence {
    fn from(ref observation: Observation) -> Self {
        let isomorphism = Permutation::from(observation);
        let transformed = isomorphism.permute(observation);
        Self(transformed)
    }
}

impl From<Equivalence> for Observation {
    fn from(equivalence: Equivalence) -> Self {
        equivalence.0
    }
}

impl Equivalence {
    pub fn is_canonical(observation: &Observation) -> bool {
        Permutation::from(observation) == Permutation::identity()
    }
}

impl std::fmt::Display for Equivalence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::hand::Hand;
    use crate::cards::permutation::Permutation;
    use crate::cards::street::Street;

    #[test]
    fn false_positives() {
        let observation = Observation::from(Street::Rive);
        let isomorphism = Equivalence::from(observation);
        assert!(Permutation::exhaust()
            .iter()
            .map(|p| p.permute(&observation))
            .map(|o| Equivalence::from(o))
            .all(|i| i == isomorphism));
    }

    #[test]
    fn false_negatives() {
        let observation = Observation::from(Street::Rive);
        let isomorphism = Equivalence::from(observation);
        let transformed = Observation::from(isomorphism);
        assert!(Permutation::exhaust()
            .iter()
            .map(|p| p.permute(&transformed))
            .any(|o| o == observation));
    }

    #[test]
    fn super_symmetry() {
        let a = Equivalence::from(Observation::from((
            Hand::from("2s Ks"),
            Hand::from("2d 5h 8c Tc Th"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("2s Ks"),
            Hand::from("2h 5c 8d Tc Td"),
        )));
        assert!(a == b);
    }

    #[test]
    fn pocket_rank_symmetry() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Ac Ad"),
            Hand::from("Jc Ts 5s"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("As Ah"),
            Hand::from("Js Tc 5c"),
        )));
        assert!(a == b);
    }

    #[test]
    fn public_rank_symmetry() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Td As"),
            Hand::from("Ts Ks Kh"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("Tc Ad"),
            Hand::from("Td Kd Kh"),
        )));
        assert!(a == b);
    }

    #[test]
    fn offsuit_backdoor() {
        let a = Equivalence::from(Observation::from((
            Hand::from("As Jh"),
            Hand::from("Ks Js 2d"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("Ah Jd"),
            Hand::from("Kh Jh 2c"),
        )));
        assert!(a == b);
    }

    #[test]
    fn offsuit_draw() {
        let a = Equivalence::from(Observation::from((
            Hand::from("As Qh"),
            Hand::from("Ks Js 2s"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("Ad Qh"),
            Hand::from("Kd Jd 2d"),
        )));
        assert!(a == b);
    }

    #[test]
    fn monochrome() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Ad Kd"),
            Hand::from("Qd Jd Td"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("As Ks"),
            Hand::from("Qs Js Ts"),
        )));
        assert!(a == b);
    }

    #[test]
    fn antichrome() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Ac Kc"),
            Hand::from("Qs Js Ts"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("As Ks"),
            Hand::from("Qh Jh Th"),
        )));
        assert!(a == b);
    }

    #[test]
    fn semichrome() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Ac Ks"),
            Hand::from("Qc Js Ts"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("Ad Kh"),
            Hand::from("Qd Jh Th"),
        )));
        assert!(a == b);
    }

    #[test]
    fn polychrome() {
        let a = Equivalence::from(Observation::from((
            Hand::from("Ac Kd"),
            Hand::from("Qh Js 9c"),
        )));
        let b = Equivalence::from(Observation::from((
            Hand::from("Ah Ks"),
            Hand::from("Qc Jd 9h"),
        )));
        assert!(a == b);
    }
}

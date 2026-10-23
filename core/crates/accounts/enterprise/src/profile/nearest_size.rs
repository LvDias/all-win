//! Which standard size's default a profile is copied from (S6 AC-2, AC-7, Q60 option c).

use crate::values::SizeAmount;

use super::account_profile::ProfileOrigin;

/// The standard size(s) nearest to `size`, ascending: the absolute difference in amount; below the
/// smallest standard size the smallest, above the largest the largest. Two entries only on an exact
/// tie (the user chooses; the Core never does). Empty only when `standard` is empty.
pub fn nearest_profile_sizes(size: &SizeAmount, standard: &[SizeAmount]) -> Vec<SizeAmount> {
    let mut sorted = standard.to_vec();
    sorted.sort_by_key(SizeAmount::value);
    sorted.dedup();
    let Some(best) = sorted
        .iter()
        .map(|s| (s.value() - size.value()).abs())
        .min()
    else {
        return Vec::new();
    };
    sorted
        .into_iter()
        .filter(|s| (s.value() - size.value()).abs() == best)
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CopySourceError {
    /// Exact tie and no `profile_size` (`*.profile_size_required`).
    #[error("profile size required")]
    ProfileSizeRequired,
    /// `profile_size` is not one of the tie's candidates (`*.profile_size_invalid`).
    #[error("profile size invalid")]
    ProfileSizeInvalid,
}

/// The size whose default an account copies, and the resulting origin. The account's kind does not
/// matter (Q73 b): only the profile is picked here; risk always uses the account's own size.
///
/// - A standard size copies its own default (`size_default`).
/// - Any other size copies the nearest standard size's default (`nearest_size_default`); on a tie
///   `profile_size` must name one candidate, otherwise it is ignored.
///
/// `Ok(None)` only when there is no standard size at all.
pub fn copy_source(
    size: &SizeAmount,
    profile_size: Option<&SizeAmount>,
    standard: &[SizeAmount],
) -> Result<Option<(SizeAmount, ProfileOrigin)>, CopySourceError> {
    if standard.iter().any(|s| s.value() == size.value()) {
        return Ok(Some((*size, ProfileOrigin::SizeDefault)));
    }
    let candidates = nearest_profile_sizes(size, standard);
    let chosen = match candidates.as_slice() {
        [] => return Ok(None),
        [only] => *only,
        _ => {
            let picked = profile_size.ok_or(CopySourceError::ProfileSizeRequired)?;
            *candidates
                .iter()
                .find(|c| c.value() == picked.value())
                .ok_or(CopySourceError::ProfileSizeInvalid)?
        }
    };
    Ok(Some((chosen, ProfileOrigin::NearestSizeDefault)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::test_support::{size, standard};

    fn nearest(amount: &str) -> Vec<String> {
        nearest_profile_sizes(&size(amount), &standard())
            .iter()
            .map(|s| s.value().normalize().to_string())
            .collect()
    }

    #[test]
    fn nearest_by_absolute_difference_with_clamps() {
        assert_eq!(nearest("1000"), ["25000"]);
        assert_eq!(nearest("25000"), ["25000"]);
        assert_eq!(nearest("30000"), ["25000"]);
        assert_eq!(nearest("37499.99"), ["25000"]);
        assert_eq!(nearest("37500.01"), ["50000"]);
        assert_eq!(nearest("120000"), ["100000"]);
        assert_eq!(nearest("299999"), ["300000"]);
        assert_eq!(nearest("1000000"), ["300000"]);
    }

    #[test]
    fn exact_ties_return_both_ascending() {
        assert_eq!(nearest("37500"), ["25000", "50000"]);
        assert_eq!(nearest("75000"), ["50000", "100000"]);
        assert_eq!(nearest("125000"), ["100000", "150000"]);
        assert_eq!(nearest("175000"), ["150000", "200000"]);
        assert_eq!(nearest("250000"), ["200000", "300000"]);
    }

    #[test]
    fn standard_size_copies_its_own_default() {
        let got = copy_source(&size("50000.00"), None, &standard()).unwrap();
        assert_eq!(got, Some((size("50000"), ProfileOrigin::SizeDefault)));
    }

    #[test]
    fn non_standard_uses_the_nearest() {
        // No tie: profile_size is ignored.
        let got = copy_source(&size("60000"), Some(&size("100000")), &standard());
        assert_eq!(
            got,
            Ok(Some((size("50000"), ProfileOrigin::NearestSizeDefault)))
        );
        assert_eq!(copy_source(&size("60000"), None, &[]), Ok(None));
    }

    #[test]
    fn tie_requires_one_candidate() {
        let s = standard();
        assert_eq!(
            copy_source(&size("75000"), None, &s),
            Err(CopySourceError::ProfileSizeRequired)
        );
        assert_eq!(
            copy_source(&size("75000"), Some(&size("150000")), &s),
            Err(CopySourceError::ProfileSizeInvalid)
        );
        assert_eq!(
            copy_source(&size("75000"), Some(&size("100000.00")), &s),
            Ok(Some((size("100000"), ProfileOrigin::NearestSizeDefault)))
        );
    }
}

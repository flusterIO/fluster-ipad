import Mathlib

/-!
# Checks of the finite-algebra and calculus steps in A. Mueller,
"The Lorentz Factor as the Slope of an Additional Spatial Coordinate: Embedding, Painleve-Gullstrand Flow, and the
Issue Of Simultaneity" (Zenodo preprint, v1, 16 Nov 2026), read from the PDF `main.pdf`.

What this file proves (standard axioms only): the identities of sections 2.2.2, 2.4.4, 2.5 (Proposition 1, the ODE
step), 2.5.1 (Buchdahl arithmetic), 2.8 (Painleve-Gullstrand, Hubble flow).  Every theorem here is a statement the
paper makes; each is true as stated.

What it does NOT prove: the paper's physical content.  The paper itself says (2.4.4, 2.5, 2.8.4) that the Hamiltonian
constraint (3)R = 16 pi G rho / c^2 and the static vacuum equations are IMPORTED from general relativity, so nothing
here (or there) is a derivation of GR from the fourth coordinate.  Nothing here touches the Hypothesis M
(horizon-limited Machian inertia, Milky Way rotation curve) or Appendix A (numerical alignments).
-/

namespace MuellerChecks
open Real

/-! ## Section 2.2: gamma as the slope -/
section lorentz
variable {c v : ℝ}

noncomputable def gam (c v : ℝ) : ℝ := 1 / Real.sqrt (1 - v ^ 2 / c ^ 2)
/-- d_x y = beta gamma, eq. (22)/(25) -/
noncomputable def sx (c v : ℝ) : ℝ := (v / c) * gam c v

theorem one_sub_pos (hc : 0 < c) (hvc : v < c) (hv : 0 ≤ v) : 0 < 1 - v ^ 2 / c ^ 2 := by
  have : v ^ 2 / c ^ 2 < 1 := by rw [div_lt_one (by positivity)]; nlinarith
  linarith

theorem gam_sq (hc : 0 < c) (hvc : v < c) (hv : 0 ≤ v) : gam c v ^ 2 * (1 - v ^ 2 / c ^ 2) = 1 := by
  have hq := one_sub_pos hc hvc hv
  unfold gam
  rw [div_pow, one_pow, Real.sq_sqrt hq.le]; exact one_div_mul_cancel hq.ne'

/-- eq. (25): (d_x y)^2 = v^2/(c^2 - v^2) -/
theorem sx_sq (hc : 0 < c) (hvc : v < c) (hv : 0 ≤ v) : sx c v ^ 2 = v ^ 2 / (c ^ 2 - v ^ 2) := by
  have hg := gam_sq hc hvc hv
  have hne : c ^ 2 - v ^ 2 ≠ 0 := by nlinarith
  unfold sx
  field_simp
  have : gam c v ^ 2 * (c ^ 2 - v ^ 2) = c ^ 2 := by
    have : c ^ 2 - v ^ 2 = c ^ 2 * (1 - v ^ 2 / c ^ 2) := by field_simp
    rw [this, ← mul_assoc, mul_comm (gam c v ^ 2), mul_assoc, hg, mul_one]
  nlinarith [this]

/-- eq. (26)-(27): gamma = sqrt (1 + (d_x y)^2) -/
theorem gamma_eq (hc : 0 < c) (hvc : v < c) (hv : 0 ≤ v) : gam c v = Real.sqrt (1 + sx c v ^ 2) := by
  have hg := gam_sq hc hvc hv
  have h1 : 1 + sx c v ^ 2 = gam c v ^ 2 := by
    unfold sx
    have : (v / c) ^ 2 = v ^ 2 / c ^ 2 := by rw [div_pow]
    rw [mul_pow, this]
    linear_combination (-1 : ℝ) * hg
  rw [h1, Real.sqrt_sq]
  unfold gam; have := one_sub_pos hc hvc hv; positivity

/-- the slope is sinh of the rapidity and gamma is cosh: (27) is cosh^2 - sinh^2 = 1 -/
theorem slope_is_sinh (hc : 0 < c) (hvc : v < c) (hv : 0 ≤ v) :
    Real.sinh (Real.arsinh (sx c v)) = sx c v ∧ Real.cosh (Real.arsinh (sx c v)) = gam c v := by
  refine ⟨Real.sinh_arsinh _, ?_⟩
  rw [Real.cosh_arsinh, gamma_eq hc hvc hv]

/-- lighthouse thought experiment (section 1.1): A's trip time over gamma equals B's contracted-distance time -/
theorem lighthouse (hc : 0 < c) (hvc : v < c) (hv : 0 < v) (d : ℝ) :
    (d / gam c v) / v = (d / v) / gam c v := by
  have : gam c v ≠ 0 := by
    unfold gam; have := one_sub_pos hc hvc hv.le; positivity
  field_simp
end lorentz

/-! ## Section 2.4.4: curvature of the slice, eq. (109) -/

/-- (3)R = 4 beta beta'/R + 2 beta^2/R^2 equals (2/R^2) d(R beta^2)/dR -/
theorem scalar_curv_form (R b b' : ℝ) (hR : R ≠ 0) :
    4 * b * b' / R + 2 * b ^ 2 / R ^ 2 = (2 / R ^ 2) * (b ^ 2 + 2 * R * b * b') := by
  field_simp; ring

theorem deriv_R_beta_sq (β : ℝ → ℝ) (R : ℝ) (hβ : DifferentiableAt ℝ β R) :
    HasDerivAt (fun r => r * β r ^ 2) (β R ^ 2 + 2 * R * β R * deriv β R) R := by
  have h := (hasDerivAt_id R).mul ((hβ.hasDerivAt).pow 2)
  refine h.congr_deriv ?_
  norm_num
  ring

/-- eq. (110): if (R beta^2)' = 8 pi G rho R^2/c^2 and m' = 4 pi rho R^2 then R beta^2 = 2 G m / c^2 + const -/
theorem beta_sq_mass (G c : ℝ) (ρ m β : ℝ → ℝ)
    (hm : ∀ R, HasDerivAt m (4 * π * ρ R * R ^ 2) R)
    (hb : ∀ R, HasDerivAt (fun r => r * β r ^ 2) (8 * π * G * ρ R * R ^ 2 / c ^ 2) R)
    (hc : c ≠ 0) (h0 : (0 : ℝ) * β 0 ^ 2 = 2 * G * m 0 / c ^ 2) (R : ℝ) :
    R * β R ^ 2 = 2 * G * m R / c ^ 2 := by
  have hF : ∀ R, HasDerivAt (fun r => r * β r ^ 2 - 2 * G * m r / c ^ 2) 0 R := by
    intro R
    have := (hb R).sub (((hm R).const_mul (2 * G)).div_const (c ^ 2))
    refine this.congr_deriv ?_
    ring
  have hconst := is_const_of_deriv_eq_zero (fun R => (hF R).differentiableAt) (fun R => (hF R).deriv) R 0
  linarith [hconst, h0]

/-! ## Section 2.4.4 cases -/

/-- exterior: y = 2 sqrt (rs (R - rs)) has slope sqrt (rs/(R-rs)) = beta gamma for beta^2 = rs/R -/
theorem exterior_slope (rs R : ℝ) (hrs : 0 < rs) (hR : rs < R) :
    HasDerivAt (fun r => 2 * Real.sqrt (rs * (r - rs))) (Real.sqrt (rs / (R - rs))) R := by
  have hpos : 0 < rs * (R - rs) := mul_pos hrs (by linarith)
  have h1 : HasDerivAt (fun r => rs * (r - rs)) rs R := by
    simpa using ((hasDerivAt_id R).sub_const rs).const_mul rs
  have h2 := (h1.sqrt hpos.ne').const_mul 2
  refine h2.congr_deriv ?_
  have hs : Real.sqrt (rs * (R - rs)) = Real.sqrt rs * Real.sqrt (R - rs) := Real.sqrt_mul hrs.le _
  have hrs' : 0 < Real.sqrt rs := Real.sqrt_pos.2 hrs
  have hR' : 0 < Real.sqrt (R - rs) := Real.sqrt_pos.2 (by linarith)
  rw [Real.sqrt_div hrs.le, hs]
  have : rs = Real.sqrt rs ^ 2 := (Real.sq_sqrt hrs.le).symm
  field_simp
  nlinarith [this]

theorem exterior_beta_gamma (rs R : ℝ) (hrs : 0 < rs) (hR : rs < R) :
    Real.sqrt (rs / R) * (1 / Real.sqrt (1 - rs / R)) = Real.sqrt (rs / (R - rs)) := by
  have hR0 : 0 < R := by linarith
  have hq : 0 < 1 - rs / R := by
    have : rs / R < 1 := by rw [div_lt_one hR0]; exact hR
    linarith
  rw [← div_eq_mul_one_div, ← Real.sqrt_div (by positivity)]
  congr 1
  field_simp

/-- uniform density: y = sqrt (L^2 - R^2) has |slope| = R/sqrt(L^2-R^2) = beta gamma for beta = R/L -/
theorem cap_slope (L R : ℝ) (h : R ^ 2 < L ^ 2) :
    HasDerivAt (fun r => Real.sqrt (L ^ 2 - r ^ 2)) (-R / Real.sqrt (L ^ 2 - R ^ 2)) R := by
  have hpos : 0 < L ^ 2 - R ^ 2 := by linarith
  have h1 : HasDerivAt (fun r => L ^ 2 - r ^ 2) (-(2 * R)) R := by
    simpa using ((hasDerivAt_pow 2 R).const_sub (L ^ 2))
  have h2 := h1.sqrt hpos.ne'
  refine h2.congr_deriv ?_
  have : 0 < Real.sqrt (L ^ 2 - R ^ 2) := Real.sqrt_pos.2 hpos
  field_simp

theorem cap_beta_gamma (L R : ℝ) (hL : 0 < L) (hR : 0 ≤ R) (h : R < L) :
    (R / L) * (1 / Real.sqrt (1 - (R / L) ^ 2)) = R / Real.sqrt (L ^ 2 - R ^ 2) := by
  have h1 : 0 < 1 - (R / L) ^ 2 := by
    have : (R / L) ^ 2 < 1 := by rw [div_pow, div_lt_one (by positivity)]; nlinarith
    linarith
  have h2 : Real.sqrt (L ^ 2 - R ^ 2) = L * Real.sqrt (1 - (R / L) ^ 2) := by
    have : L ^ 2 - R ^ 2 = L ^ 2 * (1 - (R / L) ^ 2) := by field_simp
    rw [this, Real.sqrt_mul (sq_nonneg L), Real.sqrt_sq hL.le]
  have h3 : 0 < Real.sqrt (1 - (R / L) ^ 2) := Real.sqrt_pos.2 h1
  rw [h2]; field_simp

/-- critical density: L^2 = 3 c^2/(8 pi G rho) = c^2/H^2, and the Schwarzschild radius of the ball of radius c/H is c/H -/
theorem critical_L (G c H : ℝ) (hG : G ≠ 0) (hH : H ≠ 0) :
    3 * c ^ 2 / (8 * π * G * (3 * H ^ 2 / (8 * π * G))) = (c / H) ^ 2 := by
  have hpi : π ≠ 0 := Real.pi_ne_zero
  field_simp

theorem critical_rs (G c H : ℝ) (hG : G ≠ 0) (hc : c ≠ 0) (hH : H ≠ 0) :
    2 * G * ((4 * π / 3) * (3 * H ^ 2 / (8 * π * G)) * (c / H) ^ 3) / c ^ 2 = c / H := by
  have hpi : π ≠ 0 := Real.pi_ne_zero
  field_simp; ring

/-! ## Section 2.5, Proposition 1: the ODE step -/

theorem hasDeriv_rs_over (rs R : ℝ) (hR : R ≠ 0) : HasDerivAt (fun r => rs / r) (-rs / R ^ 2) R := by
  simpa [div_eq_mul_inv, neg_div, mul_comm] using (hasDerivAt_inv hR).const_mul rs

/-- for beta^2 = rs/R:  beta beta' + beta^2/R = rs/(2R^2) -/
theorem prop1_rhs (rs R : ℝ) (hR : R ≠ 0) :
    (1 / 2) * (-rs / R ^ 2) + (rs / R) / R = rs / (2 * R ^ 2) := by
  field_simp; ring

/-- eq. (120)-(121): gamma^2 (beta beta' + beta^2/R) = rs/(2R(R - rs)) = (1/2) d/dR ln(1 - rs/R) -/
theorem prop1_ode (rs R : ℝ) (hR : R ≠ 0) (hne : R - rs ≠ 0) :
    (R / (R - rs)) * (rs / (2 * R ^ 2)) = rs / (2 * R * (R - rs)) := by
  field_simp

/-- N = sqrt(1 - rs/R) has N'/N = rs/(2R(R - rs)) -/
theorem lapse_log_deriv (rs R : ℝ) (hrs : 0 < rs) (hR : rs < R) :
    HasDerivAt (fun r => Real.sqrt (1 - rs / r)) (rs / (2 * R * (R - rs)) * Real.sqrt (1 - rs / R)) R := by
  have hR0 : 0 < R := by linarith
  have hq : 0 < 1 - rs / R := by
    have : rs / R < 1 := by rw [div_lt_one hR0]; exact hR
    linarith
  have h1 : HasDerivAt (fun r => 1 - rs / r) (rs / R ^ 2) R := by
    have := (hasDeriv_rs_over rs R hR0.ne').const_sub 1
    simpa [neg_div] using this
  have h2 := h1.sqrt hq.ne'
  refine h2.congr_deriv ?_
  have hsq : Real.sqrt (1 - rs / R) ^ 2 = 1 - rs / R := Real.sq_sqrt hq.le
  have hs : 0 < Real.sqrt (1 - rs / R) := Real.sqrt_pos.2 hq
  have hne : R - rs ≠ 0 := by linarith
  generalize Real.sqrt (1 - rs / R) = s at hsq hs ⊢
  rw [div_eq_iff (by positivity)]
  have e : rs / (2 * R * (R - rs)) * s * (2 * s) = rs / (R * (R - rs)) * s ^ 2 := by field_simp
  rw [e, hsq]
  field_simp

/-- g_00 h_RR = -1 for N = 1/gamma -/
theorem g00_hRR (b : ℝ) (hb : b ^ 2 < 1) : -(1 - b ^ 2) * (1 / (1 - b ^ 2)) = -1 := by
  have : 1 - b ^ 2 ≠ 0 := by linarith
  field_simp

/-! ## Section 2.5.1: Buchdahl arithmetic -/

/-- 3 s* > 1 with s*^2 = 1 - rs/R*  is  R* > (9/8) rs -/
theorem buchdahl_iff (rs R : ℝ) (hrs : 0 < rs) (hR : rs < R) :
    1 < 3 * Real.sqrt (1 - rs / R) ↔ 9 / 8 * rs < R := by
  have hR0 : 0 < R := by linarith
  have hq : 0 < 1 - rs / R := by
    have : rs / R < 1 := by rw [div_lt_one hR0]; exact hR
    linarith
  have hsq : Real.sqrt (1 - rs / R) ^ 2 = 1 - rs / R := Real.sq_sqrt hq.le
  have hs : 0 < Real.sqrt (1 - rs / R) := Real.sqrt_pos.2 hq
  constructor
  · intro h
    have h1 : 1 / 9 < Real.sqrt (1 - rs / R) ^ 2 := by nlinarith
    rw [hsq] at h1
    have : rs / R < 8 / 9 := by linarith
    rw [div_lt_iff₀ hR0] at this; linarith
  · intro h
    have : rs / R < 8 / 9 := by rw [div_lt_iff₀ hR0]; linarith
    have h1 : 1 / 9 < Real.sqrt (1 - rs / R) ^ 2 := by rw [hsq]; linarith
    nlinarith

/-! ## Section 2.8: Painleve-Gullstrand form and the Hubble flow -/

/-- (134): h_RR = g_RR - g_TR^2/g_TT = 1 + beta^2 gamma^2 = gamma^2 for the metric (133) -/
theorem pg_hRR (c b : ℝ) (hc : c ≠ 0) (hb : b ^ 2 < 1) :
    1 - (b * c) ^ 2 / (-c ^ 2 + (b * c) ^ 2) = 1 / (1 - b ^ 2) := by
  have h1 : 1 - b ^ 2 ≠ 0 := by linarith
  have h2 : -c ^ 2 + (b * c) ^ 2 ≠ 0 := by
    have : -c ^ 2 + (b * c) ^ 2 = -(c ^ 2 * (1 - b ^ 2)) := by ring
    rw [this]; exact neg_ne_zero.2 (mul_ne_zero (pow_ne_zero 2 hc) h1)
  have hden : -c ^ 2 + (b * c) ^ 2 = -(c ^ 2 * (1 - b ^ 2)) := by ring
  rw [hden, div_neg, sub_neg_eq_add, mul_pow]
  have hc2 : c ^ 2 ≠ 0 := pow_ne_zero 2 hc
  field_simp
  ring

/-- (135): the change of time dT = dt + k dR, k = b/(c(1 - b^2)), turns (133) into Schwarzschild form:
coefficients of dt^2, dt dR, dR^2 -/
theorem pg_to_schwarzschild (c b : ℝ) (hc : c ≠ 0) (hb : b ^ 2 < 1) :
    let k := b / (c * (1 - b ^ 2))
    -- ds^2 = -c^2 (dt + k dR)^2 + (dR + b c (dt + k dR))^2, expanded
    (-c ^ 2 + (b * c) ^ 2 = -(1 - b ^ 2) * c ^ 2) ∧
    (-2 * c ^ 2 * k + 2 * (b * c) * (1 + b * c * k) = 0) ∧
    (-c ^ 2 * k ^ 2 + (1 + b * c * k) ^ 2 = 1 / (1 - b ^ 2)) := by
  intro k
  have h1 : 1 - b ^ 2 ≠ 0 := by linarith
  refine ⟨by ring, ?_, ?_⟩
  · simp only [k]; field_simp; ring
  · simp only [k]; field_simp; ring

/-- (136): beta_H^2 = H^2 R^2/c^2 equals 2 G m/(c^2 R) with m = (4 pi/3) rho R^3 and H^2 = 8 pi G rho/3 -/
theorem hubble_beta (G c R ρ H : ℝ) (hc : c ≠ 0) (hR : R ≠ 0) (hF : H ^ 2 = 8 * π * G * ρ / 3) :
    (H * R / c) ^ 2 = 2 * G * ((4 * π / 3) * ρ * R ^ 3) / (c ^ 2 * R) := by
  rw [div_pow, mul_pow, hF]; field_simp; ring

end MuellerChecks

#print axioms MuellerChecks.gamma_eq
#print axioms MuellerChecks.slope_is_sinh
#print axioms MuellerChecks.beta_sq_mass
#print axioms MuellerChecks.exterior_slope
#print axioms MuellerChecks.cap_slope
#print axioms MuellerChecks.lapse_log_deriv
#print axioms MuellerChecks.buchdahl_iff
#print axioms MuellerChecks.pg_to_schwarzschild
#print axioms MuellerChecks.hubble_beta

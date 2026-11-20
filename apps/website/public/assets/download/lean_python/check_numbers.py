#!/usr/bin/env python3
"""
check_numbers.py -- recompute the numbers quoted in A. Mueller, "The Lorentz Factor as the Slope of an Additional Spatial
Coordinate" (Zenodo v1, 16 Nov 2026) from public constants.  Run first (R24).  CODATA/IAU constants typed here, SI units.
Each block prints SHOWN or FAIL; blocks marked CONTROL can fail by design if the paper's number were wrong.
NOT recomputed: the Eilers et al. rotation-curve chi^2 values (need the data), the Cautun et al. baryonic profile.
"""
import math, sys
FAIL = []
def check(name, ok, detail=""):
    print("  [%s] %s%s" % ("SHOWN" if ok else "FAIL ", name, ("  -- " + detail) if detail else ""))
    if not ok: FAIL.append(name)

G = 6.67430e-11; c = 299792458.0; Msun = 1.98847e30
pc = 3.0856775814913673e16; kpc = 1e3 * pc; Mpc = 1e6 * pc
H0 = 70e3 / Mpc                      # 70 km/s/Mpc, the paper's value
yr = 365.25 * 86400

print("Sec 3.1: cosmological correction vs Newtonian acceleration, m = 6e10 Msun")
m = 6e10 * Msun
for R_kpc, quoted in ((8, 1e-5), (25, 3e-4)):
    R = R_kpc * kpc
    ratio = H0**2 * R / (G * m / R**2)
    check("H0^2 R / gN at %d kpc ~ %.0e" % (R_kpc, quoted), 0.5 * quoted < ratio < 2 * quoted, "%.2e" % ratio)
for R_kpc, lo, hi in ((8, 0.6, 0.6), (25, 1.8, 1.8)):
    v = H0 * R_kpc * kpc / 1e3
    check("H0 R at %d kpc ~ %.1f km/s" % (R_kpc, lo), abs(v - lo) < 0.1, "%.2f km/s" % v)

print("Sec 3.2: Sciama identity (137)")
rho_c = 3 * H0**2 / (8 * math.pi * G); RH = c / H0
mH = 4 * math.pi / 3 * rho_c * RH**3
check("G m_H(R_H)/(c^2 R_H) = 1/2 at the Hubble radius", abs(G * mH / (c**2 * RH) - 0.5) < 1e-12, "%.12f" % (G * mH / (c**2 * RH)))

print("Sec 3.3-3.5: acceleration scale")
cH0 = c * H0
check("cH0 = 6.8e-10 m/s^2 (Table A3 caption)", abs(cH0 - 6.8e-10) < 0.05e-10, "%.3e" % cH0)
g0 = cH0 / (2 * math.pi)
check("g0 = cH0/(2 pi) ~ 1.1e-10 m/s^2", abs(g0 - 1.1e-10) < 0.05e-10, "%.3e" % g0)
for g0v, quoted in ((2.4e-10, 0.35), (1.5e-10, 0.22), (1.0e-10, 0.15)):
    check("Table A3: g0/(cH0) = %.2f" % quoted, abs(g0v / cH0 - quoted) < 0.01, "%.3f" % (g0v / cH0))
Mb = 4.44e10 * Msun; R0 = 8.12 * kpc
v = math.sqrt(2 * G * Mb / R0) / 1e3
check("A33: v_orbit = sqrt(2GM/R) = 216.9 km/s for Mb(<8.12 kpc) = 4.44e10 Msun", abs(v - 216.9) < 0.3, "%.1f km/s" % v)
vflat = (G * Mb * g0) ** 0.25 / 1e3
print("  (info) deep-MOND flat speed v^4 = G M g0 with M = 4.44e10 Msun and g0 = cH0/2pi: %.1f km/s;"
      " the observed outer speed quoted in the paper is 198-227 km/s" % vflat)

print("Appendix A: 'kinematic velocity' c sqrt(G M/R^3) (1 s) and the CMB dipole")
def kin(M, R, tau=1.0):
    return c * math.sqrt(G * M / R**3) * tau / 1e3
earth = kin(3.986004418e14 / G, 6.371e6)
print("  Earth: %.1f km/s (paper A31 quotes 371.6 km/s)" % earth)
check("Earth value within 0.5 percent of the paper's 371.6", abs(earth - 371.6) / 371.6 < 0.005)
dip = 369.82  # Planck 2018 CMB dipole, km/s (+-0.11)
check("agreement with the CMB dipole 369.8 km/s at the 1% level", abs(earth - dip) / dip < 0.01, "%.2f%%" % (100 * abs(earth - dip) / dip))
# CONTROL: the match is a fit of the one free reference time tau
tau_needed = dip / earth
print("  CONTROL: the reference time needed for an exact match is %.4f s, i.e. the match fixes tau to %.2f%%" % (tau_needed, 100 * abs(1 - tau_needed)))
for tau, label in ((60.0, "1 minute"), (1e-3, "1 millisecond")):
    check("CONTROL: with tau = %s the 'agreement' disappears (value %.3g km/s)" % (label, kin(3.986004418e14 / G, 6.371e6, tau)),
          abs(kin(3.986004418e14 / G, 6.371e6, tau) - dip) / dip > 0.5)
for name, rho, quoted in (("Sun", 1408.0, 190), ("Saturn", 687.0, 130), ("Moon", 3344.0, 290)):
    vk = c * math.sqrt(4 * math.pi * G * rho / 3) / 1e3
    check("%s: c sqrt(4 pi G rho/3)(1 s) ~ %d km/s" % (name, quoted), abs(vk - quoted) / quoted < 0.05, "%.0f km/s" % vk)
check("A32: v_cosmo/v_kin = sqrt 2 (525.5/371.6)", abs(525.5 / 371.6 - math.sqrt(2)) < 0.003, "%.4f vs %.4f" % (525.5 / 371.6, math.sqrt(2)))

print("Appendix B / sec 2.6: standard Schwarzschild results (CONTROL: Mercury)")
M = 1.98847e30 * G / G
a = 5.7909e10; e = 0.20563; P = 87.969
dphi = 6 * math.pi * G * Msun / (a * (1 - e**2) * c**2)
asec = dphi * (36525 / P) * 180 / math.pi * 3600
check("perihelion advance 6 pi GM/(a(1-e^2)c^2) = 42.98 arcsec/century (observed 42.98 +- 0.04)", abs(asec - 42.98) < 0.1, "%.2f" % asec)

print("Sec 2.3.4: Cassini bound on G44")
for gm1, label in ((2.3e-5, "1 sigma upper"),):
    omega = 1 / gm1 - 2
    check("omega_BD >~ 4e4 for |gamma_PPN - 1| ~ 2.3e-5 (%s)" % label, 3e4 < omega < 5e4, "%.0f" % omega)

print()
if FAIL: print("FAIL: " + "; ".join(FAIL)); sys.exit(1)
print("ALL PASS")

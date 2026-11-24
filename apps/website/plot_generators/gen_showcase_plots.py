"""Dark-theme showcase figures for cdrm/on_the_gravitational_nature_of_time.cdrm.

Matches the look of the Lorentz Slope Mac app. Needs only numpy + matplotlib.
Run from anywhere:  python3 plot_generators/gen_showcase_plots.py
Output: public/assets/diagrams/showcase/*.png

Every curve here is a closed-form relation from the paper, or a direct numerical
integration of the orbit equation; no data files are required.
"""
from pathlib import Path
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib import cm, colors
from matplotlib.patches import Polygon

OUT = Path(__file__).resolve().parent.parent / "public" / "assets" / "diagrams" / "showcase"
OUT.mkdir(parents=True, exist_ok=True)

BG = "#16161a"; PANEL = "#1e1e24"; FG = "#e8e8ee"; MUTED = "#8b8b98"; GRID = "#2c2c36"
BLUE = "#6cb6ff"; GREEN = "#7ee0b0"; PINK = "#ff6b81"; GOLD = "#f5b85a"; PURPLE = "#a78bfa"

plt.rcParams.update({
    "figure.facecolor": BG, "savefig.facecolor": BG, "axes.facecolor": PANEL,
    "axes.edgecolor": GRID, "axes.labelcolor": FG, "text.color": FG,
    "xtick.color": MUTED, "ytick.color": MUTED, "axes.grid": True, "grid.color": GRID,
    "grid.linewidth": 0.6, "axes.spines.top": False, "axes.spines.right": False,
    "font.size": 12, "savefig.dpi": 200, "figure.dpi": 100, "mathtext.fontset": "cm",
})


def autocrop(path, pad=40):
    """Trim the flat background border (3D / equal-aspect axes leave a lot of it)."""
    import matplotlib.image as mpimg
    img = mpimg.imread(path)
    bg = np.array(colors.to_rgb(BG))
    mask = np.abs(img[..., :3] - bg).max(axis=2) > 0.02
    ys, xs = np.where(mask)
    y0, y1 = max(ys.min() - pad, 0), min(ys.max() + pad, img.shape[0])
    x0, x1 = max(xs.min() - pad, 0), min(xs.max() + pad, img.shape[1])
    plt.imsave(path, img[y0:y1, x0:x1])


def save(fig, name, tight=True):
    if tight:
        fig.savefig(OUT / name, bbox_inches="tight", pad_inches=0.25)
    else:
        fig.savefig(OUT / name)
        autocrop(OUT / name)
    plt.close(fig)
    print("wrote", name)


# ---------------------------------------------------------------- 1. hero funnel
def hero_funnel():
    rs = 1.0
    R = np.linspace(rs, 9, 160)
    phi = np.linspace(0, 2 * np.pi, 200)
    RR, PP = np.meshgrid(R, phi)
    y = 2 * np.sqrt(rs * (RR - rs))
    X, Y, Z = RR * np.cos(PP), RR * np.sin(PP), y
    N = np.sqrt(1 - rs / RR)                      # lapse = cos(theta) = 1/gamma
    cmap_n = colors.LinearSegmentedColormap.from_list('lapse', ['#4fd1ff', '#6c8cff', '#8b6cff'])
    face = cmap_n(N)
    fig = plt.figure(figsize=(9, 5.4))
    ax = fig.add_subplot(111, projection="3d")
    ax.set_facecolor(BG)
    ax.plot_surface(X, Y, Z, facecolors=face, rstride=2, cstride=3, linewidth=0.25,
                    edgecolor=(1, 1, 1, 0.18), antialiased=True, shade=False, alpha=0.95)
    rng = np.random.default_rng(7)
    rt = rng.uniform(1.6, 8.5, 38); pt = rng.uniform(0, 2 * np.pi, 38)
    ax.scatter(rt * np.cos(pt), rt * np.sin(pt), 2 * np.sqrt(rs * (rt - rs)) + 0.12,
               s=30, color="white", depthshade=False)
    th = np.linspace(0, 2 * np.pi, 200)
    ax.plot(np.cos(th), np.sin(th), 0 * th, color=PINK, lw=2.5)
    ax.set_axis_off()
    ax.view_init(elev=22, azim=-58)
    ax.set_box_aspect((1, 1, 0.5), zoom=1.25)
    fig.subplots_adjust(0, 0, 1, 1)
    save(fig, "hero_funnel.png", tight=False)


# ---------------------------------------------------------------- 2. the fan of triangles
def slope_fan():
    betas = [0.2, 0.5, 0.8, 0.9, 0.97]
    cols = [BLUE, GREEN, GOLD, PURPLE, PINK]
    fig, (a, b) = plt.subplots(1, 2, figsize=(12.2, 5), gridspec_kw={"width_ratios": [1.1, 1]})
    a.grid(False)
    for be, c in zip(betas, cols):
        g = 1 / np.sqrt(1 - be ** 2); h = be * g
        a.add_patch(Polygon([[0, 0], [1, 0], [1, h]], closed=True, fc=c, alpha=0.10, ec=c, lw=1.8))
        a.text(1.04, h, rf"$\beta={be}$", color=c, va="center", fontsize=11)
    a.plot([1, 1], [0, 4.2], color=MUTED, lw=0.8, ls=":")
    a.text(0.5, -0.38, r"base = $v\tau$ (one unit of $dx$)", ha="center", color=MUTED)
    a.text(0.40, 2.2, r"hypotenuse $=\gamma\,dx$", rotation=75, color=FG, ha="center", fontsize=11)
    a.text(1.75, 2.9, "height\n" + r"$=\beta\gamma\,dx=\partial_x y$", color=FG, fontsize=11)
    a.set_xlim(-0.1, 2.3); a.set_ylim(-0.6, 4.6); a.set_aspect("equal"); a.axis("off")
    a.set_title("Faster means steeper: every boost is a right triangle", fontsize=12)
    be = np.linspace(0, 0.985, 500); g = 1 / np.sqrt(1 - be ** 2)
    b.plot(be, g, color=PINK, lw=2.4, label=r"$\gamma=\cosh\eta$  (hypotenuse)")
    b.plot(be, be * g, color=GOLD, lw=2.4, label=r"$\beta\gamma=\sinh\eta=\partial_x y$  (slope)")
    b.plot(be, 1 / g, color=GREEN, lw=2.4, label=r"$1/\gamma=\cos\theta$  (foreshortening)")
    b.set_ylim(0, 5.6); b.set_xlabel(r"$\beta=v/c$")
    b.legend(frameon=False, loc="upper left", fontsize=11)
    b.set_title(r"$\gamma^2 = 1+(\partial_x y)^2$", fontsize=12)
    fig.tight_layout()
    save(fig, "slope_fan.png")


# ---------------------------------------------------------------- 3. tilt along the funnel
def tilt_profile():
    rs = 1.0
    R = np.linspace(1.0, 10, 600); y = 2 * np.sqrt(rs * (R - rs))
    fig, (a, b) = plt.subplots(1, 2, figsize=(12.2, 5.2), gridspec_kw={"width_ratios": [1.15, 1]})
    a.plot(R, y, color=BLUE, lw=2.6); a.plot(R, -y, color=BLUE, lw=2.6, alpha=0.35, ls="--")
    cmap = cm.plasma
    for r0 in [1.04, 1.25, 1.6, 2.2, 3.0, 4.2, 5.8, 7.8]:
        be = np.sqrt(rs / r0); th = np.arcsin(be)
        y0 = 2 * np.sqrt(rs * (r0 - rs)); L = 0.55
        dx, dy = L * np.cos(th) * 0 + L * np.sqrt(1 - be ** 2) * 0 + 0, 0
        slope = be / np.sqrt(1 - be ** 2)               # dy/dR = beta*gamma
        v = np.array([1, slope]); v /= np.linalg.norm(v)
        a.plot([r0 - L * v[0], r0 + L * v[0]], [y0 - L * v[1], y0 + L * v[1]],
               color=cmap(be), lw=4, solid_capstyle="round")
        a.plot([r0], [y0], "o", color="white", ms=5)
    a.axvline(1.0, color=PINK, lw=1, ls=":")
    a.text(1.6, -5.4, r"horizon, $R=r_s$:" "\nslope is vertical", color=PINK, fontsize=10.5)
    a.text(5.2, 0.4, "far away the surface flattens\n" r"($\beta\to0,\ \gamma\to1$)", color=MUTED, fontsize=10.5)
    a.set_xlabel(r"$R/r_s$"); a.set_ylabel(r"$y/r_s$"); a.set_aspect("equal"); a.set_xlim(0.7, 10)
    a.set_title("The tilt of space at each radius", fontsize=12)
    R2 = np.linspace(1.0005, 10, 600); be = np.sqrt(rs / R2); N = np.sqrt(1 - rs / R2)
    b.plot(R2, be, color=GOLD, lw=2.4, label=r"$\beta=\sin\theta=\sqrt{r_s/R}$")
    b.plot(R2, N, color=GREEN, lw=2.4, label=r"$N=\cos\theta=\sqrt{1-r_s/R}$")
    b.plot(R2, be * np.sqrt(1 / N ** 2), color=PINK, lw=2.4, alpha=0.9, label=r"$\partial_R y=\beta\gamma$")
    b.set_ylim(0, 3); b.set_xlabel(r"$R/r_s$"); b.legend(frameon=False, fontsize=11)
    b.set_title("Gravitational time dilation is the cosine of the tilt", fontsize=12)
    fig.tight_layout()
    save(fig, "tilt_profile.png")


# ---------------------------------------------------------------- 4. precession rosette
def orbit_rosette(p=24.0, e=0.5, revs=6):
    """Schwarzschild orbit  u'' + u = 1/p + 3u^2  (units GM/c^2), integrated with RK4 in phi."""
    def f(s):
        u, w = s
        return np.array([w, 1 / p - u + 3 * u * u])
    u0 = (1 + e) / p; s = np.array([u0, 0.0]); h = 0.002
    phis, us = [0.0], [u0]; ph = 0.0
    while ph < revs * 2 * np.pi * 1.25:
        k1 = f(s); k2 = f(s + h / 2 * k1); k3 = f(s + h / 2 * k2); k4 = f(s + h * k3)
        s = s + h / 6 * (k1 + 2 * k2 + 2 * k3 + k4); ph += h
        phis.append(ph); us.append(s[0])
    ph, u = np.array(phis), np.array(us)
    r = 1 / u
    # periapsis passes = local maxima of u
    pk = [i for i in range(1, len(u) - 1) if u[i] > u[i - 1] and u[i] >= u[i + 1] and u[i] > 0.5 * u0]
    adv = np.degrees(np.diff(ph[pk]) - 2 * np.pi)
    print(f"orbit p={p}, e={e}: measured advance/orbit = {adv.mean():.2f} deg; leading order 6*pi/p = {np.degrees(6*np.pi/p):.2f} deg")
    n = int(revs * 2 * np.pi / h)
    fig, ax = plt.subplots(figsize=(7, 7)); ax.grid(False)
    ax.plot(r[:n] * np.cos(ph[:n]), r[:n] * np.sin(ph[:n]), color=PURPLE, lw=1.4, alpha=0.95)
    for i in pk[: revs + 1]:
        ax.plot([0, r[i] * 1.18 * np.cos(ph[i])], [0, r[i] * 1.18 * np.sin(ph[i])], color=PINK, lw=0.9, ls="--", alpha=0.7)
    ax.add_patch(plt.Circle((0, 0), 2, color=PINK, alpha=0.9))
    ax.add_patch(plt.Circle((0, 0), 6, fill=False, ec=GREEN, ls="--", lw=1)); ax.text(0, -8.4, r"ISCO $6GM/c^2$", color=GREEN, ha="center", fontsize=10)
    ax.set_aspect("equal"); ax.axis("off")
    m = 1.03 * (p / (1 - e))
    ax.set_xlim(-m, m); ax.set_ylim(-m, m)
    fig.subplots_adjust(0, 0, 1, 1)
    save(fig, "orbit_rosette.png", tight=False)


# ---------------------------------------------------------------- 5. Mercury check (analytic)
def mercury_number():
    GM = 1.32712440018e20; c = 299792458.0
    a = 5.7909050e10; e = 0.205630; T = 87.9691  # days
    dphi = 6 * np.pi * GM / (a * (1 - e ** 2) * c ** 2)
    orbits = 36525 / T
    arcsec = np.degrees(dphi * orbits) * 3600
    print(f"Mercury: {arcsec:.2f} arcsec / century")
    return arcsec


# ---------------------------------------------------------------- 6. where the flows meet
def two_flows_dark():
    G = 6.67430e-11; c = 299792458.0; Msun = 1.98847e30; kpc = 3.0856776e19; Mpc = 3.0856776e22
    H = 70e3 / Mpc; M = 6e10 * Msun
    R = np.logspace(np.log10(0.5 * kpc), np.log10(2 * c / H), 600)
    bN = np.sqrt(2 * G * M / (c ** 2 * R)); bH = H * R / c
    fig, a = plt.subplots(figsize=(9.4, 5))
    a.loglog(R / kpc, bN, color=PINK, lw=2.6, label=r"falling in: $\beta=\sqrt{2GM/c^2R}$")
    a.loglog(R / kpc, bH, color=BLUE, lw=2.6, label=r"Hubble flow: $\beta=HR/c$")
    Rx = (2 * G * M / H ** 2) ** (1 / 3)
    a.plot([Rx / kpc], [H * Rx / c], "o", color="white", ms=7)
    a.annotate("the two flows are equal here", (Rx / kpc, H * Rx / c), (3e3, 3e-3), color=FG, fontsize=11,
               arrowprops=dict(arrowstyle="->", color=MUTED))
    a.axhline(1, color=MUTED, ls=":"); a.set_ylim(1e-9, 3)
    a.set_xlabel("distance from the mass [kpc]"); a.set_ylabel(r"flow speed $\beta=v/c$")
    a.legend(frameon=False, loc="lower right", fontsize=11)
    a.set_title(r"One relation, $\beta^2 = 2Gm/(c^2R)$, read inward and outward ($M=6\times10^{10}M_\odot$)", fontsize=12)
    fig.tight_layout()
    save(fig, "two_flows_dark.png")


if __name__ == "__main__":
    hero_funnel(); slope_fan(); tilt_profile(); orbit_rosette(); two_flows_dark(); mercury_number()

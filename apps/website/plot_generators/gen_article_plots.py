"""Generates every figure used by the note (PNG, white background). Run: python3 make_figures.py
Needs: numpy, scipy, matplotlib; data/results_per_radius.csv (written by run_tests.py of the Cautun test)."""
import numpy as np, csv, matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon, Arc
from scipy.optimize import curve_fit
from pathlib import Path
OUT=Path(__file__).parent.parent / "public" / "assets" / "diagrams" / "ai_generated"
plt.rcParams.update({'font.size':11,'axes.spines.top':False,'axes.spines.right':False,'figure.dpi':100,'savefig.dpi':200,
                     'savefig.facecolor':'white','axes.facecolor':'white'})
BLUE='#1f4e79'; ORG='#d9822b'; RED='#b2262e'; GRY='#666666'

# ------------------------------------------------ 1. the Lorentz factor is a slope
fig,ax=plt.subplots(1,2,figsize=(11,4.4),gridspec_kw={'width_ratios':[1.05,1]})
beta=0.8; g=1/np.sqrt(1-beta**2); h=np.sqrt(g**2-1)
a=ax[0]
a.add_patch(Polygon([[0,0],[1,0],[1,h]],closed=True,fc='#e8eef5',ec=BLUE,lw=2))
a.text(0.5,-0.14,r'$dx = v\tau$',ha='center',color=BLUE,fontsize=13)
a.text(1.04,h/2,r'$dy = v\tau\sqrt{\gamma^2-1}$',va='center',color=ORG,fontsize=13)
a.text(0.40,h/2+0.1,r'$dx^{(U)} = \gamma\, v\tau$',ha='right',color=RED,fontsize=13,rotation=np.degrees(np.arctan(h)))
a.add_patch(Arc((0,0),0.6,0.6,theta1=0,theta2=np.degrees(np.arctan(h)),color=GRY))
a.text(0.36,0.10,r'$\theta$',color=GRY,fontsize=13)
a.text(0.02,1.75,r'$\sin\theta=\beta,\quad\cos\theta=1/\gamma$'+'\n'+r'slope $=\partial_x y=\gamma\beta$',fontsize=12,va='top')
a.set_xlim(-0.1,1.9); a.set_ylim(-0.3,1.9); a.set_aspect('equal'); a.axis('off')
a.set_title(r'The extra separation is a leg of a right triangle ($\beta=0.8$ shown)',fontsize=11)
b=ax[1]; bb=np.linspace(0,0.97,400); gg=1/np.sqrt(1-bb**2)
b.plot(bb,gg,color=RED,lw=2,label=r'$\gamma=\cosh\eta$ (hypotenuse)')
b.plot(bb,gg*bb,color=ORG,lw=2,label=r'$\gamma\beta=\sinh\eta$ (slope)')
b.plot(bb,np.ones_like(bb),color=BLUE,lw=2,label=r'$1$ (base)')
b.set_ylim(0,4.2); b.set_xlabel(r'$\beta=v/c$'); b.legend(frameon=False,loc='upper left')
b.set_title(r'$\gamma^2 = 1+(\gamma\beta)^2$: the hyperbolic identity',fontsize=11)
plt.tight_layout(); plt.savefig(OUT / 'slope_triangle.png'); plt.close()

# ------------------------------------------------ 2. Flamm paraboloid, tilt angle, lapse
fig,ax=plt.subplots(1,2,figsize=(11,4.6))
a=ax[0]; Rr=np.linspace(1,7,400); y=2*np.sqrt(Rr-1)
a.plot(Rr,y,color=BLUE,lw=2.5); a.plot(Rr,-y,color=BLUE,lw=2.5,ls='--',alpha=0.45)
R0=2.5; y0=2*np.sqrt(R0-1); slope=1/np.sqrt(R0-1)       # dy/dR = beta*gamma = sqrt(1/(R-1))
xs=np.array([R0-0.9,R0+0.9]); a.plot(xs,y0+slope*(xs-R0),color=ORG,lw=1.8)
a.plot([R0],[y0],'o',color=RED); a.annotate(r'tilt $\theta$: $\sin\theta=\beta=\sqrt{r_s/R}$'+'\n'+r'$\cos\theta=N=\sqrt{1-r_s/R}$',(R0,y0),(3.9,0.6),
        arrowprops=dict(arrowstyle='->',color=GRY),fontsize=11)
a.set_xlabel(r'$R/r_s$'); a.set_ylabel(r'$y/r_s$'); a.set_xlim(0.8,7); a.set_aspect('equal')
a.set_title(r'Flamm paraboloid $y=2\sqrt{r_s(R-r_s)}$',fontsize=11)
b=ax[1]; R2=np.linspace(1.001,10,500); be=np.sqrt(1/R2); N=np.sqrt(1-1/R2)
b.plot(R2,be,color=ORG,lw=2,label=r'$\beta=\sin\theta=\sqrt{r_s/R}$')
b.plot(R2,N,color=RED,lw=2,label=r'$N=\cos\theta=1/\gamma$')
b.set_xlabel(r'$R/r_s$'); b.legend(frameon=False); b.set_ylim(0,1.05)
b.set_title('Escape-velocity slope and lapse',fontsize=11)
plt.tight_layout(); plt.savefig(OUT / 'flamm_paraboloid.png'); plt.close()

# ------------------------------------------------ 3. two flows
G=6.67430e-11; c=299792458.0; Msun=1.98847e30; kpc=3.0856776e19; Mpc=3.0856776e22
H=70e3/Mpc; M=6e10*Msun
Rm=np.logspace(np.log10(0.5*kpc),np.log10(c/H*2),600)
bN=np.sqrt(2*G*M/(c**2*Rm)); bH=H*Rm/c
fig,a=plt.subplots(figsize=(8,4.8))
a.loglog(Rm/kpc,bN,color=RED,lw=2,label=r'local infall $\beta_N=\sqrt{2GM/c^2R}$  ($M=6\times10^{10}M_\odot$)')
a.loglog(Rm/kpc,bH,color=BLUE,lw=2,label=r'Hubble flow $\beta_H=HR/c$  ($H_0=70$)')
a.axhline(1,color=GRY,ls=':'); a.axvline(c/H/kpc,color=GRY,ls=':')
a.text(c/H/kpc*0.9,1.4,r'$R_H=c/H$: $\beta_H=1$',ha='right',color=GRY)
a.axvspan(5,25,color=ORG,alpha=0.18); a.text(11,3e-9,'Milky Way\ndisc',ha='center',color=ORG)
Rzg=(2*G*M/c**2*c**2/H**2)**(1/3)
a.plot([Rzg/kpc],[np.sqrt(2*G*M/(c**2*Rzg))],'ko'); a.text(Rzg/kpc*1.15,np.sqrt(2*G*M/(c**2*Rzg))*1.3,'flows equal',fontsize=10)
a.set_xlabel('R [kpc]'); a.set_ylabel(r'flow speed $\beta = v/c$'); a.set_ylim(1e-9,3); a.legend(frameon=False,loc='lower right',fontsize=9)
a.set_title('Local infall and cosmic expansion are the same relation',fontsize=11)
plt.tight_layout(); plt.savefig(OUT/ 'two_flows.png'); plt.close()

# ------------------------------------------------ 4. single-body orbits
sysm=[('Mercury\n(Sun)',1.32712440018e20,5.7909e10,47.36e3),('Earth\n(Sun)',1.32712440018e20,1.49598e11,29.78e3),
      ('Jupiter\n(Sun)',1.32712440018e20,7.7857e11,13.07e3),('Moon\n(Earth)',3.986004418e14,3.844e8,1.022e3),('Io\n(Jupiter)',1.26686534e17,4.217e8,17.33e3)]
ratios=[(v/np.sqrt(2*GM/R))**2 for _,GM,R,v in sysm]
fig,a=plt.subplots(figsize=(7.5,4.2))
a.bar([s[0] for s in sysm],ratios,color=BLUE,width=0.55)
a.axhline(0.5,color=ORG,lw=2,ls='--',label=r'$1/2$: circular speed $=v_{\rm orbit}/\sqrt{2}$')
a.axhline(1.0,color=RED,lw=1.5,ls=':',label=r'$1$: orbit speed $=v_{\rm orbit}$')
for i,r in enumerate(ratios): a.text(i,r+0.025,f'{r:.2f}',ha='center')
a.legend(frameon=False,loc='upper center',bbox_to_anchor=(0.5,0.80),ncol=1)
a.set_ylim(0,1.2); a.set_ylabel(r'$(v_{\rm obs}/\sqrt{2GM/R})^2$')
a.set_title('Observed orbits in near single-body systems',fontsize=11)
plt.tight_layout(); plt.savefig(OUT/'single_body_orbits.png'); plt.close()

# ------------------------------------------------ 5. rotation curve with Cautun baryons
rows=list(csv.DictReader(open('data/results_per_radius.csv')))
f=lambda k: np.array([float(r[k]) for r in rows])
R=f('R_kpc'); v=f('v_obs'); ss=f('sig_stat'); sy=f('sig_sys'); vor=f('v_orbit'); vn=f('v_newton_discaware')
Menc=f('M_enc_total_Msun'); Mreq=f('M_required_Msun'); sig=np.hypot(ss,sy)
fig,ax=plt.subplots(1,2,figsize=(11.5,4.6))
a=ax[0]
a.errorbar(R,v,yerr=sig,fmt='o',ms=3.5,color='k',elinewidth=1,label='Eilers et al. (2019)')
a.plot(R,vor,color=RED,lw=2.2,label=r'$v_{\rm orbit}=\sqrt{2GM_b(<R)/R}$ (Cautun baryons)')
a.plot(R,vn,color=BLUE,lw=2,ls='--',label='Newtonian circular speed, baryons only')
a.set_xlabel('Galactocentric radius R [kpc]'); a.set_ylabel('speed [km/s]'); a.set_ylim(90,260); a.legend(frameon=False,fontsize=8.5,loc='lower left')
a.set_title('Predicted vs observed rotation',fontsize=11)
b=ax[1]
b.plot(R,Mreq/1e10,'ko-',ms=3,lw=1,label=r'enclosed mass needed: $v_{\rm obs}^2R/2G$')
b.plot(R,Menc/1e10,color=RED,lw=2.2,label=r'Cautun baryons $M_b(<R)$')
b.set_xlabel('Galactocentric radius R [kpc]'); b.set_ylabel(r'mass [$10^{10}\,M_\odot$]'); b.legend(frameon=False,fontsize=8.5,loc='upper left')
b.set_title('Mass needed vs baryons available',fontsize=11)
plt.tight_layout(); plt.savefig(OUT / 'rotation_curve_cautun.png'); plt.close()

# ------------------------------------------------ 6. acceleration plane and Hypothesis M
conv=1e6/kpc                       # (km/s)^2/kpc -> m/s^2
gobs=v**2/R*conv; gbar=vn**2/R*conv; gerr=2*v*sig/R*conv
mask=ss<10
rar=lambda g,g0: g/(1-np.exp(-np.sqrt(g/g0)))
p,_=curve_fit(lambda g,x: rar(g,x*1e-10),gbar[mask],gobs[mask],p0=[1.5],sigma=gerr[mask]); g0=p[0]*1e-10
cH0=c*H; print(f'fitted g0 = {g0:.3e} m/s^2 = {g0/cH0:.3f} cH0 ; cH0/2pi = {cH0/2/np.pi:.3e}')
gg=np.logspace(-11.3,-9.2,300)
fig,a=plt.subplots(figsize=(7.4,5.2))
a.errorbar(gbar,gobs,yerr=gerr,fmt='o',ms=4,color='k',elinewidth=1,label='Eilers + Cautun baryons')
a.loglog(gg,gg,color=GRY,ls=':',label=r'Newtonian: $g=g_N$')
a.loglog(gg,rar(gg,g0),color=RED,lw=2.2,label=rf'Hypothesis M, fitted $g_0={g0/cH0:.2f}\,cH_0$')
a.loglog(gg,rar(gg,cH0/2/np.pi),color=BLUE,lw=2,ls='--',label=r'$g_0=cH_0/2\pi$ (not fitted)')
a.set_xlabel(r'baryonic acceleration $g_N$ [m/s$^2$]'); a.set_ylabel(r'observed acceleration $v^2/R$ [m/s$^2$]'); a.legend(frameon=False,fontsize=9,loc='upper left')
a.set_title('Where the extra inertia effect switches on',fontsize=11)
plt.tight_layout(); plt.savefig(OUT / 'hypothesis_M_fit.png'); plt.close()
print('done')

# ------------------------------------------------ 7. three geometries from one relation
fig,ax=plt.subplots(1,2,figsize=(11.2,4.6))
a=ax[0]; rs=1.0; Rs=3.0; L=np.sqrt(Rs**3/rs)
Rin=np.linspace(0,Rs,300); yin=L-L*np.sqrt(1-(Rin/L)**2)
Rout=np.linspace(Rs,8,400); yout=yin[-1]+2*np.sqrt(rs*(Rout-rs))-2*np.sqrt(rs*(Rs-rs))
a.plot(Rin,yin,color=RED,lw=2.8,label=r'inside: spherical cap, $\beta^2=R^2/L^2$')
a.plot(Rout,yout,color=BLUE,lw=2.8,label=r'outside: paraboloid, $\beta^2=r_s/R$')
a.plot([Rs],[yin[-1]],'ko'); a.text(Rs+0.15,yin[-1]-0.45,'surface of the star',fontsize=10)
a.set_xlabel(r'$R/r_s$'); a.set_ylabel(r'$y/r_s$'); a.legend(frameon=False,loc='upper left',fontsize=9.5)
a.set_title('A uniform-density star: one relation, two shapes',fontsize=11)
b=ax[1]; Rc=np.linspace(0,1,400); yc=1-np.sqrt(1-Rc**2)
b.plot(Rc,yc,color=ORG,lw=2.8); b.plot([1],[1],'ko')
b.annotate(r'$R=c/H$: slope is vertical, $\beta=1$',(1,1),(0.18,0.78),arrowprops=dict(arrowstyle='->',color=GRY),fontsize=10.5)
b.text(0.03,0.40,r'critical density: $\beta=HR/c$'+'\n(the Hubble flow)',fontsize=10.5)
b.set_xlabel(r'$R\,/\,(c/H)$'); b.set_ylabel(r'$y\,/\,(c/H)$'); b.set_aspect('equal')
b.set_title('The whole universe at critical density',fontsize=11)
plt.tight_layout(); plt.savefig(OUT / 'three_geometries.png'); plt.close()
print('three_geometries done')
